use ascii::{AsciiChar, AsciiString};
use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;

pub static IA5STRING_TAG_NUMBER: u8 = 0x16;

/// Class to encode/decode IA5String ASN1
#[derive(Debug, PartialEq, Default)]
pub struct IA5String {
    _value: Option<AsciiString>
}

impl Asn1Object for IA5String {
    
    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(IA5STRING_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        let value;

        match &self._value {
            Some(_value) => {
                value = _value;
            },
            None => {
                return Err(asn1err::ErrorKind::NoValue)?;
            }
        }

        let mut encoded_value : Vec<u8> = Vec::with_capacity(value.len());

        for c in value.chars() {
            encoded_value.push(c as u8);
        }

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let mut value = AsciiString::with_capacity(raw.len());

        for byte in raw.iter() {
            value.push(AsciiChar::from_ascii(*byte)?);
        }

        self._value = Some(value);

        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

impl IA5String {

    pub fn value(&self) -> Option<&AsciiString> {
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

impl From<AsciiString> for IA5String {
    fn from(string: AsciiString) -> Self {
        return IA5String {
            _value: Some(string)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap());
        assert_eq!(&AsciiString::from_ascii("test1@rsa.com").unwrap(), b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        assert_eq!(
            IA5String {
                _value: None
            },
            IA5String::default()
        )
    }

    #[test]
    fn test_unset_value() {
        let mut b = IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap());
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_encode_ia5string() {
        assert_eq!(vec![0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d],
        IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap()).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap()),
        _parse(&[0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d]));
    }

    #[test]
    fn test_decode_empty_value() {
        assert_eq!(IA5String::from(AsciiString::from_ascii("").unwrap()),
        _parse(&[0x16, 0x00]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap()), 15),
        _parse_with_consumed_octets(&[0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d, 
                                        0x22, 0x22, 0x22]));
    }

    #[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic (expected = "Invalid value: Error formating non-ascii characters")]
    #[test]
    fn test_decode_non_ascii_characters() {
        _parse(&[0x16, 0x1, 0x80]);
    }

    fn _parse(raw: &[u8]) -> IA5String {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (IA5String, usize) {
        let mut b = IA5String::from(AsciiString::from_ascii("".to_string()).unwrap());
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
