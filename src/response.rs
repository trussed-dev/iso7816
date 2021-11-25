mod status;
pub use status::Status;

use crate::somebytes::Bytes;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response<B: AsRef<[u8]>> {
    Data(Bytes<B>),
    Status(Status),
}

impl<B: AsRef<[u8]>> Default for Response<B> {
    fn default() -> Self {
        Self::Status(Default::default())
    }
}
