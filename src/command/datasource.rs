/// W is a type parameter of the trait and not the method to make the trait dyn-safe
#[allow(clippy::len_without_is_empty)]
pub trait DataSource<W: super::Writer> {
    fn len(&self) -> usize;
    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error>;
}

impl<W: super::Writer> DataSource<W> for &[u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }
    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error> {
        writer.write_all(self)
    }
}

impl<W: super::Writer> DataSource<W> for [&dyn DataSource<W>] {
    fn len(&self) -> usize {
        self.iter().map(|item| item.len()).sum()
    }

    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error> {
        for item in self {
            item.to_writer(writer)?;
        }
        Ok(())
    }
}
