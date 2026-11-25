//! BER-TLV writer and parser

use crate::command::{writer::Error as _, DataSource, DataStream, Writer};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Tag([u8; 3]);

impl Tag {
    pub const fn from_u8(value: u8) -> Self {
        Tag([0, 0, value])
    }

    pub const fn from_u16(value: u16) -> Self {
        Tag::from_2(value.to_be_bytes())
    }

    pub const fn from_2([b1, b2]: [u8; 2]) -> Self {
        if b1 == 0 {
            Tag([0, 0, b2])
        } else {
            Tag([0, b1, b2])
        }
    }
    pub const fn from_3([b1, b2, b3]: [u8; 3]) -> Self {
        if b1 == 0 {
            Self::from_2([b2, b3])
        } else {
            Tag([b1, b2, b3])
        }
    }
}

impl From<u8> for Tag {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<u16> for Tag {
    fn from(value: u16) -> Self {
        value.to_be_bytes().into()
    }
}

impl From<[u8; 1]> for Tag {
    fn from([value]: [u8; 1]) -> Self {
        value.into()
    }
}

impl From<[u8; 2]> for Tag {
    fn from(value: [u8; 2]) -> Self {
        Self::from_2(value)
    }
}

impl From<[u8; 3]> for Tag {
    fn from(value: [u8; 3]) -> Self {
        Self::from_3(value)
    }
}

impl Tag {
    pub const fn as_slice(&self) -> &[u8] {
        let [b1, b2, b3] = self.0;
        if b1 == 0 {
            if b2 == 0 {
                debug_assert!(b3 & 0b11111 != 0b11111, "Invalid encoding for 1 byte tag");
                // indexing with a range is not yet `const`
                self.0.split_at(2).1
            } else {
                debug_assert!(
                    b2 & 0b11111 == 0b11111,
                    "Invalid encoding for first byte of tag"
                );
                debug_assert!(
                    0x1F <= b3 && b3 <= 0x7F,
                    "Invalid encoding for first byte of tag"
                );
                self.0.split_at(1).1
            }
        } else {
            debug_assert!(
                b1 & 0b11111 == 0b11111,
                "Invalid encoding for first byte of tag"
            );
            debug_assert!(b2 > 0x80);
            debug_assert!(b3 <= 0x7F);
            &self.0
        }
    }

    pub fn serialize(&self) -> heapless::Vec<u8, 3> {
        heapless::Vec::try_from(self.as_slice()).unwrap()
    }
}

pub fn get_data_object<'input>(tag_path: &[Tag], data: &'input [u8]) -> Option<&'input [u8]> {
    let mut to_ret = data;
    let mut remainder = data;
    for tag in tag_path {
        loop {
            let (cur_tag, cur_value, cur_remainder) = take_data_object(remainder)?;
            remainder = cur_remainder;
            if *tag == cur_tag {
                to_ret = cur_value;
                remainder = cur_value;
                break;
            }
        }
    }
    Some(to_ret)
}

/// Returns (tag, data, remainder)
pub fn take_data_object(data: &[u8]) -> Option<(Tag, &[u8], &[u8])> {
    let (tag, remainder) = take_tag(data)?;
    let (len, remainder) = take_len(remainder)?;
    if remainder.len() < len {
        None
    } else {
        let (value, remainder) = remainder.split_at(len);
        Some((tag, value, remainder))
    }
}

// See
// https://www.emvco.com/wp-content/uploads/2017/05/EMV_v4.3_Book_3_Application_Specification_20120607062110791.pdf
// Annex B1
pub fn take_tag(data: &[u8]) -> Option<(Tag, &[u8])> {
    let b1 = *data.first()?;
    if (b1 & 0x1f) == 0x1f {
        let b2 = *data.get(1)?;
        if b2 & 0x80 == 0x80 {
            // three bytes
            let b3 = *data.get(2)?;
            if b3 & 0x80 == 0x80 {
                None
            } else {
                Some((Tag([b1, b2, b3]), &data[3..]))
            }
        } else {
            Some((Tag([0, b1, b2]), &data[2..]))
        }
    } else {
        Some((Tag([0, 0, b1]), &data[1..]))
    }
}

pub fn take_len(data: &[u8]) -> Option<(usize, &[u8])> {
    let l1 = *data.first()?;
    if l1 <= 0x7F {
        Some((l1 as usize, &data[1..]))
    } else if l1 == 0x81 {
        Some((*data.get(1)? as usize, &data[2..]))
    } else {
        if l1 != 0x82 {
            return None;
        }
        let l2 = *data.get(1)?;
        let l3 = *data.get(2)?;
        let len = u16::from_be_bytes([l2, l3]) as usize;
        Some((len, &data[3..]))
    }
}

fn serialize_len(len: usize) -> Option<heapless::Vec<u8, 3>> {
    let mut buf = heapless::Vec::new();
    if let Ok(len) = u8::try_from(len) {
        if len <= 0x7f {
            buf.extend_from_slice(&[len]).ok();
        } else {
            buf.extend_from_slice(&[0x81, len]).ok();
        }
    } else if let Ok(len) = u16::try_from(len) {
        let [ar1, ar2] = len.to_be_bytes();
        buf.extend_from_slice(&[0x82, ar1, ar2]).ok();
    } else {
        return None;
    }
    Some(buf)
}

#[derive(Debug)]
pub struct Tlv<S> {
    tag: Tag,
    data: S,
}

impl<S> Tlv<S> {
    pub fn new(tag: Tag, data: S) -> Self {
        Self { tag, data }
    }
}

impl<S: DataSource> DataSource for Tlv<S> {
    fn len(&self) -> usize {
        let tag = self.tag.serialize();
        let len = serialize_len(self.data.len())
            .map(|l| l.len())
            .unwrap_or_default();
        tag.len() + len + self.data.len()
    }

    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer, S: DataStream<W>> DataStream<W> for Tlv<S> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        writer.write_all(&self.tag.serialize())?;
        writer.write_all(
            &serialize_len(self.data.len()).ok_or_else(|| {
                W::Error::failed_serialization("Data is longer than 0xFFFF bytes")
            })?,
        )?;
        self.data.to_writer(writer)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[test]
    fn dos() {
        assert_eq!(
            get_data_object(&[0x02u16].map(Tag::from), &hex!("02 02 1DB9 02 02 1DB9")),
            Some(hex!("1DB9").as_slice())
        );
        assert_eq!(
            get_data_object(&[0xA6u16, 0x7F49, 0x86].map(Tag::from), &hex!("A6 26 7F49 23 86 21 04 2525252525252525252525252525252525252525252525252525252525252525")),
            Some(hex!("04 2525252525252525252525252525252525252525252525252525252525252525").as_slice())
        );

        // Multiple nested
        assert_eq!(
            get_data_object(&[0xA6u16, 0x7F49, 0x86].map(Tag::from), &hex!("A6 2A 02 02 DEAD 7F49 23 86 21 04 2525252525252525252525252525252525252525252525252525252525252525")),
            Some(hex!("04 2525252525252525252525252525252525252525252525252525252525252525").as_slice())
        );

        // Multiple nested
        assert_eq!(
            get_data_object(&[1u16, 2].map(Tag::from), &hex!("01 04 01 00 02 00")),
            Some(hex!("").as_slice())
        );
    }

    #[test]
    fn tag_serialize() {
        let values = [
            (0xA6, hex!("A6").as_slice()),
            (0x7F49, &hex!("7F49")),
            (0x86, &hex!("86")),
            (0x4D, &hex!("4D")),
            (0x7F48, &hex!("7F48")),
            (0x5F48, &hex!("5F48")),
            (0x60, &hex!("60")),
            (0x5C, &hex!("5C")),
            (0xA4, &hex!("A4")),
            (0xB6, &hex!("B6")),
            (0x007C, &hex!("7C")),
            (0x80, &hex!("80")),
            (0x81, &hex!("81")),
            (0x82, &hex!("82")),
            (0x85, &hex!("85")),
            (0xAC, &hex!("AC")),
            (0x01, &hex!("01")),
            (0x02, &hex!("02")),
            (0x03, &hex!("03")),
        ];

        for (tag, serialized) in values {
            assert_eq!(Tag::from_u16(tag).serialize(), serialized);
            assert_eq!(Tag::from_u16(tag).as_slice(), serialized);
        }
    }

    #[test]
    fn tlv() {
        let mut buf = [0u8; 4];
        Tlv::new(Tag::from_u8(0x41), hex!("012A"))
            .to_writer(&mut buf.as_mut_slice())
            .unwrap();
        assert_eq!(buf.as_slice(), &hex!("41 02 012A"))
    }

    #[derive(Debug)]
    enum TlvProp {
        Leaf(Vec<u8>),
        Node(Vec<Tlv<Self>>),
    }

    impl DataSource for TlvProp {
        fn len(&self) -> usize {
            match self {
                Self::Leaf(v) => v.len(),
                Self::Node(v) => DataSource::len(&**v),
            }
        }
    }

    impl<W: Writer> DataStream<W> for TlvProp {
        fn to_writer(&self, writer: &mut W) -> Result<(), W::Error> {
            match self {
                Self::Leaf(v) => writer.write_all(v),
                Self::Node(t) => t.iter().try_for_each(|inner| inner.to_writer(writer)),
            }
        }
    }

    fn valid_tlv_tag() -> impl Strategy<Value = Tag> {
        (0..=0b11u8, any::<bool>(), 1..=16383u16).prop_filter_map(
            "invalid bytes",
            |(class, constructed, value)| {
                let [_, b2] = value.to_be_bytes();
                let class = class << 6;
                let constructed = (constructed as u8) << 5;
                let long = 0b11111;
                if value < 31 {
                    return Some(Tag::from_u8(class | constructed | b2));
                }

                if value < 128 {
                    return Some(Tag::from_2([class | constructed | long, b2]));
                }

                Some(Tag::from_3([
                    class | constructed | long,
                    0b10000000 | ((value & 0b11111111111111) >> 7) as u8,
                    (value & 0b01111111) as u8,
                ]))
            },
        )
    }

    fn tlv_strat() -> impl Strategy<Value = TlvProp> {
        let leaf = any::<Vec<u8>>().prop_map(TlvProp::Leaf);
        leaf.prop_recursive(3, 20, 3, |inner| {
            prop::collection::vec(
                (valid_tlv_tag(), inner).prop_map(|(tag, data)| Tlv { tag, data }),
                0..10,
            )
            .prop_filter("Duplicate tag", |v| {
                let tags: BTreeSet<_> = v.iter().map(|v| v.tag).collect();
                tags.len() == v.len()
            })
            .prop_map(TlvProp::Node)
        })
    }

    proptest!(
        #[test]
        fn tlv_round_trip(tlv in tlv_strat()) {
            let len = tlv.len();
            let mut buffer = vec![0u8;len];
            let mut writer= &mut *buffer;
            tlv.to_writer(&mut writer).unwrap();
            assert_eq!(writer.len(),0);


            println!("=====");
            fn recursive_tag_fetch(buffer: &[u8], acc: &mut Vec<Tag>, node: &TlvProp) {
                println!("{buffer:02x?}, {acc:02x?} {node:?}");
                match node {
                    TlvProp::Leaf(v) if acc.is_empty() => {},
                    TlvProp::Leaf(v) => assert_eq!(get_data_object(acc, buffer), Some(&**v), "{buffer:02x?} {acc:02x?} {v:02x?}"),
                    TlvProp::Node(tlv) => for inner  in tlv {
                        acc.push(inner.tag);
                        recursive_tag_fetch(get_data_object(acc, buffer).unwrap(), &mut Vec::new(), &inner.data);
                        recursive_tag_fetch(buffer, acc, &inner.data);
                        acc.pop();
                    }
                }
            }
            recursive_tag_fetch(&buffer, &mut Vec::new(), &tlv);
        }
    );
}
