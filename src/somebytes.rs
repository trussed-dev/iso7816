// This could be a separate crate.

use core::{cmp, fmt, ops};

pub trait TryExtendFromSlice<T> {
    type Error;
    fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), Self::Error>;
}

impl<T: Clone, const N: usize> TryExtendFromSlice<T> for heapless::Vec<T, N> {
    type Error = ();
    fn extend_from_slice(&mut self, slice: &[T]) -> Result<(), Self::Error> {
        heapless::Vec::extend_from_slice(self, slice)
    }
}

/// A wrapper type for a byte sequence.
///
/// This wrapper implements common traits based on the content of the byte sequence.
#[derive(Clone)]
pub struct Bytes<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> Bytes<T> {
    pub fn into_bytes(self) -> T {
        self.0
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Bytes<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: AsRef<[u8]>> ops::Deref for Bytes<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: AsRef<[u8]>> ops::DerefMut for Bytes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: AsRef<[u8]>> From<T> for Bytes<T> {
    fn from(bytes: T) -> Self {
        Self(bytes)
    }
}

impl<T: AsRef<[u8]>> fmt::Debug for Bytes<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_ref().fmt(f)
    }
}

impl<T: AsRef<[u8]>> Eq for Bytes<T> {}

impl<T: AsRef<[u8]>> PartialEq for Bytes<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().eq(other.0.as_ref())
    }
}

impl<T: AsRef<[u8]>> Ord for Bytes<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}

impl<T: AsRef<[u8]>> PartialOrd for Bytes<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
    }
}
