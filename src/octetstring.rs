use super::tag::Tag;
use super::traits::{Asn1Object, Asn1InstanciableObject, Asn1Tagged};
use super::error::Asn1Error;
use std::result::Result;

pub static OCTET_STRING_TAG_NUMBER: u8 = 0x4;

#[derive(Debug, PartialEq)]
pub struct OctetString {
    tag: Tag,
    value: Vec<u8>
}

impl Asn1Tagged for OctetString {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(OCTET_STRING_TAG_NUMBER);
    }
}

impl Asn1Object for OctetString {

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
        let mut encoded_value = Vec::with_capacity(self.value.len());

        for i in 0..self.value.len() {
            encoded_value.push(self.value[i])
        }

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        self.value = raw.to_vec();
        return Ok(());
    }

    fn unset_value(&mut self) {
    }
}

impl Asn1InstanciableObject for OctetString {

    fn new_default() -> OctetString {
        return OctetString::new(vec![]);
    }

}

impl OctetString {

    pub fn new(value: Vec<u8>) -> OctetString {
        return OctetString {
            tag: OctetString::type_tag(),
            value
        }
    }

    pub fn value(&self) -> &[u8] {
        return &self.value;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_octet_string() {
        assert_eq!(vec![0x4, 0x1, 0x0], OctetString::new(vec![0x0]).encode().unwrap());
        assert_eq!(vec![0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], 
        OctetString::new(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).encode().unwrap());
        assert_eq!(vec![0x4, 0x0], OctetString::new(vec![]).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(OctetString::new(vec![0x0]), _parse_octet_string(&[0x4, 0x1, 0x0]));
        assert_eq!(OctetString::new(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
        _parse_octet_string(&[0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]));
        assert_eq!(OctetString::new(vec![]), _parse_octet_string(&[0x4, 0x0]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((OctetString::new(vec![0x0]), 3), _parse_octet_string_with_consumed_octets(&[0x4, 0x1, 0x0,
        0x01, 0x02, 0x03, 0x04]));
        assert_eq!((OctetString::new(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), 0xa),
        _parse_octet_string_with_consumed_octets(&[0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 
        0x01, 0x02, 0x03, 0x04]));
        assert_eq!((OctetString::new(vec![]), 2), _parse_octet_string_with_consumed_octets(&[0x4, 0x0,
        0x01, 0x02, 0x03, 0x04]));
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse_octet_string(&[0x7, 0x1, 0x0]);
    }

    fn _parse_octet_string(raw: &[u8]) -> OctetString {
        return _parse_octet_string_with_consumed_octets(raw).0;
    }

    fn _parse_octet_string_with_consumed_octets(raw: &[u8]) -> (OctetString, usize) {
        let mut os = OctetString::new(vec![]);
        let consumed_octets = os.decode(raw).unwrap();
        return (os, consumed_octets);
    }
}
