
use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;
use super::BitSringValue;

pub static BIT_STRING_TAG_NUMBER: u8 = 0x3;

/// Class to encode/decode BitSring ASN1
#[derive(Debug, PartialEq, Default)]
pub struct BitSring {
    _value: Option<BitSringValue>
}


impl BitSring {

    pub fn new(bytes: Vec<u8>, padding_length: u8) -> BitSring{
        let bs = BitSring {
            _value: Some(BitSringValue::new(bytes, padding_length % 8))
        };
        return bs;
    }

    pub fn value(&self) -> Option<&BitSringValue> {
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


impl Asn1Object for BitSring {

    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(BIT_STRING_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        let bitstring_value;

        match &self._value {
            Some(value) => {
                bitstring_value = value;
            },
            None => {
                return Err(asn1err::ErrorKind::NoValue)?;
            }
        };

        let mut encoded_value: Vec<u8> = vec![bitstring_value.get_padding_length()];

        let mut values: Vec<u8> = Vec::new();
        let bytes = bitstring_value.get_bytes();
        for i in 0..bytes.len() {
            values.push(bytes[i])
        }
        encoded_value.append(&mut values);

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::ValueErrorKind::NoDataForType)?;
        }

        let (padding_length, raw_value) = raw.split_at(1);

        self._value = Some(BitSringValue::new(raw_value.to_vec(), padding_length[0]));

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
        let b = BitSring::new(vec![0x0], 0);
        assert_eq!(&BitSringValue::new(vec![0x0], 0), b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        let b = BitSring::default();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_unset_value() {
        let mut b = BitSring::new(vec![0x0], 0);
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_encode_bit_string() {
        assert_eq!(vec![0x3, 0x2, 0x0, 0x0], BitSring::new(vec![0x0], 0).encode().unwrap());
        assert_eq!(vec![0x3, 0x4, 0x6, 0x6e, 0x5d, 0xC0], BitSring::new(vec![0x6e, 0x5d, 0xFF], 6).encode().unwrap());
        assert_eq!(vec![0x3, 0x2, 0x4, 0xF0], BitSring::new(vec![0xF0], 4).encode().unwrap());
        assert_eq!(vec![0x3, 0x1, 0x4], BitSring::new(vec![], 4).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(BitSring::new(vec![0x0], 0), _parse(&[0x3, 0x2, 0x0, 0x0]));
        assert_eq!(BitSring::new(vec![0x6e, 0x5d, 0xFF], 6), _parse(&[0x3, 0x4, 0x6, 0x6e, 0x5d, 0xFF]));
        assert_eq!(BitSring::new(vec![0xF0], 4), _parse(&[0x3, 0x2, 0x4, 0xF0]));
        assert_eq!(BitSring::new(vec![], 4), _parse(&[0x3, 0x1, 0x4]));
    }

    #[test]
    fn test_decode_boolean_with_excesive_bytes() {
        assert_eq!((BitSring::new(vec![0x0], 0), 4), 
                    _parse_with_consumed_octets(&[0x3, 0x2, 0x0, 0x0, 0x11, 0x22]));
        assert_eq!((BitSring::new(vec![0x6e, 0x5d, 0xFF], 6), 6), 
                    _parse_with_consumed_octets(&[0x3, 0x4, 0x6, 0x6e, 0x5d, 0xFF, 0x11, 0x22]));
        assert_eq!((BitSring::new(vec![0xF0], 4), 4),
                    _parse_with_consumed_octets(&[0x3, 0x2, 0x4, 0xF0, 0x11, 0x22]));
        assert_eq!((BitSring::new(vec![], 4), 3),
                    _parse_with_consumed_octets(&[0x3, 0x1, 0x4, 0x11, 0x22]));
    }

    #[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
    #[test]
    fn test_decode_boolean_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic (expected = "Invalid value: Not enough data for type")]
    #[test]
    fn test_decode_boolean_without_enough_value_octets() {
        _parse(&[0x3, 0x0]);
    }

    fn _parse(raw: &[u8]) -> BitSring {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (BitSring, usize) {
        let mut b = BitSring::new(vec![], 0);
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }


    #[test]
    fn test_value_get_bytes() {
        let b = BitSring::new(vec![0x0, 0x1, 0x2, 0x3], 0);
        assert_eq!(&vec![0x0, 0x1, 0x2, 0x3], b.value().unwrap().get_bytes());
    }

    #[test]
    fn test_value_padding_length() {
        let b = BitSring::new(vec![0x0, 0x1, 0x2, 0x3], 7);
        assert_eq!(7, b.value().unwrap().get_padding_length());
    }
}
