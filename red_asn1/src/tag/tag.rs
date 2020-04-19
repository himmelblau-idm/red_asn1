use crate::error as asn1err;
use super::{TagClass, TagType};

/// Class to represent DER-ASN1 tags of the different types.
/// 
/// Each tag is divided into 3 parts:
/// * Class: If tag is of an Primitive or Constructed object
/// * Type: The scope of the object
/// * Number: A distinguished number between objects of the same type and class
/// 
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tag {
    tag_number: u8,
    tag_type: TagType,
    tag_class: TagClass
}

impl Tag {

    /// Creates a new tag from a given number, type and class
    pub fn new(tag_number: u8, tag_type: TagType, tag_class: TagClass) -> Tag {
        return Tag{tag_number, tag_type, tag_class};
    }

    /// Shorcut of: `Tag::new(tag_number, TagType::Primitive, TagClass::Universal)`
    pub fn new_primitive_universal(tag_number: u8) -> Tag {
        return Tag::new(tag_number, TagType::Primitive, TagClass::Universal);
    }

    /// Shorcut of: `Tag::new(tag_number, TagType::Constructed, TagClass::Universal)`
    pub fn new_constructed_universal(tag_number: u8) -> Tag {
        return Tag::new(tag_number, TagType::Constructed, TagClass::Universal);
    }


    /// Produces an DER version of the tag in bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded_tag: u8 = 0;

        encoded_tag += self.tag_class as u8;
        encoded_tag <<= 1;

        encoded_tag += self.tag_type as u8;
        encoded_tag <<= 5;

        if self.tag_number <= 30 {
            encoded_tag += self.tag_number;
            return vec![encoded_tag];
        }
        encoded_tag |= 0b11111;

        let mut encoded_tags = vec![encoded_tag];

        let mut next_octet = self.tag_number;

        while next_octet > 127 {
            encoded_tags.push(next_octet | 0b10000000);
            next_octet >>= 7;
        }
        encoded_tags.push(next_octet);

        return encoded_tags;
    }

    pub fn set_number(&mut self, tag_number: u8) {
        self.tag_number = tag_number;
    }

    pub fn set_class(&mut self, tag_class: TagClass) {
        self.tag_class = tag_class;
    }

    /// Set the Tag values from a array of bytes
    pub fn decode(&mut self, raw: &[u8]) -> asn1err::Result<usize> {
        let raw_len = raw.len();
        if raw_len == 0 {
            return Err(asn1err::Error::EmptyTag(TagClass::Universal))?;
        }

        let mut consumed_octets = 1;
        let octet = raw[0];

        let tag_class = (octet & 0xc0) >> 6;
        let tag_type = (octet & 0x20) >> 5;
        let mut tag_number = octet & 0x1f;

        if tag_number == 0x1f {
            let (tag_number_long_form, octets_consumed_long_form) = self._decode_high_tag_number(raw)?;
            consumed_octets += octets_consumed_long_form;
            tag_number = tag_number_long_form;
        }

        self.tag_type = TagType::from(tag_type);
        self.tag_class = TagClass::from(tag_class);
        self.tag_number = tag_number;

        return Ok(consumed_octets);
    }

    fn _decode_high_tag_number(&self, raw: &[u8]) -> asn1err::Result<(u8, usize)> {
        let mut consumed_octets = 1;
        let mut tag_number: u8 = 0;
        while consumed_octets < raw.len() {
            let next_octet = raw[consumed_octets];
            tag_number += (next_octet & 0b01111111) << (7 * (consumed_octets - 1));
            if next_octet & 0b10000000 == 0 {
                break;
            }
            consumed_octets += 1;
        }
        if consumed_octets == raw.len() {
            return Err(asn1err::Error::NotEnoughTagOctets(TagClass::Universal))?;
        }

        return Ok((tag_number,consumed_octets));
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_tag() {
        assert_eq!(vec![0x00], Tag::new(0, TagType::Primitive, TagClass::Universal).encode());
        assert_eq!(vec![0x40], Tag::new(0, TagType::Primitive, TagClass::Application).encode());
        assert_eq!(vec![0x80], Tag::new(0, TagType::Primitive, TagClass::Context).encode());
        assert_eq!(vec![0xc0], Tag::new(0, TagType::Primitive, TagClass::Private).encode());
        assert_eq!(vec![0x20], Tag::new(0, TagType::Constructed, TagClass::Universal).encode());
        assert_eq!(vec![0x60], Tag::new(0, TagType::Constructed, TagClass::Application).encode());
        assert_eq!(vec![0xa0], Tag::new(0, TagType::Constructed, TagClass::Context).encode());
        assert_eq!(vec![0xe0], Tag::new(0, TagType::Constructed, TagClass::Private).encode());

        assert_eq!(vec![0x1E], Tag::new(30, TagType::Primitive, TagClass::Universal).encode());
        assert_eq!(vec![0x1F, 0x1F], Tag::new(31, TagType::Primitive, TagClass::Universal).encode());
        assert_eq!(vec![0x1F, 0x7F], Tag::new(127, TagType::Primitive, TagClass::Universal).encode());
        assert_eq!(vec![0x1F, 0x80, 0x01], Tag::new(128, TagType::Primitive, TagClass::Universal).encode());
        assert_eq!(vec![0x1F, 0xFF, 0x01], Tag::new(255, TagType::Primitive, TagClass::Universal).encode());

        assert_eq!(vec![0xdf, 0xc6, 0x01], Tag::new(198, TagType::Primitive, TagClass::Private).encode());
        assert_eq!(vec![0xff, 0x6a], Tag::new(106, TagType::Constructed, TagClass::Private).encode());
        assert_eq!(vec![0x3f, 0x39], Tag::new(57, TagType::Constructed, TagClass::Universal).encode());
        assert_eq!(vec![0xbf, 0x24], Tag::new(36, TagType::Constructed, TagClass::Context).encode());
        assert_eq!(vec![0xf4], Tag::new(20, TagType::Constructed, TagClass::Private).encode());
        assert_eq!(vec![0x6b], Tag::new(11, TagType::Constructed, TagClass::Application).encode());
    }

    #[test]
    fn test_decode_tag() {
        assert_eq!(Tag::new(0, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x00]));
        assert_eq!(Tag::new(0, TagType::Primitive, TagClass::Application), _parse_tag(vec![0x40]));
        assert_eq!(Tag::new(0, TagType::Primitive, TagClass::Context), _parse_tag(vec![0x80]));
        assert_eq!(Tag::new(0, TagType::Primitive, TagClass::Private), _parse_tag(vec![0xc0]));
        assert_eq!(Tag::new(0, TagType::Constructed, TagClass::Universal), _parse_tag(vec![0x20]));
        assert_eq!(Tag::new(0, TagType::Constructed, TagClass::Application), _parse_tag(vec![0x60]));
        assert_eq!(Tag::new(0, TagType::Constructed, TagClass::Context), _parse_tag(vec![0xa0]));
        assert_eq!(Tag::new(0, TagType::Constructed, TagClass::Private), _parse_tag(vec![0xe0]));

        assert_eq!(Tag::new(30, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x1E]));
        assert_eq!(Tag::new(31, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x1F, 0x1F]));
        assert_eq!(Tag::new(127, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x1F, 0x7F]));
        assert_eq!(Tag::new(128, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x1F, 0x80, 0x01]));
        assert_eq!(Tag::new(255, TagType::Primitive, TagClass::Universal), _parse_tag(vec![0x1F, 0xFF, 0x01]));

        assert_eq!(Tag::new(198, TagType::Primitive, TagClass::Private), _parse_tag(vec![0xdf, 0xc6, 0x01]));
        assert_eq!(Tag::new(106, TagType::Constructed, TagClass::Private), _parse_tag(vec![0xff, 0x6a]));
        assert_eq!(Tag::new(57, TagType::Constructed, TagClass::Universal), _parse_tag(vec![0x3f, 0x39]));
        assert_eq!(Tag::new(36, TagType::Constructed, TagClass::Context), _parse_tag(vec![0xbf, 0x24]));
        assert_eq!(Tag::new(20, TagType::Constructed, TagClass::Private), _parse_tag(vec![0xf4]));
        assert_eq!(Tag::new(11, TagType::Constructed, TagClass::Application), _parse_tag(vec![0x6b]));
    }

    #[test]
    fn test_decode_tag_with_excesive_bytes() {
        assert_eq!((Tag::new(0, TagType::Primitive, TagClass::Application), 1), _parse_tag_with_consumed_octets(vec![0x40, 0x01]));
        assert_eq!((Tag::new(31, TagType::Primitive, TagClass::Universal), 2), _parse_tag_with_consumed_octets(vec![0x1F, 0x1F, 0x01]));
        assert_eq!((Tag::new(198, TagType::Primitive, TagClass::Private), 3), _parse_tag_with_consumed_octets(vec![0xdf, 0xc6, 0x01, 0x01, 0x02]));
    }

    #[should_panic (expected = "EmptyTag")]
    #[test]
    fn test_decode_empty_tag() {
        _parse_tag(vec![]);
    }
    

    #[should_panic (expected = "NotEnoughTagOctets")]
    #[test]
    fn test_decode_invalid_tag_with_unfinished_tag_number() {
        _parse_tag(vec![0x1F, 0x80, 0x81]);
    }

    fn _parse_tag(raw: Vec<u8>) -> Tag {
        let mut tag = Tag::new(0, TagType::Primitive, TagClass::Universal);
        tag.decode(&raw).unwrap();
        return tag;
    }

    fn _parse_tag_with_consumed_octets(raw: Vec<u8>) -> (Tag, usize) {
        let mut tag = Tag::new(0, TagType::Primitive, TagClass::Universal);
        let consumed_octets = tag.decode(&raw).unwrap();
        return (tag, consumed_octets);
    }
}
