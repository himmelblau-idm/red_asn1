use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;

pub static BOOLEAN_TAG_NUMBER: u8 = 0x1;

/// Class to encode/decode Boolean ASN1
pub type Boolean = bool;

impl Asn1Object for Boolean {

    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(BOOLEAN_TAG_NUMBER);
    }

    fn encode_value(&self) -> Vec<u8> {
        return vec![(*self as u8) * 0xff];
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::Error::NoDataForType)?;
        }

        *self = raw[0] != 0;
        return Ok(());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x1, 0x1, 0x0], false.encode());
        assert_eq!(vec![0x1, 0x1, 0xff], true.encode());
    }

    #[test]
    fn test_decode() {
        assert_eq!(false, _parse(&[0x1, 0x1, 0x0]));
        assert_eq!(true, _parse(&[0x1, 0x1, 0xff]));
        assert_eq!(true, _parse(&[0x1, 0x1, 0x01]));
        assert_eq!(true, _parse(&[0x1, 0x1, 0x7b]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((false, 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x0, 0x1]));
        assert_eq!((true, 3), _parse_with_consumed_octets(&[0x1, 0x1, 0xff, 0x0, 0x1, 0x2]));
        assert_eq!((true, 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x01, 0x0, 0x1]));
        assert_eq!((true, 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x7b, 0x0]));

        assert_eq!((false, 4), _parse_with_consumed_octets(&[0x1, 0x2, 0x0, 0x1]));
    }

    #[should_panic (expected = "UnmatchedTag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic (expected = "NoDataForType")]
    #[test]
    fn test_decode_without_enough_value_octets() {
        _parse(&[0x1, 0x0]);
    }

    fn _parse(raw: &[u8]) -> Boolean {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (Boolean, usize) {
        let (consumed_octets, b) = Boolean::decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
