use super::tag::Tag;
use super::traits::{Asn1Object, Asn1Tagged};
use super::error::*;
use std::result::Result;

pub static BOOLEAN_TAG_NUMBER: u8 = 0x1;

#[derive(Debug, PartialEq)]
pub struct Boolean {
    tag: Tag,
    value: bool
}

impl Boolean {

    pub fn new(value: bool) -> Boolean {
        return Boolean {
            tag: Boolean::type_tag(),
            value
        }
    }
}

impl Asn1Tagged for Boolean {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(BOOLEAN_TAG_NUMBER);
    }
}

impl Asn1Object for Boolean {

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Asn1Result<Vec<u8>> {
        return Ok(vec![(self.value as u8) * 0xff]);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Asn1Result<()> {
        if raw.len() == 0 {
            return Err(Asn1ErrorKind::NoDataForType)?;
        }

        self.value = raw[0] != 0;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x1, 0x1, 0x0], Boolean::new(false).encode().unwrap());
        assert_eq!(vec![0x1, 0x1, 0xff], Boolean::new(true).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(Boolean::new(false), _parse(&[0x1, 0x1, 0x0]));
        assert_eq!(Boolean::new(true), _parse(&[0x1, 0x1, 0xff]));
        assert_eq!(Boolean::new(true), _parse(&[0x1, 0x1, 0x01]));
        assert_eq!(Boolean::new(true), _parse(&[0x1, 0x1, 0x7b]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((Boolean::new(false), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x0, 0x1]));
        assert_eq!((Boolean::new(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0xff, 0x0, 0x1, 0x2]));
        assert_eq!((Boolean::new(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x01, 0x0, 0x1]));
        assert_eq!((Boolean::new(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x7b, 0x0]));

        assert_eq!((Boolean::new(false), 4), _parse_with_consumed_octets(&[0x1, 0x2, 0x0, 0x1]));
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic (expected = "Invalid value: Not enough data for type")]
    #[test]
    fn test_decode_without_enough_value_octets() {
        _parse(&[0x1, 0x0]);
    }

    fn _parse(raw: &[u8]) -> Boolean {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (Boolean, usize) {
        let mut b = Boolean::new(false);
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
