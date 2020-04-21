use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;

pub static OCTET_STRING_TAG_NUMBER: u8 = 0x4;


/// Class to encode/decode OctetString ASN1
pub type OctetString = Vec<u8>;

impl Asn1Object for OctetString {

    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(OCTET_STRING_TAG_NUMBER);
    }

    fn encode_value(&self) -> Vec<u8> {
        return self.clone();
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        *self = raw.to_vec();
        return Ok(());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_octet_string() {
        assert_eq!(vec![0x4, 0x1, 0x0], OctetString::from(vec![0x0]).encode());
        assert_eq!(vec![0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], 
        OctetString::from(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).encode());
        assert_eq!(vec![0x4, 0x0], OctetString::from(vec![]).encode());
    }

    #[test]
    fn test_decode() {
        assert_eq!(OctetString::from(vec![0x0]), _parse_octet_string(&[0x4, 0x1, 0x0]));
        assert_eq!(OctetString::from(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
        _parse_octet_string(&[0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]));
        assert_eq!(OctetString::from(vec![]), _parse_octet_string(&[0x4, 0x0]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((OctetString::from(vec![0x0]), 3), _parse_octet_string_with_consumed_octets(&[0x4, 0x1, 0x0,
        0x01, 0x02, 0x03, 0x04]));
        assert_eq!((OctetString::from(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]), 0xa),
        _parse_octet_string_with_consumed_octets(&[0x04, 0x08, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 
        0x01, 0x02, 0x03, 0x04]));
        assert_eq!((OctetString::from(vec![]), 2), _parse_octet_string_with_consumed_octets(&[0x4, 0x0,
        0x01, 0x02, 0x03, 0x04]));
    }

    #[should_panic (expected = "UnmatchedTag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse_octet_string(&[0x7, 0x1, 0x0]);
    }

    fn _parse_octet_string(raw: &[u8]) -> OctetString {
        return _parse_octet_string_with_consumed_octets(raw).0;
    }

    fn _parse_octet_string_with_consumed_octets(raw: &[u8]) -> (OctetString, usize) {
        let (consumed_octets, os) = OctetString::decode(raw).unwrap();
        return (os, consumed_octets);
    }
}
