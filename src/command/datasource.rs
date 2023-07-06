pub trait DataSource {
    /// Length of the serialized data
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Datasource for APDU serialization
///
/// W is a type parameter of the trait and not the method to make the trait dyn-safe
pub trait DataStream<W: super::Writer>: DataSource {
    /// Serialize the data to a writer.
    ///
    /// The length of the data serialized to the writer must not exceed the value returned by `len`.
    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error>;
}

impl DataSource for [u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    fn is_empty(&self) -> bool {
        <[u8]>::is_empty(&self)
    }
}

impl<W: super::Writer> DataStream<W> for [u8] {
    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error> {
        writer.write_all(self)
    }
}

impl DataSource for [&dyn DataSource] {
    fn len(&self) -> usize {
        self.iter().map(|item| item.len()).sum()
    }

    fn is_empty(&self) -> bool {
        self.iter().find(|item| !item.is_empty()).is_some()
    }
}

impl<W: super::Writer> DataSource for [&dyn DataStream<W>] {
    fn len(&self) -> usize {
        self.iter().map(|item| item.len()).sum()
    }

    fn is_empty(&self) -> bool {
        self.iter().find(|item| !item.is_empty()).is_some()
    }
}

impl<W: super::Writer> DataStream<W> for [&dyn DataStream<W>] {
    fn to_writer(&self, writer: &mut W) -> Result<(), W::Error> {
        for item in self {
            item.to_writer(writer)?;
        }
        Ok(())
    }
}
