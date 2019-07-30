use crate::tag::Tag;
use crate::traits::{Asn1Object, Asn1InstanciableObject, Asn1Tagged};
use crate::error::*;

pub static OCTET_STRING_TAG_NUMBER: u8 = 0x4;

#[derive(Debug, PartialEq)]
pub struct OctetString {
    tag: Tag,
    _value: Option<Vec<u8>>
}

impl Asn1Tagged for OctetString {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(OCTET_STRING_TAG_NUMBER);
    }
}

impl Asn1Object for OctetString {

    fn tag(&self) -> Tag {
        return self.tag.clone();
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
        let value;

        match &self._value {
            Some(_value) => {
                value = _value;
            },
            None => {
                return Err(Asn1ErrorKind::NoValue)?;
            }
        }

        let mut encoded_value = Vec::with_capacity(value.len());

        for i in 0..value.len() {
            encoded_value.push(value[i])
        }

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        self._value = Some(raw.to_vec());
        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

impl Asn1InstanciableObject for OctetString {

    fn new_default() -> OctetString {
        return OctetString::new_empty();
    }

}

impl OctetString {

    pub fn new(value: Vec<u8>) -> OctetString {
        return OctetString {
            tag: OctetString::type_tag(),
            _value: Some(value)
        }
    }

    pub fn new_empty() -> OctetString {
        return OctetString {
            tag: OctetString::type_tag(),
            _value: None
        }
    }

    pub fn value(&self) -> Option<&Vec<u8>> {
        match &self._value {
            Some(ref value) => {
                return Some(value);
            }
            None => {
                return None;
            }
        };
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = OctetString::new(vec![0x0]);
        assert_eq!(&vec![0x0], b.value().unwrap());
    }

    #[test]
    fn test_create_empty() {
        let b = OctetString::new_empty();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_create_default() {
        let b = OctetString::new_default();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_unset_value() {
        let mut b = OctetString::new(vec![0x0]);
        b.unset_value();
        assert_eq!(None, b.value());
    }

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

    #[should_panic (expected = "Invalid type tag: Not match with expected tag")]
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
