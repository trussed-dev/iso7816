use core::fmt;

/// Error returned when the [Aid::try_new](Aid::try_new) or
/// [Aid::try_new_truncatable](Aid::try_new_truncatable) fail
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FromSliceError {
    Empty,
    TooLong,
    TruncatedLengthLargerThanLength,
    NationalRidTooShort,
    InternationalRidTooShort,
}

impl fmt::Debug for FromSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "AID needs at least a category identifier",
            Self::TooLong => "AID too long",
            Self::TruncatedLengthLargerThanLength => "truncated length too long",
            Self::NationalRidTooShort => "National RID must have length 5",
            Self::InternationalRidTooShort => "International RID must have length 5",
        })
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// ISO 7816-4 Application identifier
pub struct Aid {
    /// Array containing the AID (padded with zeros)
    ///
    /// Does not use heapless as its Vec is not `Copy`.
    bytes: [u8; Self::MAX_LEN],

    /// Length in bytes
    len: u8,

    /// Length used to truncated AID when matching SELECT requests
    truncated_len: u8,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Category {
    /// International registration of application providers according to ISO/IEC 7816-5
    International,
    /// National (ISO 3166-1) registration of application providers according to ISO/IEC 7816-5
    National,
    /// Identification of a standard by an object identifier according to ISO/IEC 8825-1
    Standard,
    /// No registration of application providers
    Proprietary,
    /// 0-9 are reserved for backwards compatibility, B-C are RFU.
    Other,
}

impl fmt::Debug for Aid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_bytes(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
            for b in bytes {
                write!(f, "{b:02X}")?;
            }
            Ok(())
        }

        fn write_truncated_bytes(
            f: &mut fmt::Formatter<'_>,
            bytes: &[u8],
            n: Option<usize>,
        ) -> fmt::Result {
            if let Some((head, tail)) = n.and_then(|n| bytes.split_at_checked(n)) {
                write_bytes(f, head)?;
                if !head.is_empty() && !tail.is_empty() {
                    f.write_str(" ")?;
                }
                write_bytes(f, tail)?;
            } else {
                write_bytes(f, bytes)?;
            }
            Ok(())
        }

        let t = usize::from(self.truncated_len);
        f.write_str("'")?;
        if let Some((rid, pix)) = self.rid_pix() {
            write_truncated_bytes(f, rid, Some(t))?;
            if !pix.is_empty() {
                f.write_str(" ")?;
            }
            write_truncated_bytes(f, pix, t.checked_sub(rid.len()))?;
        } else {
            write_truncated_bytes(f, self.as_bytes(), Some(t))?;
        }
        f.write_str("'")?;
        Ok(())
    }
}

// According to ISO 7816-4, "Application selection using AID as DF name":
// A multi-application card shall support the SELECT command with P1='04', P2='00' and a data field
// containing 5 to 16 bytes with the AID of an application that may reside on the card.
// The command shall complete successfully if the AID of an application the card holds matches the data field.

// It is also specified that:
// In a multi-application card an application in the card shall be identified by
//  a single AID in the proprietary, national or international category, and/or
//  one or more AIDs in the standard category.

pub trait App {
    // using an associated constant here would make the trait object unsafe
    fn aid(&self) -> Aid;
    //    fn select_via_aid(&mut self, interface: Interface, aid: Aid) -> Result<()>;
    //    fn deselect(&mut self) -> Result<()>;
    //    fn call(&mut self, interface: Interface, command: &Command<C>, response: &mut Response<R>) -> Result<()>;
}

impl core::ops::Deref for Aid {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Aid {
    const RID_LEN: usize = 5;
    const MAX_LEN: usize = 16;

    pub const fn as_bytes(&self) -> &[u8] {
        self.bytes.split_at(self.len as usize).0
    }

    pub const fn truncated(&self) -> &[u8] {
        self.bytes.split_at(self.truncated_len as usize).0
    }

    /// Checks whether this AID can be selected using the given AID, taking into account the
    /// configured truncated length.
    ///
    /// This function returns true if the given AID is a prefix of this AID and its length is at
    /// least the configured truncated length.
    pub fn matches(&self, aid: &[u8]) -> bool {
        if aid.len() < usize::from(self.truncated_len) {
            return false;
        }
        self.as_bytes().starts_with(aid)
    }

    /// Create an Aid
    ///
    /// This method panics if the given aid is invalid. For a similar method returning a result
    /// instead, use [try_new](Aid::try_new)
    pub const fn new(aid: &[u8]) -> Self {
        Self::new_truncatable(aid, aid.len())
    }

    /// Create an Aid that can be trucated in select commands
    ///
    /// This method panics if the given aid is invalid. For a similar method returning a result
    /// instead, use [try_new_truncatable](Aid::try_new_truncatable)
    pub const fn new_truncatable(aid: &[u8], truncated_len: usize) -> Self {
        match Self::try_new_truncatable(aid, truncated_len) {
            Ok(s) => s,
            Err(_e) => {
                panic!("Invalid aid")
            }
        }
    }

    /// Create an Aid
    pub const fn try_new(aid: &[u8]) -> Result<Self, FromSliceError> {
        Self::try_new_truncatable(aid, aid.len())
    }

    /// Create an Aid that can be trucated in select commands
    pub const fn try_new_truncatable(
        aid: &[u8],
        truncated_len: usize,
    ) -> Result<Self, FromSliceError> {
        if aid.is_empty() {
            return Err(FromSliceError::Empty);
        } else if aid.len() > Self::MAX_LEN {
            return Err(FromSliceError::TooLong);
        } else if truncated_len > aid.len() {
            return Err(FromSliceError::TruncatedLengthLargerThanLength);
        }
        let mut s = Self {
            bytes: [0u8; Self::MAX_LEN],
            len: aid.len() as u8,
            truncated_len: truncated_len as u8,
        };
        s = s.fill(aid, 0);
        if s.is_national() && aid.len() < Self::RID_LEN {
            return Err(FromSliceError::NationalRidTooShort);
        }
        if s.is_international() && aid.len() < Self::RID_LEN {
            return Err(FromSliceError::InternationalRidTooShort);
        }
        Ok(s)
    }

    // workaround to copy in the aid while remaining "const"
    // maybe there is a better way?
    const fn fill(mut self, bytes: &[u8], i: usize) -> Self {
        match i == bytes.len() {
            true => self,
            false => {
                self.bytes[i] = bytes[i];
                self.fill(bytes, i + 1)
            }
        }
    }

    pub const fn category(&self) -> Category {
        match self.bytes[0] >> 4 {
            0x0A => Category::International,
            0x0D => Category::National,
            0x0E => Category::Standard,
            0x0F => Category::Proprietary,
            _ => Category::Other,
        }
    }
    pub const fn is_international(&self) -> bool {
        // This is not "const" yet.
        // self.category() == Category::International
        matches!(self.category(), Category::International)
    }

    pub const fn is_national(&self) -> bool {
        matches!(self.category(), Category::National)
    }

    pub const fn is_standard(&self) -> bool {
        matches!(self.category(), Category::Standard)
    }

    pub const fn is_proprietary(&self) -> bool {
        matches!(self.category(), Category::Proprietary)
    }

    const fn has_rid_pix(&self) -> bool {
        self.is_national() || self.is_international()
    }

    const fn rid_pix(&self) -> Option<(&[u8; 5], &[u8])> {
        if self.has_rid_pix() {
            self.as_bytes().split_first_chunk()
        } else {
            None
        }
    }

    // pub fn rid(&self) -> &[u8; 5] {
    /// International or national registered application provider identifier, 5 bytes.
    pub fn rid(&self) -> Option<&[u8]> {
        self.has_rid_pix().then(|| &self.bytes[..5])
    }

    /// Proprietary application identifier extension, up to 11 bytes.
    pub fn pix(&self) -> Option<&[u8]> {
        self.has_rid_pix().then(|| &self.bytes[5..])
    }
}

#[cfg(test)]
mod test {
    use super::{Aid, Category, FromSliceError};
    use hex_literal::hex;
    const PIV_AID: Aid = Aid::new_truncatable(&hex!("A000000308 00001000 0100"), 9);

    #[test]
    fn aid() {
        let piv_aid = Aid::new(&hex!("A000000308 00001000 0100"));
        assert!(piv_aid.matches(&PIV_AID));
        assert!(PIV_AID.matches(&piv_aid));
        // panics
        // let aid = Aid::new(&hex_literal::hex!("A000000308 00001000 01001232323333333333333332"));
    }

    #[test]
    fn aid_fmt() {
        let piv_aid = Aid::new(&hex!("A000000308 00001000 0100"));
        let piv_aid_truncatable = Aid::new_truncatable(&hex!("A000000308 00001000 0100"), 9);
        assert_eq!(format!("{piv_aid:?}"), "'A000000308 000010000100'");
        assert_eq!(
            format!("{piv_aid_truncatable:?}"),
            "'A000000308 00001000 0100'"
        );

        // proprietary
        let aid1 = Aid::try_new(&hex!("F0000000030001")).unwrap();
        let aid2 = Aid::try_new_truncatable(aid1.as_bytes(), 1).unwrap();
        let aid3 = Aid::try_new_truncatable(aid1.as_bytes(), 5).unwrap();
        let aid4 = Aid::try_new_truncatable(aid1.as_bytes(), 6).unwrap();
        assert_eq!(format!("{aid1:?}"), "'F0000000030001'");
        assert_eq!(format!("{aid2:?}"), "'F0 000000030001'");
        assert_eq!(format!("{aid3:?}"), "'F000000003 0001'");
        assert_eq!(format!("{aid4:?}"), "'F00000000300 01'");

        // short proprietary
        let aid1 = Aid::try_new(&hex!("F000")).unwrap();
        let aid2 = Aid::try_new_truncatable(aid1.as_bytes(), 1).unwrap();
        assert_eq!(format!("{aid1:?}"), "'F000'");
        assert_eq!(format!("{aid2:?}"), "'F0 00'");
    }

    #[test]
    fn aid_too_short() {
        let result = Aid::try_new(&hex!("A0000008"));
        assert_eq!(result, Err(FromSliceError::InternationalRidTooShort));
        let result = Aid::try_new(&hex!("D0000008"));
        assert_eq!(result, Err(FromSliceError::NationalRidTooShort));

        // only national and internaional AIDs have a minimum length
        Aid::try_new(&hex!("F0")).unwrap();
        Aid::try_new(&hex!("00")).unwrap();
        Aid::try_new(&hex!("E0")).unwrap();
    }

    #[test]
    fn nk3_aids() {
        // AIDs used by the NK3

        // admin-app
        let aid = Aid::try_new(&hex!("A00000084700000001")).unwrap();
        assert_eq!(aid.category(), Category::International);

        // opcard
        let aid = Aid::try_new(&hex!("D276000124 01 0304 000F 00000000 0000")).unwrap();
        assert_eq!(aid.category(), Category::National);

        // piv-authenticator
        let aid = Aid::try_new(&hex!("A000000308 00001000 0100")).unwrap();
        assert_eq!(aid.category(), Category::International);

        // secrets-app
        let aid = Aid::try_new(&hex!("A000000527 2101")).unwrap();
        assert_eq!(aid.category(), Category::International);
    }

    #[test]
    fn known_aids() {
        // Example AIDs are based on this list:
        // https://www.eftlab.com/knowledge-base/complete-list-of-application-identifiers-aid

        // eGK
        let aid = Aid::try_new(&hex!("D2760001448000")).unwrap();
        assert_eq!(aid.category(), Category::National);

        // nPA
        let aid = Aid::try_new(&hex!("E80704007F00070302")).unwrap();
        assert_eq!(aid.category(), Category::Standard);

        // Bradesco
        let aid = Aid::try_new(&hex!("F0000000030001")).unwrap();
        assert_eq!(aid.category(), Category::Proprietary);

        // MiFare
        let aid = Aid::try_new(&hex!("6D6966617265")).unwrap();
        assert_eq!(aid.category(), Category::Other);
    }

    #[test]
    fn aid_matches() {
        let piv_rid = hex!("A000000308").as_slice();
        let piv_pix = hex!("000010000100").as_slice();
        let piv_aid: Vec<_> = [piv_rid, piv_pix].concat();

        let partial_rid = &piv_rid[..piv_rid.len() - 1];
        let partial_aid = &piv_aid[..piv_aid.len() - 1];

        let mut bad_aid1 = partial_aid.to_owned();
        bad_aid1.push(0xff);

        let bad_aid2 = hex!("D2760001448000").as_slice();

        let aid = Aid::try_new_truncatable(&piv_aid, piv_rid.len()).unwrap();
        assert!(!aid.matches(partial_rid));
        assert!(aid.matches(piv_rid));
        assert!(aid.matches(partial_aid));
        assert!(aid.matches(&piv_aid));
        assert!(!aid.matches(&bad_aid1));
        assert!(!aid.matches(bad_aid2));

        let aid = Aid::try_new(&piv_aid).unwrap();
        assert!(!aid.matches(partial_rid));
        assert!(!aid.matches(piv_rid));
        assert!(!aid.matches(partial_aid));
        assert!(aid.matches(&piv_aid));
        assert!(!aid.matches(&bad_aid1));
        assert!(!aid.matches(bad_aid2));
    }

    #[test]
    fn rid_pix() {
        // international
        let bytes = hex!("A000000308000010000100").as_slice();
        let aid = Aid::try_new(bytes).unwrap();
        assert_eq!(
            aid.rid_pix(),
            Some((bytes[..5].try_into().unwrap(), &bytes[5..]))
        );
        assert_eq!(aid.rid(), Some(&bytes[..5]));
        assert_eq!(aid.pix(), Some(&bytes[5..]));

        // national
        let bytes = hex!("D2760001448000").as_slice();
        let aid = Aid::try_new(bytes).unwrap();
        assert_eq!(
            aid.rid_pix(),
            Some((bytes[..5].try_into().unwrap(), &bytes[5..]))
        );
        assert_eq!(aid.rid(), Some(&bytes[..5]));
        assert_eq!(aid.pix(), Some(&bytes[5..]));

        // proprietary
        let aid = Aid::try_new(&hex!("F0000000030001")).unwrap();
        assert_eq!(aid.rid_pix(), None);
        assert_eq!(aid.rid(), None);
        assert_eq!(aid.pix(), None);

        // other
        let aid = Aid::try_new(&hex!("6D6966617265")).unwrap();
        assert_eq!(aid.rid_pix(), None);
        assert_eq!(aid.rid(), None);
        assert_eq!(aid.pix(), None);
    }
}
