use ascii::AsciiString;
use ascii::AsciiChar;
use super::tag::Tag;
use super::traits::{Asn1Object, Asn1Tagged};
use super::error::*;
use std::result::Result;

pub static IA5STRING_TAG_NUMBER: u8 = 0x16;

#[derive(Debug, PartialEq)]
pub struct IA5String {
    tag: Tag,
    value: AsciiString
}

impl Asn1Tagged for IA5String {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(IA5STRING_TAG_NUMBER);
    }
}

impl Asn1Object for IA5String {
    
    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
        let mut encoded_value : Vec<u8> = Vec::with_capacity(self.value.len());

        for &c in self.value.chars() {
            encoded_value.push(c as u8);
        }

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        let mut value = AsciiString::with_capacity(raw.len());

        for byte in raw.iter() {
            let ascii_char;
            
            match AsciiChar::from(*byte as char) {
                Ok(value) => {
                    ascii_char = value;
                },
                Err(_) => {
                    return Err(Asn1ErrorKind::InvalidValue("Error formating non-ascii characters".to_string()))?;
                }
            };
            value.push(ascii_char);
        }

        self.value = value;

        return Ok(());
    }
}

impl IA5String {
    pub fn new(value: AsciiString) -> IA5String {
        return IA5String {
            tag: IA5String::type_tag(),
            value
        }
    }

    pub fn new_empty() -> IA5String {
        return IA5String {
            tag: IA5String::type_tag(),
            value: AsciiString::from_ascii("").unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_ia5string() {
        assert_eq!(vec![0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d],
        IA5String::new(AsciiString::from_ascii("test1@rsa.com").unwrap()).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(IA5String::new(AsciiString::from_ascii("test1@rsa.com").unwrap()),
        _parse(&[0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d]));
    }

    #[test]
    fn test_decode_empty_value() {
        assert_eq!(IA5String::new(AsciiString::from_ascii("").unwrap()),
        _parse(&[0x16, 0x00]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((IA5String::new(AsciiString::from_ascii("test1@rsa.com").unwrap()), 15),
        _parse_with_consumed_octets(&[0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73, 0x61, 0x2e, 0x63, 0x6f, 0x6d, 
                                        0x22, 0x22, 0x22]));
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
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
        let mut b = IA5String::new(AsciiString::from_ascii("".to_string()).unwrap());
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
