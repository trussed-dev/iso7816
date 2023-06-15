use crate::Data;

pub mod class;
pub mod instruction;
pub use instruction::Instruction;

mod writer;
pub use writer::{BufferFull, Writer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command<const S: usize> {
    class: class::Class,
    instruction: Instruction,

    p1: u8,
    p2: u8,

    /// The main reason this is modeled as Bytes and not
    /// a fixed array is for serde purposes.
    data: Data<S>,

    le: usize,
    extended: bool,
}

impl<const S: usize> Command<S> {
    pub fn try_from(apdu: &[u8]) -> Result<Self, FromSliceError> {
        apdu.try_into()
    }

    pub fn class(&self) -> class::Class {
        self.class
    }

    pub fn instruction(&self) -> Instruction {
        self.instruction
    }

    pub fn data(&self) -> &Data<S> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Data<S> {
        &mut self.data
    }

    pub fn expected(&self) -> usize {
        self.le
    }

    pub fn as_view(&self) -> CommandView {
        CommandView {
            class: self.class,
            instruction: self.instruction,
            p1: self.p1,
            p2: self.p2,
            data: self.data(),
            le: self.le,
            extended: self.extended,
        }
    }

    pub fn p1(&self) -> u8 {
        self.p1
    }

    pub fn p2(&self) -> u8 {
        self.p2
    }

    pub fn extended(&self) -> bool {
        self.extended
    }

    /// This can be use for APDU chaining to convert
    /// multiple APDU's into one.
    /// * Global Platform GPC_SPE_055 3.10
    #[allow(clippy::result_unit_err)]
    pub fn extend_from_command<const T: usize>(
        &mut self,
        command: &Command<T>,
    ) -> core::result::Result<(), ()> {
        self.extend_from_command_view(command.as_view())
    }

    /// This can be use for APDU chaining to convert
    /// multiple APDU's into one.
    /// * Global Platform GPC_SPE_055 3.10
    #[allow(clippy::result_unit_err)]
    pub fn extend_from_command_view(
        &mut self,
        command: CommandView,
    ) -> core::result::Result<(), ()> {
        // Always take the header from the last command;
        self.class = command.class();
        self.instruction = command.instruction();
        self.p1 = command.p1;
        self.p2 = command.p2;
        self.le = command.le;
        self.extended = true;

        // add the data to the end.
        self.data.extend_from_slice(command.data())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Memory-efficient unowned version of [`Command`]
pub struct CommandView<'a> {
    class: class::Class,
    instruction: Instruction,

    p1: u8,
    p2: u8,

    data: &'a [u8],

    le: usize,
    extended: bool,
}

impl<'a> CommandView<'a> {
    pub fn class(&self) -> class::Class {
        self.class
    }

    pub fn instruction(&self) -> Instruction {
        self.instruction
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn expected(&self) -> usize {
        self.le
    }

    pub fn p1(&self) -> u8 {
        self.p1
    }

    pub fn p2(&self) -> u8 {
        self.p2
    }

    pub fn extended(&self) -> bool {
        self.extended
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandBuilder<'a> {
    class: class::Class,
    instruction: Instruction,

    p1: u8,
    p2: u8,

    data: &'a [u8],

    le: u16,
}

const HEADER_LEN: usize = 4;

impl<'a> CommandBuilder<'a> {
    /// Panics if data.len() > u16::MAX
    pub fn new(
        class: class::Class,
        instruction: instruction::Instruction,
        p1: u8,
        p2: u8,
        data: &'a [u8],
        le: u16,
    ) -> Self {
        assert!(data.len() <= u16::MAX as usize);
        Self {
            class,
            instruction,
            p1,
            p2,
            data,
            le,
        }
    }

    fn header_data(&self, supports_extended_length: bool) -> BuildingHeaderData {
        /// Returns (data, len of data, and is_extended)
        fn serialize_data_len(len: u16, expected_len: u16) -> (heapless::Vec<u8, 3>, bool) {
            match (len, expected_len > 256) {
                (0, _) => (Default::default(), false),
                (1..=255, false) => ([len as u8].as_slice().try_into().unwrap(), false),
                _ => {
                    let l = len.to_be_bytes();
                    ([0, l[0], l[1]].as_slice().try_into().unwrap(), true)
                }
            }
        }

        fn serialize_expected_len(
            len: u16,
            lc_extended: bool,
            data_is_empty: bool,
        ) -> heapless::Vec<u8, 3> {
            match (len, lc_extended, data_is_empty) {
                (0, _, _) => Default::default(),
                (1..=255, false, _) => [len as u8].as_slice().try_into().unwrap(),
                (256, false, _) => [0].as_slice().try_into().unwrap(),
                (_, true, false) => {
                    let l = len.to_be_bytes();
                    [l[0], l[1]].as_slice().try_into().unwrap()
                }
                (_, false, true) => {
                    let l = len.to_be_bytes();
                    [0, l[0], l[1]].as_slice().try_into().unwrap()
                }
                (257.., false, false) => unreachable!("Can't have non extended Lc and extended Le"),
                (_, true, true) => unreachable!("Can't have both no data and data extended length"),
            }
        }

        let le = if supports_extended_length {
            self.le
        } else {
            self.le.min(256)
        };

        // Safe to unwrap because of check in `new`
        let (data_len, lc_extended) = serialize_data_len(self.data.len().try_into().unwrap(), le);

        let expected_data_len = serialize_expected_len(le, lc_extended, self.data.is_empty());
        BuildingHeaderData {
            le,
            data_len,
            expected_data_len,
        }
    }

    /// Required length for serialization in only one command.
    /// Assumes extended length support
    ///
    /// This can be useful to get the necessary dimension for the buffer to provide to [serialize_into](Self::serialize_into)
    pub fn required_len(&self, supports_extended_length: bool) -> usize {
        let header_data = self.header_data(supports_extended_length);
        let header_len = 4;
        let length_len = header_data.data_len.len() + header_data.expected_data_len.len();
        header_len + length_len + self.data.len()
    }

    /// Serialize into one vector with assuming support for extended length information
    #[cfg(any(feature = "std", test))]
    pub fn serialize_to_vec(self, supports_extended_length: bool) -> Vec<u8> {
        let required_len = self.required_len(supports_extended_length);
        let mut buffer = Vec::with_capacity(required_len);
        self.serialize_into(&mut buffer, supports_extended_length)
            .unwrap();
        debug_assert_eq!(required_len, buffer.len());
        buffer
    }

    /// Given the available length and the extended length support, split the command in 2 commands that use command chaining to be sent
    ///
    /// `None` means that the command can we serialized withinn `available_len` without needing Chaining
    /// `Some(command, rem)` means that `command` can be sent within `available_len` and that `rem` must then be sent (for command chaining). Note that `should_split` should also be called on `rem` as more than 2 commands might be required.
    ///
    /// In certain conditions can panic if `available_len <= 9` since 9 is the minimum length required to encode the header and trailer of a command.
    pub fn should_split(
        &self,
        available_len: usize,
        supports_extended_length: bool,
    ) -> Option<(Self, Self)> {
        if available_len < HEADER_LEN {
            panic!("Commands cannot be encoded to fit in buffers smaller than 9 bytes");
        }

        let BuildingHeaderData {
            le,
            data_len,
            expected_data_len,
        } = self.header_data(supports_extended_length);

        let mut max_data_len = u16::MAX as usize;
        if !supports_extended_length {
            max_data_len = 255;
        }

        let available_data_len = (available_len - HEADER_LEN)
            .saturating_sub(data_len.len() + expected_data_len.len())
            .min(max_data_len);
        if available_data_len >= self.data.len() {
            // slitting not necessary
            return None;
        }

        if available_data_len == 0 {
            // Let's not support this case
            panic!("Commands cannot be encoded to fit in buffers smaller than 9 bytes");
        }

        let (send_now, send_later) = self.data.split_at(available_data_len);

        let send_now = Self {
            class: self.class.as_chained(),
            instruction: self.instruction,
            p1: self.p1,
            p2: self.p2,
            data: send_now,
            le: 0,
        };
        let send_later = Self {
            class: self.class,
            instruction: self.instruction,
            p1: self.p1,
            p2: self.p2,
            data: send_later,
            le,
        };
        Some((send_now, send_later))
    }

    /// This assumes that the writer has enough space to encode the APDU.
    /// If that might not be the case, first use [`should_split`](Self::should_split)
    pub fn serialize_into<W: Writer>(
        self,
        writer: &mut W,
        supports_extended_length: bool,
    ) -> Result<(), W::Error> {
        let BuildingHeaderData {
            data_len,
            expected_data_len,
            ..
        } = self.header_data(supports_extended_length);

        writer.write_all(&[
            self.class.into_inner(),
            self.instruction.into(),
            self.p1,
            self.p2,
        ])?;

        writer.write_all(&data_len)?;
        writer.write_all(self.data)?;
        writer.write_all(&expected_data_len)?;
        Ok(())
    }
}

struct BuildingHeaderData {
    le: u16,
    data_len: heapless::Vec<u8, 3>,
    expected_data_len: heapless::Vec<u8, 3>,
}

impl<'a, 'b> PartialEq<CommandView<'a>> for CommandBuilder<'b> {
    fn eq(&self, other: &CommandView<'a>) -> bool {
        let Self {
            class,
            instruction,
            p1,
            p2,
            data,
            le,
        } = self;
        class == &other.class
            && instruction == &other.instruction
            && p1 == &other.p1
            && p2 == &other.p2
            && data == &other.data
            && *le as usize == other.le
    }
}

impl<'a, 'b> PartialEq<CommandBuilder<'a>> for CommandView<'b> {
    fn eq(&self, other: &CommandBuilder<'a>) -> bool {
        other == self
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

impl<'a> TryFrom<&'a [u8]> for CommandView<'a> {
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
        let data = &body[parsed.offset..][..parsed.lc];

        Ok(Self {
            // header
            class,
            instruction,
            p1,
            p2,
            // maximum expected response length
            le: parsed.le,
            // payload
            data,
            extended: parsed.extended,
        })
    }
}

impl<'a> CommandView<'a> {
    pub fn to_owned<const S: usize>(&self) -> Result<Command<S>, FromSliceError> {
        let &CommandView {
            class,
            instruction,
            p1,
            p2,
            le,
            data,
            extended,
        } = self;
        Ok(Command {
            // header
            class,
            instruction,
            p1,
            p2,
            // maximum expected response length
            le,
            // payload
            data: Data::from_slice(data).map_err(|_| FromSliceError::TooLong)?,
            extended,
        })
    }
}

impl<const S: usize> TryFrom<&[u8]> for Command<S> {
    type Error = FromSliceError;
    fn try_from(apdu: &[u8]) -> core::result::Result<Self, Self::Error> {
        let view: CommandView = apdu.try_into()?;
        view.to_owned()
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
        return Ok(parsed);
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
        parsed.le = replace_zero(body[l - 1] as usize, 256);
        parsed.offset = 1;
        return Ok(parsed);
    }

    parsed.extended = true;

    // only extended cases left now
    if b1 != 0 {
        return Err(FromSliceError::InvalidFirstBodyByteForExtended);
    } else if l < 3 {
        return Err(FromSliceError::InvalidSliceLength);
    }

    // Case 2E (no data)
    if l == 3 && b1 == 0 {
        parsed.lc = 0;
        parsed.le = replace_zero(u16::from_be_bytes([body[1], body[2]]) as usize, 65_536);
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
        parsed.le = replace_zero(
            u16::from_be_bytes([body[l - 2], body[l - 1]]) as usize,
            65_536,
        );
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
    use hex_literal::hex;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn parse_no_panic(data: Vec<u8>) {
        let _ = parse_lengths(&data);
    }

    #[quickcheck]
    fn lengths(lc: u16, le: Option<u16>) {
        let extended =
            lc > u16::from(u8::MAX) || le.map(|val| val > u16::from(u8::MAX)).unwrap_or_default();
        let nc = usize::from(lc);
        let ne = le
            .map(usize::from)
            .map(|val| {
                if val == 0 {
                    (if extended {
                        usize::from(u16::MAX)
                    } else {
                        usize::from(u8::MAX)
                    }) + 1
                } else {
                    val
                }
            })
            .unwrap_or_default();

        let mut data = Vec::new();
        let mut offset = 0;

        if lc > 0 {
            if extended {
                data.push(0);
                data.extend_from_slice(&lc.to_be_bytes());
                offset = 3;
            } else {
                data.push(lc as u8);
                offset = 1;
            }
        }

        for _ in 0..nc {
            data.push(0);
        }

        if let Some(le) = le {
            if extended {
                if lc == 0 {
                    data.push(0);
                }
                data.extend_from_slice(&le.to_be_bytes());
            } else {
                data.push(le as u8);
            }
        }

        let lengths = parse_lengths(&data).expect("failed to parse lengths");
        assert_eq!(extended, lengths.extended);
        assert_eq!(offset, lengths.offset);
        assert_eq!(nc, lengths.lc);
        assert_eq!(ne, lengths.le);
    }

    #[test]
    fn builder() {
        let cla = 0.try_into().unwrap();
        let ins = 1.into();
        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 0x04);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 04"));

        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 0x00);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03"));

        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 256);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 00"));
        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 257);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 00 0101"));
        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 0xFFFF);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 00 FFFF"));

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x05, 0x06], 0x04);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 02 05 06 04"));

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x05, 0x06], 0x00);
        assert_eq!(command.serialize_to_vec(), &hex!("00 01 02 03 02 05 06"));

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x05, 0x06], 0x100);
        assert_eq!(
            command.serialize_to_vec(),
            // Large LE also forces the data length to be extended (can't mix extended/non-extended)
            &hex!("00 01 02 03 02 05 06 00")
        );

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x01; 0x2AE], 0x100);
        assert_eq!(
            command.serialize_to_vec(),
            [
                hex!("00 01 02 03 00 02AE").as_slice(),
                &[0x01; 0x2AE],
                &hex!("01 00"),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x01; 0x2AE], 0x01);
        assert_eq!(
            command.serialize_to_vec(),
            [
                hex!("00 01 02 03 00 02AE").as_slice(),
                &[0x01; 0x2AE],
                &hex!("00 01"),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x01; 0x2AE], 0x00);
        assert_eq!(
            command.serialize_to_vec(),
            [hex!("00 01 02 03 00 02AE").as_slice(), &[0x01; 0x2AE],]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<u8>>()
        );

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x01; 0x2AE], 0xFF);
        assert_eq!(
            command.serialize_to_vec(),
            [
                hex!("00 01 02 03 00 02AE").as_slice(),
                &[0x01; 0x2AE],
                &[0x00, 0xFF]
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );

        let command = CommandBuilder::new(cla, ins, 2, 3, &[0x01; 0xFFFF], 0xFFFF);
        assert_eq!(
            command.serialize_to_vec(),
            [
                hex!("00 01 02 03 00 FFFF").as_slice(),
                &[0x01; 0xFFFF],
                &[0xFF, 0xFF]
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );
    }

    #[test]
    fn building_chained() {
        let cla = 0x00.try_into().unwrap();
        let ins = 0x01.into();
        let mut buffer = heapless::Vec::<u8, 4096>::new();
        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 0xFFFF);
        command
            .clone()
            .serialize_into(&mut buffer, true)
            .unwrap()
            .unwrap();
        assert_eq!(&*buffer, &command.clone().serialize_to_vec());

        buffer.clear();
        //  without extended length
        command
            .clone()
            .serialize_into(&mut buffer, false)
            .unwrap()
            .unwrap();
        assert_eq!(
            &*buffer,
            &CommandBuilder::new(cla, ins, 2, 3, &[], 0x0100).serialize_to_vec()
        );

        buffer.clear();
        //  without extended length
        let command = CommandBuilder::new(cla, ins, 2, 3, &[], 0);
        command
            .clone()
            .serialize_into(&mut buffer, false)
            .unwrap()
            .unwrap();
        assert_eq!(
            &*buffer,
            &CommandBuilder::new(cla, ins, 2, 3, &[], 0).serialize_to_vec()
        );
        buffer.clear();

        let mut buffer = heapless::Vec::<u8, 105>::new();

        let command = CommandBuilder::new(cla, ins, 2, 3, &[5; 200], 0);
        let rem = command
            .serialize_into(&mut buffer, false)
            .unwrap()
            .unwrap_err();
        assert_eq!(buffer.len(), 105);
        assert_eq!(rem, CommandBuilder::new(cla, ins, 2, 3, &[5; 100], 0));
        assert_eq!(
            &*buffer,
            &CommandBuilder::new(cla.as_chained(), ins, 2, 3, &[5; 100], 0).serialize_to_vec()
        );
    }

    #[test]
    fn lengths_4s() {
        let data = &[0x02, 0xB6, 0x00, 0x00];
        let lengths = parse_lengths(data).expect("failed to parse lengths");
        assert_eq!(lengths.lc, 2);
        assert_eq!(lengths.le, 256);
        assert_eq!(lengths.offset, 1);
    }

    #[test]
    fn command_chaining() {
        let apdu = &[
            0x10, 0xdb, 0x3f, 0xff, 0xff, 0x5c, 0x03, 0x5f, 0xc1, 0x05, 0x53, 0x82, 0x01, 0x5b,
            0x70, 0x82, 0x01, 0x52, 0x30, 0x82, 0x01, 0x4e, 0x30, 0x81, 0xf5, 0xa0, 0x03, 0x02,
            0x01, 0x02, 0x02, 0x11, 0x00, 0x8b, 0xab, 0x31, 0xcf, 0x3e, 0xb9, 0xf5, 0x6a, 0x6f,
            0x38, 0xf0, 0x5a, 0x4d, 0x7f, 0x55, 0x62, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48,
            0xce, 0x3d, 0x04, 0x03, 0x02, 0x30, 0x2a, 0x31, 0x16, 0x30, 0x14, 0x06, 0x03, 0x55,
            0x04, 0x0a, 0x13, 0x0d, 0x79, 0x75, 0x62, 0x69, 0x6b, 0x65, 0x79, 0x2d, 0x61, 0x67,
            0x65, 0x6e, 0x74, 0x31, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x04, 0x0b, 0x13, 0x07,
            0x28, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x29, 0x30, 0x20, 0x17, 0x0d, 0x32, 0x30, 0x30,
            0x35, 0x31, 0x36, 0x30, 0x31, 0x31, 0x37, 0x32, 0x36, 0x5a, 0x18, 0x0f, 0x32, 0x30,
            0x36, 0x32, 0x30, 0x35, 0x31, 0x36, 0x30, 0x32, 0x31, 0x37, 0x32, 0x36, 0x5a, 0x30,
            0x12, 0x31, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x04, 0x03, 0x13, 0x07, 0x53, 0x53,
            0x48, 0x20, 0x6b, 0x65, 0x79, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48,
            0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07,
            0x03, 0x42, 0x00, 0x04, 0x4f, 0x98, 0x63, 0x2f, 0x53, 0xbd, 0xab, 0xee, 0xbf, 0x69,
            0x73, 0x3a, 0x84, 0x0f, 0xfd, 0x9f, 0x9d, 0xb3, 0xce, 0x5c, 0x1e, 0x1b, 0x84, 0x06,
            0x63, 0x32, 0xff, 0x9c, 0x44, 0x0b, 0xce, 0x56, 0x13, 0x94, 0x00, 0x98, 0xe3, 0x46,
            0xc2, 0xbc, 0x3d, 0xe6, 0x5e, 0xf2, 0x81, 0x4b, 0xbc, 0xea, 0x2b, 0x9d, 0x47, 0xcc,
            0x9b, 0x5e, 0xbe, 0x1e, 0x2c, 0x69, 0x1d, 0xc3, 0x53, 0x4c, 0x89, 0x14, 0xa3, 0x12,
            0x30, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x1d,
        ];

        let _command = Command::<256>::try_from(apdu).unwrap();
    }

    #[test]
    fn lc_oob() {
        let apdu = &hex!("00C00000 00FF");
        let _ = Command::<256>::try_from(apdu);
        let apdu = &hex!("00C00000 0000");
        let _ = Command::<256>::try_from(apdu);
    }
}
