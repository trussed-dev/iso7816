// use core::convert::TryInto;

// // 7816-4, 8.2.1.2
// pub type Aid = crate::Bytes<heapless::consts::U16>;

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
    bytes: [u8; Self::max_len()],

    /// Length in bytes
    len: u8,

    /// Length used to truncated AID when matching SELECT requests
    truncated_len: u8,
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

#[allow(non_snake_case)]
pub const fn Aid(aid: &[u8], truncated_len: usize) -> Aid {
    Aid::new(aid, truncated_len)
}

pub trait App {
    fn aid(&self) -> Aid;
}

impl core::ops::Deref for Aid {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Aid {
    /// Maximum length of an AID.
    pub const fn max_len() -> usize {
        16
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }

    pub fn truncated(&self) -> &[u8] {
        &self.bytes[..self.truncated_len as usize]
    }

    pub fn matches(&self, aid: &[u8]) -> bool {
        aid.starts_with(self.truncated())
    }

    pub const fn new(aid: &[u8], truncated_len: usize) -> Self {
        const_assert!(!aid.is_empty(), "AID needs at least a category identifier");
        const_assert!(aid.len() <= Self::max_len(), "AID too long");
        const_assert!(truncated_len <= aid.len(), "truncated length too long");
        let mut s = Self { bytes: [0u8; Self::max_len()], len: aid.len() as u8, truncated_len: truncated_len as u8 };
        s = s.fill(aid, 0);
        const_assert!(!s.national() || aid.len() >= 5, "National RID must have length 5");
        const_assert!(!s.international() || aid.len() >= 5, "International RID must have length 5");
        s
    }

    // pub fn try_new(aid: &[u8], truncated_len: u8) -> Result<Self, ()> {
    //     if aid.len() > Self::max_len() {
    //         return Err(());
    //     }
    //     if truncated_len > aid.len() {
    //         return Err(());
    //     }
    // }

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

    pub const fn international(&self) -> bool {
        (self.bytes[0] >> 4) == b'A'
    }

    pub const fn national(&self) -> bool {
        (self.bytes[0] >> 4) == b'D'
    }

    pub const fn standard(&self) -> bool {
        (self.bytes[0] >> 4) == b'E'
    }

    pub const fn proprietary(&self) -> bool {
        (self.bytes[0] >> 4) == b'F'
    }

    // pub fn rid(&self) -> &[u8; 5] {
    /// International or national registered application provider identifier, 5 bytes.
    pub fn rid(&self) -> Option<&[u8]> {
        if self.national() || self.international() {
            Some(&self.bytes[..5])
        } else {
            // "RID not defined"
            None
        }
    }

    /// Proprietary application identifier extension, up to 11 bytes.
    pub fn pix(&self) -> Option<&[u8]> {
        if self.national() || self.international() {
            Some(&self.bytes[5..])
        } else {
            // "PIX not defined"
            None
        }
    }

}

#[cfg(test)]
mod test {
    use super::Aid;
    #[allow(dead_code)]
    const PIV_AID: Aid = Aid::new(&hex_literal::hex!("A000000308 00001000 0100"), 11);

    #[test]
    fn non_const_aid() {
        let aid = Aid::new(&hex_literal::hex!("A000000308 00001000 0100"), 11);
        // panics
        // let aid = Aid::new(&hex_literal::hex!("A000000308 00001000 01001232323333333333333332"));
    }
}
