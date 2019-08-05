use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;

pub static BOOLEAN_TAG_NUMBER: u8 = 0x1;


/// Class to encode/decode Boolean ASN1
#[derive(Debug, PartialEq, Default)]
pub struct Boolean {
    _value: Option<bool>
}

impl Boolean {

    pub fn new(value: bool) -> Boolean {
        return Boolean {
            _value: Some(value)
        };
    }

    pub fn value(&self) -> Option<&bool> {
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

impl Asn1Object for Boolean {

    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(BOOLEAN_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        match self._value {
            Some(value) => {
                return Ok(vec![(value as u8) * 0xff]);
            },
            None => {
                return Err(asn1err::ErrorKind::NoValue)?;
            }
        }
        
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::ErrorKind::NoDataForType)?;
        }

        self._value = Some(raw[0] != 0);
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
        let b = Boolean::new(true);
        assert_eq!(&true, b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        let b = Boolean::default();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_unset_value() {
        let mut b = Boolean::new(true);
        b.unset_value();
        assert_eq!(None, b.value());
    }

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

    #[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
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
