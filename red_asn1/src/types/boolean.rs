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

    pub fn value(&self) -> Option<bool> {
        match &self._value {
            Some(ref value) => {
                return Some(*value);
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
                return Err(asn1err::Error::NoValue)?;
            }
        }
        
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::Error::NoDataForType)?;
        }

        self._value = Some(raw[0] != 0);
        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        return Self {
            _value: Some(value)
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = Boolean::from(true);
        assert_eq!(true, b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        let b = Boolean::default();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_unset_value() {
        let mut b = Boolean::from(true);
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x1, 0x1, 0x0], Boolean::from(false).encode().unwrap());
        assert_eq!(vec![0x1, 0x1, 0xff], Boolean::from(true).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(Boolean::from(false), _parse(&[0x1, 0x1, 0x0]));
        assert_eq!(Boolean::from(true), _parse(&[0x1, 0x1, 0xff]));
        assert_eq!(Boolean::from(true), _parse(&[0x1, 0x1, 0x01]));
        assert_eq!(Boolean::from(true), _parse(&[0x1, 0x1, 0x7b]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((Boolean::from(false), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x0, 0x1]));
        assert_eq!((Boolean::from(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0xff, 0x0, 0x1, 0x2]));
        assert_eq!((Boolean::from(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x01, 0x0, 0x1]));
        assert_eq!((Boolean::from(true), 3), _parse_with_consumed_octets(&[0x1, 0x1, 0x7b, 0x0]));

        assert_eq!((Boolean::from(false), 4), _parse_with_consumed_octets(&[0x1, 0x2, 0x0, 0x1]));
    }

    #[should_panic (expected = "InvalidTag(Unmatched")]
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
        let mut b = Boolean::from(false);
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
