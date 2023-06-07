use core::convert::Infallible;
use core::fmt::{Debug, Display};
use core::mem::replace;

pub trait Writer {
    type Error: Debug + Display;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error>;

    fn remaining_len(&self) -> usize;

    /// data must be smaller than [`remaining_len`](Writer::remaining_len)
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        debug_assert!(data.len() <= self.remaining_len());
        let mut offset = 0;
        while offset < data.len() {
            offset += self.write(data)?;
        }
        Ok(())
    }
}

impl<'a> Writer for &'a mut [u8] {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        let amt = data.len().min(self.len());
        let (a, b) = replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    fn remaining_len(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> Writer for heapless::Vec<u8, N> {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        let written_len = data.len().min(self.capacity() - self.len());
        self.extend_from_slice(&data[..written_len]).unwrap();
        Ok(written_len)
    }

    fn remaining_len(&self) -> usize {
        self.capacity() - self.len()
    }
}

#[cfg(feature = "heapless_bytes")]
impl<const N: usize> Writer for heapless_bytes::Bytes<N> {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        let written_len = data.len().min(self.capacity() - self.len());
        self.extend_from_slice(&data[..written_len]).unwrap();
        Ok(written_len)
    }

    fn remaining_len(&self) -> usize {
        self.capacity() - self.len()
    }
}

#[cfg(any(feature = "std", test))]
impl Writer for Vec<u8> {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        self.extend_from_slice(data);
        Ok(data.len())
    }

    fn remaining_len(&self) -> usize {
        usize::MAX
    }
}
