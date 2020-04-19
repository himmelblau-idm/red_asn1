use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::*;

pub static INTEGER_TAG_NUMBER: u8 = 0x2;

/// Class to encode/decode Integer ASN1
#[derive(Debug, PartialEq, Default)]
pub struct Integer {
    _value: Option<i64>,
}

impl Asn1Object for Integer {
    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(INTEGER_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        let mut shifted_value;

        match self._value {
            Some(value) => {
                shifted_value = value;
            }
            None => {
                return Err(asn1err::Error::NoValue)?;
            }
        }

        let length = self._encoded_value_size();

        let mut encoded_value: Vec<u8> = Vec::new();

        for _ in 0..length {
            encoded_value.push((shifted_value & 0xFF) as u8);
            shifted_value >>= 8;
        }

        encoded_value.reverse();

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::Error::NoDataForType)?;
        }

        if raw.len() > 8 {
            return Err(asn1err::Error::ImplementationError(
                "Too much data for implementation".to_string(),
            ))?;
        }

        let signed_bit = (raw[0] & 0x80) >> 7;
        let mut value: i64 = (signed_bit as i64) * -1;

        for byte in raw.iter() {
            value <<= 8;
            value += (*byte as i64) & 0xFF;
        }

        self._value = Some(value);

        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

impl Integer {
    pub fn value(&self) -> Option<i64> {
        match &self._value {
            Some(ref value) => {
                return Some(*value);
            }
            None => {
                return None;
            }
        };
    }

    pub fn set_value(&mut self, value: i64) {
        self._value = Some(value);
    }

    fn _encoded_value_size(&self) -> usize {
        if self._value.unwrap() >= 0 {
            return self._calculate_positive_integer_size() as usize;
        }

        return self._calculate_negative_integer_size() as usize;
    }

    fn _calculate_negative_integer_size(&self) -> u8 {
        let mut bytes_count = 1;
        let mut shifted_integer = self._value.unwrap();

        while shifted_integer < -128 {
            bytes_count += 1;
            shifted_integer >>= 8;
        }

        return bytes_count;
    }

    fn _calculate_positive_integer_size(&self) -> u8 {
        let mut bytes_count = 1;
        let mut shifted_integer = self._value.unwrap();

        while shifted_integer > 127 {
            bytes_count += 1;
            shifted_integer >>= 8;
        }

        return bytes_count;
    }
}

impl From<i64> for Integer {
    fn from(int: i64) -> Self {
        return Integer { _value: Some(int) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = Integer::from(78);
        assert_eq!(78, b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        assert_eq!(Integer { _value: None }, Integer::default())
    }

    #[test]
    fn test_set_value() {
        let mut b = Integer::default();
        b.set_value(56);
        assert_eq!(56, b.value().unwrap());
    }

    #[test]
    fn test_unset_value() {
        let mut b = Integer::from(78);
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_empty_integer() {
        let integer1 = Integer::default();
        assert_eq!(None, integer1.value());
    }

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x2, 0x1, 0x0], Integer::from(0).encode().unwrap());
        assert_eq!(vec![0x2, 0x1, 0x1], Integer::from(1).encode().unwrap());
        assert_eq!(vec![0x2, 0x1, 0xff], Integer::from(-1).encode().unwrap());

        assert_eq!(vec![0x2, 0x1, 0x7F], Integer::from(127).encode().unwrap());
        assert_eq!(
            vec![0x2, 0x2, 0x00, 0x80],
            Integer::from(128).encode().unwrap()
        );
        assert_eq!(
            vec![0x2, 0x2, 0x01, 0x00],
            Integer::from(256).encode().unwrap()
        );
        assert_eq!(vec![0x2, 0x1, 0x80], Integer::from(-128).encode().unwrap());
        assert_eq!(
            vec![0x2, 0x2, 0xFF, 0x7F],
            Integer::from(-129).encode().unwrap()
        );

        assert_eq!(
            vec![0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8],
            Integer::from(4165284616).encode().unwrap()
        );
        assert_eq!(
            vec![0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB],
            Integer::from(-3310595109).encode().unwrap()
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(Integer::from(0), _parse(&[0x2, 0x1, 0x0]));
        assert_eq!(Integer::from(1), _parse(&[0x2, 0x1, 0x1]));
        assert_eq!(Integer::from(-1), _parse(&[0x2, 0x1, 0xff]));

        assert_eq!(Integer::from(127), _parse(&[0x2, 0x1, 0x7F]));
        assert_eq!(Integer::from(128), _parse(&[0x2, 0x2, 0x00, 0x80]));
        assert_eq!(Integer::from(256), _parse(&[0x2, 0x2, 0x01, 0x00]));
        assert_eq!(Integer::from(-128), _parse(&[0x2, 0x1, 0x80]));
        assert_eq!(Integer::from(-129), _parse(&[0x2, 0x2, 0xFF, 0x7F]));

        assert_eq!(
            Integer::from(4165284616),
            _parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8])
        );
        assert_eq!(
            Integer::from(-3310595109),
            _parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB])
        );
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!(
            (Integer::from(0), 3),
            _parse_with_consumed_octets(&[0x2, 0x1, 0x0, 0x22])
        );
        assert_eq!(
            (Integer::from(1), 3),
            _parse_with_consumed_octets(&[0x2, 0x1, 0x1, 0x22])
        );
        assert_eq!(
            (Integer::from(-1), 3),
            _parse_with_consumed_octets(&[0x2, 0x1, 0xff, 0x22])
        );

        assert_eq!(
            (Integer::from(127), 3),
            _parse_with_consumed_octets(&[0x2, 0x1, 0x7F, 0x22])
        );
        assert_eq!(
            (Integer::from(128), 4),
            _parse_with_consumed_octets(&[0x2, 0x2, 0x00, 0x80, 0x22])
        );
        assert_eq!(
            (Integer::from(256), 4),
            _parse_with_consumed_octets(&[0x2, 0x2, 0x01, 0x00, 0x22])
        );
        assert_eq!(
            (Integer::from(-128), 3),
            _parse_with_consumed_octets(&[0x2, 0x1, 0x80, 0x22])
        );
        assert_eq!(
            (Integer::from(-129), 4),
            _parse_with_consumed_octets(&[0x2, 0x2, 0xFF, 0x7F, 0x22])
        );

        assert_eq!(
            (Integer::from(4165284616), 7),
            _parse_with_consumed_octets(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8, 0x22])
        );
        assert_eq!(
            (Integer::from(-3310595109), 7),
            _parse_with_consumed_octets(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB, 0x22])
        );
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic(expected = "NoDataForType")]
    #[test]
    fn test_decode_without_enough_value_octets() {
        _parse(&[0x2, 0x0]);
    }

    #[should_panic(expected = "ImplementationError")]
    #[test]
    fn test_decode_wit_too_much_value_octets() {
        _parse(&[0x2, 0x9, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9]);
    }

    fn _parse(raw: &[u8]) -> Integer {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (Integer, usize) {
        let mut b = Integer::from(0);
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
