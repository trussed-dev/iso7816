// use crate::{Command, Interface, Response, Result};

/// Constant panicking assertion.
// TODO(tarcieri): use const panic when stable.
// See: https://github.com/rust-lang/rust/issues/51999
macro_rules! const_assert {
    ($bool:expr, $msg:expr) => {
        [$msg][!$bool as usize]
    };
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

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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

impl core::fmt::Debug for Aid {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      if self.len <= self.truncated_len {
          f.write_fmt(format_args!("'{} {}'",
              hexstr!(&self.bytes[..5]),
              hexstr!(&self.bytes[5..self.len as _])))
      } else {
          f.write_fmt(format_args!("'{} {} {}'",
              hexstr!(&self.bytes[..5]),
              hexstr!(&self.bytes[5..self.truncated_len as _]),
              hexstr!(&self.bytes[self.truncated_len as _..self.len as _])))
      }
  }
}

/// According to ISO 7816-4, "Application selection using AID as DF name":
/// A multi-application card shall support the SELECT command with P1='04', P2='00' and a data field
/// containing 5 to 16 bytes with the AID of an application that may reside on the card.
/// The command shall complete successfully if the AID of an application the card holds matches the data field.
///
/// It is also specified that:
/// In a multi-application card an application in the card shall be identified by
///  a single AID in the proprietary, national or international category, and/or
///  one or more AIDs in the standard category.

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
    const MAX_LEN: usize = 16;

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }

    pub fn truncated(&self) -> &[u8] {
        &self.bytes[..self.truncated_len as usize]
    }

    pub fn matches(&self, aid: &[u8]) -> bool {
        aid.starts_with(self.truncated())
    }

    pub const fn new(aid: &[u8]) -> Self {
        Self::new_truncatable(aid, aid.len())
    }

    // pub fn try_new(aid: &[u8], truncated_len: u8) -> Result<Self, ()> {
    pub const fn new_truncatable(aid: &[u8], truncated_len: usize) -> Self {
        const_assert!(!aid.is_empty(), "AID needs at least a category identifier");
        const_assert!(aid.len() <= Self::MAX_LEN, "AID too long");
        const_assert!(truncated_len <= aid.len(), "truncated length too long");
        let mut s = Self { bytes: [0u8; Self::MAX_LEN], len: aid.len() as u8, truncated_len: truncated_len as u8 };
        s = s.fill(aid, 0);
        const_assert!(!s.is_national() || aid.len() >= 5, "National RID must have length 5");
        const_assert!(!s.is_international() || aid.len() >= 5, "International RID must have length 5");
        s
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
            b'A' => Category::International,
            b'D' => Category::National,
            b'E' => Category::Standard,
            b'F' => Category::Proprietary,
            _ => Category::Other,
        }
    }
    pub const fn is_international(&self) -> bool {
        // This is not "const" yet.
        // self.category() == Category::International
        match self.category() {
            Category::International => true,
            _ => false,
        }
    }

    pub const fn is_national(&self) -> bool {
        match self.category() {
            Category::National => true,
            _ => false,
        }
    }

    pub const fn is_standard(&self) -> bool {
        match self.category() {
            Category::Standard => true,
            _ => false,
        }
    }

    pub const fn is_proprietary(&self) -> bool {
        match self.category() {
            Category::Proprietary => true,
            _ => false,
        }
    }

    const fn has_rid_pix(&self) -> bool {
        self.is_national() || self.is_international()
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
    use super::Aid;
    use hex_literal::hex;
    #[allow(dead_code)]
    const PIV_AID: Aid = Aid::new_truncatable(&hex!("A000000308 00001000 0100"), 9);

    #[test]
    fn aid() {
        let piv_aid = Aid::new(&hex!("A000000308 00001000 0100"));
        assert!(piv_aid.matches(&*PIV_AID));
        assert!(PIV_AID.matches(&*piv_aid));
        // panics
        // let aid = Aid::new(&hex_literal::hex!("A000000308 00001000 01001232323333333333333332"));
    }
}
