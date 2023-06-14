use core::convert::Infallible;
use core::fmt::{Debug, Display};
use core::mem::replace;

#[derive(Debug)]
pub struct BufferFull;

impl Display for BufferFull {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer is full")
    }
}

pub trait Writer {
    type Error: Debug + Display;

    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error>;
    /// data must be smaller than [`remaining_len`](Writer::remaining_len)
    fn write_all(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        let mut offset = 0;
        while offset < data.len() {
            offset += self.write(data)?;
        }
        Ok(())
    }
}

impl<'a> Writer for &'a mut [u8] {
    type Error = BufferFull;
    fn write(&mut self, data: &[u8]) -> Result<usize, BufferFull> {
        let amt = data.len().min(self.len());

        if amt == 0 {
            return Err(BufferFull);
        }

        let (a, b) = replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }
}

impl<const N: usize> Writer for heapless::Vec<u8, N> {
    type Error = BufferFull;
    fn write(&mut self, data: &[u8]) -> Result<usize, BufferFull> {
        let amt = data.len().min(self.capacity() - self.len());

        if amt == 0 {
            return Err(BufferFull);
        }

        self.extend_from_slice(&data[..amt]).unwrap();
        Ok(amt)
    }
}

#[cfg(feature = "heapless_bytes")]
impl<const N: usize> Writer for heapless_bytes::Bytes<N> {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        let amt = data.len().min(self.capacity() - self.len());

        if amt == 0 {
            return Err(BufferFull);
        }

        self.extend_from_slice(&data[..amt]).unwrap();
        Ok(amt)
    }
}

#[cfg(any(feature = "std", test))]
impl Writer for Vec<u8> {
    type Error = Infallible;
    fn write(&mut self, data: &[u8]) -> Result<usize, Infallible> {
        self.extend_from_slice(data);
        Ok(data.len())
    }
}
