use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;

pub static GENERALSTRING_TAG_NUMBER: u8 = 0x1b;

/// Class to encode/decode GeneralString ASN1
#[derive(Debug, PartialEq, Default)]
pub struct GeneralString {
    _value: Option<String>
}


impl GeneralString {
    pub fn new(value: String) -> Self {
        return Self {
            _value: Some(value)
        }
    }

    pub fn value(&self) -> Option<&String> {
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

impl Asn1Object for GeneralString {
    
    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(GENERALSTRING_TAG_NUMBER);
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

        return Ok(value.as_bytes().to_vec());
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let value = String::from_utf8(raw.to_vec())?;

        self._value = Some(value);

        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = GeneralString::new("test1@rsa.com".to_string());
        assert_eq!("test1@rsa.com", b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        assert_eq!(
            GeneralString {
                _value: None
            },
            GeneralString::default()
        )
    }

    #[test]
    fn test_unset_value() {
        let mut b = GeneralString::new("test1@rsa.com".to_string());
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x1b, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d],
        GeneralString::new("test1@rsa.com".to_string()).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(GeneralString::new("test1@rsa.com".to_string()),
        _parse(&[0x1b, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d]));
    }

    #[test]
    fn test_decode_empty_value() {
        assert_eq!(GeneralString::new("".to_string()),
        _parse(&[0x1b, 0x00]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((GeneralString::new("test1@rsa.com".to_string()), 15),
        _parse_with_consumed_octets(&[0x1b, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d, 
                                        0x22, 0x22, 0x22]));
    }

    #[should_panic (expected = "Invalid value: Error formating non-utf8 characters")]
    #[test]
    fn test_decode_non_ascii_characters() {
        _parse(&[0x1b, 0x1, 0xff]);
    }

    #[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    fn _parse(raw: &[u8]) -> GeneralString {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (GeneralString, usize) {
        let mut b = GeneralString::new("".to_string());
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
