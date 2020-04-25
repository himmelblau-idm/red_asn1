use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::*;
use ascii::{AsciiChar, AsciiString};

pub static IA5STRING_TAG_NUMBER: u8 = 0x16;

/// Class to build/parse IA5String ASN1
pub type IA5String = AsciiString;

impl Asn1Object for IA5String {
    fn tag() -> Tag {
        return Tag::new_primitive_universal(IA5STRING_TAG_NUMBER);
    }

    fn build_value(&self) -> Vec<u8> {
        let mut encoded_value: Vec<u8> = Vec::with_capacity(self.len());

        for ch in self.chars() {
            encoded_value.push(ch as u8);
        }

        return encoded_value;
    }

    fn parse_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let mut value = AsciiString::with_capacity(raw.len());

        for byte in raw.iter() {
            value.push(AsciiChar::from_ascii(*byte)?);
        }

        *self = value;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ia5string() {
        assert_eq!(
            vec![
                0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73,
                0x61, 0x2e, 0x63, 0x6f, 0x6d
            ],
            IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap())
                .build()
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            IA5String::from(AsciiString::from_ascii("test1@rsa.com").unwrap()),
            _parse(&[
                0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73,
                0x61, 0x2e, 0x63, 0x6f, 0x6d
            ])
        );
    }

    #[test]
    fn test_parse_empty_value() {
        assert_eq!(
            IA5String::from(AsciiString::from_ascii("").unwrap()),
            _parse(&[0x16, 0x00])
        );
    }

    #[test]
    fn test_parse_with_excesive_bytes() {
        assert_eq!(
            (
                IA5String::from(
                    AsciiString::from_ascii("test1@rsa.com").unwrap()
                ),
                15
            ),
            _parse_with_consumed_octets(&[
                0x16, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73,
                0x61, 0x2e, 0x63, 0x6f, 0x6d, 0x22, 0x22, 0x22
            ])
        );
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_parse_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic(expected = "AsciiError")]
    #[test]
    fn test_parse_non_ascii_characters() {
        _parse(&[0x16, 0x1, 0x80]);
    }

    fn _parse(raw: &[u8]) -> IA5String {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (IA5String, usize) {
        let (consumed_octets, b) = IA5String::parse(raw).unwrap();
        return (b, consumed_octets);
    }
}
