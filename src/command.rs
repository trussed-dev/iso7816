use core::convert::TryFrom;

use crate::somebytes::{Bytes, TryExtendFromSlice};

pub mod class;
pub mod instruction;
pub use instruction::Instruction;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command<B: AsRef<[u8]>>
{
    class: class::Class,
    instruction: Instruction,

    pub p1: u8,
    pub p2: u8,

    data: Bytes<B>,

    le: usize,
    pub extended: bool,
}

impl<'a, B: AsRef<[u8]> + TryFrom<&'a [u8]>> Command<B>
{
    pub fn try_from(apdu: &'a [u8]) -> Result<Self, FromSliceError> {
        use core::convert::TryInto;
        apdu.try_into()
    }
}

impl<B: AsRef<[u8]>> Command<B>
{
    pub fn class(&self) -> class::Class {
        self.class
    }

    pub fn instruction(&self) -> Instruction {
        self.instruction
    }

    pub fn data(&self) -> &B {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut B {
        &mut self.data
    }

    pub fn expected(&self) -> usize {
        self.le
    }
}

impl<B: AsRef<[u8]> + TryExtendFromSlice<u8>> Command<B> {
    /// This can be use for APDU chaining to convert
    /// multiple APDU's into one.
    /// * Global Platform GPC_SPE_055 3.10
    pub fn extend_from_command<T: AsRef<[u8]>>(&mut self, command: &Command<T>) -> core::result::Result<(), B::Error> {

        // Always take the header from the last command;
        self.class = command.class();
        self.instruction = command.instruction();
        self.p1 = command.p1;
        self.p2 = command.p2;
        self.le = command.le;
        self.extended = true;

        // add the data to the end.
        self.data.extend_from_slice(command.data().as_ref())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FromSliceError {
    TooShort,
    TooLong,
    InvalidClass,
    InvalidFirstBodyByteForExtended,
    InvalidSliceLength,
}

impl From<class::InvalidClass> for FromSliceError {
    fn from(_: class::InvalidClass) -> Self {
        Self::InvalidClass
    }
}

impl<'a, B: AsRef<[u8]> + TryFrom<&'a [u8]>> TryFrom<&'a [u8]> for Command<B> {
    type Error = FromSliceError;
    fn try_from(apdu: &'a [u8]) -> core::result::Result<Self, Self::Error> {
        if apdu.len() < 4 {
            return Err(FromSliceError::TooShort);
        }
        #[cfg(test)]
        println!("{}", apdu.len());
        let (header, body) = apdu.split_at(4);
        let class = class::Class::try_from(header[0])?;
        let instruction = Instruction::from(header[1]);
        let p1 = header[2];
        let p2 = header[3];
        let parsed = parse_lengths(body)?;
        let data_slice = &body[parsed.offset..][..parsed.lc];

        Ok(Self {
            // header
            class, instruction, p1, p2,
            // maximum expected response length
            le: parsed.le,
            // payload
            data: B::try_from(data_slice).map_err(|_| Self::Error::TooLong)?.into(),
            extended: parsed.extended,
        })
    }
}

// cf. ISO 7816-3, 12.1.3: Decoding conventions for command APDUs
// freely available version:
// http://www.ttfn.net/techno/smartcards/iso7816_4.html#table5

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct ParsedLengths {
    lc: usize,
    le: usize,
    offset: usize,
    extended: bool,
}

#[inline(always)]
fn replace_zero(value: usize, replacement: usize) -> usize {
    if value == 0 {
        replacement
    } else {
        value
    }
}
#[inline]
fn parse_lengths(body: &[u8]) -> Result<ParsedLengths, FromSliceError> {

    // Encoding rules:
    // - Lc or Le = 0 => leave out
    // - short + extended length fields shall not be combined
    // - for extended, if Lc > 0, then Le has no leading zero byte

    let l = body.len();

    let mut parsed: ParsedLengths = Default::default();

    // Case 1
    if l == 0 {
        return Ok(parsed);
    }

    // the reference starts indexing at 1
    let b1 = body[0] as usize;

    #[cfg(test)]
    println!("l = {}, b1 = {}", l, b1);

    // Case 2S
    if l == 1 {
        parsed.lc = 0;
        parsed.le = replace_zero(b1, 256);
        return Ok(parsed)
    }

    // Case 3S
    if l == 1 + b1 && b1 != 0 {
        // B1 encodes Lc valued from 1 to 255
        parsed.lc = b1;
        parsed.le = 0;
        parsed.offset = 1;
        return Ok(parsed);
    }

    // Case 4S
    if l == 2 + b1 && b1 != 0 {
        // B1 encodes Lc valued from 1 to 255
        // Bl encodes Le from 1 to 256
        parsed.lc = b1;
        parsed.le = replace_zero(body[l - 1] as usize + 1, 256);
        parsed.offset = 1;
        return Ok(parsed);
    }

    parsed.extended = true;

    // only extended cases left now
    if b1 != 0 {
        return Err(FromSliceError::InvalidFirstBodyByteForExtended);
    };

    // Case 2E (no data)
    if l == 3 && b1 == 0 {
        parsed.lc = 0;
        parsed.le = replace_zero(
            u16::from_be_bytes([body[1], body[2]]) as usize,
            65_536);
        return Ok(parsed);
    }

    parsed.lc = u16::from_be_bytes([body[1], body[2]]) as usize;

    // Case 3E
    if l == 3 + parsed.lc {
        parsed.le = 0;
        parsed.offset = 3;
        return Ok(parsed);
    }

    // Case 4E
    if l == 5 + parsed.lc {
        parsed.le =  replace_zero(
            u16::from_be_bytes([body[l - 2], body[l - 1]]) as usize,
            65_536);
        parsed.offset = 3;
        return Ok(parsed);
    }

    // If we haven’t returned yet, the slice has an invalid length:  Either the encoded lc value is
    // wrong, or the lc and le lengths are not encoded properly (one byte per value for simple
    // APDU, two bytes per value for extended APDU).

    Err(FromSliceError::InvalidSliceLength)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_chaining() {
        let apdu = &[
            0x10, 0xdb, 0x3f, 0xff, 0xff, 0x5c, 0x03, 0x5f, 0xc1, 0x05, 0x53, 0x82, 0x01, 0x5b, 0x70, 0x82,
            0x01, 0x52, 0x30, 0x82, 0x01, 0x4e, 0x30, 0x81, 0xf5, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x11,
            0x00, 0x8b, 0xab, 0x31, 0xcf, 0x3e, 0xb9, 0xf5, 0x6a, 0x6f, 0x38, 0xf0, 0x5a, 0x4d, 0x7f, 0x55,
            0x62, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02, 0x30, 0x2a, 0x31,
            0x16, 0x30, 0x14, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x13, 0x0d, 0x79, 0x75, 0x62, 0x69, 0x6b, 0x65,
            0x79, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x31, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x04, 0x0b,
            0x13, 0x07, 0x28, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x29, 0x30, 0x20, 0x17, 0x0d, 0x32, 0x30, 0x30,
            0x35, 0x31, 0x36, 0x30, 0x31, 0x31, 0x37, 0x32, 0x36, 0x5a, 0x18, 0x0f, 0x32, 0x30, 0x36, 0x32,
            0x30, 0x35, 0x31, 0x36, 0x30, 0x32, 0x31, 0x37, 0x32, 0x36, 0x5a, 0x30, 0x12, 0x31, 0x10, 0x30,
            0x0e, 0x06, 0x03, 0x55, 0x04, 0x03, 0x13, 0x07, 0x53, 0x53, 0x48, 0x20, 0x6b, 0x65, 0x79, 0x30,
            0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86,
            0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x4f, 0x98, 0x63, 0x2f, 0x53, 0xbd,
            0xab, 0xee, 0xbf, 0x69, 0x73, 0x3a, 0x84, 0x0f, 0xfd, 0x9f, 0x9d, 0xb3, 0xce, 0x5c, 0x1e, 0x1b,
            0x84, 0x06, 0x63, 0x32, 0xff, 0x9c, 0x44, 0x0b, 0xce, 0x56, 0x13, 0x94, 0x00, 0x98, 0xe3, 0x46,
            0xc2, 0xbc, 0x3d, 0xe6, 0x5e, 0xf2, 0x81, 0x4b, 0xbc, 0xea, 0x2b, 0x9d, 0x47, 0xcc, 0x9b, 0x5e,
            0xbe, 0x1e, 0x2c, 0x69, 0x1d, 0xc3, 0x53, 0x4c, 0x89, 0x14, 0xa3, 0x12, 0x30, 0x10, 0x30, 0x0e,
            0x06, 0x03, 0x55, 0x1d,
        ];

        let command = Command::<heapless::Vec::<u8, 256>>::try_from(apdu).unwrap();
    }
}
