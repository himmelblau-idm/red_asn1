use super::tag::Tag;
use super::traits::{Asn1Object, Asn1InstanciableObject, Asn1Tagged};
use super::error::Asn1Error;
use std::result::Result;

pub static INTEGER_TAG_NUMBER: u8 = 0x2;

#[derive(Debug, PartialEq)]
pub struct Integer {
    tag: Tag,
    value: i64
}

impl Asn1Tagged for Integer {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(INTEGER_TAG_NUMBER);
    }
}



impl Asn1Object for Integer {    

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
         let mut shifted_value = self.value;
         let length = self._encoded_value_size();

        let mut encoded_value: Vec<u8> = Vec::new();

        for _ in 0..length {
            encoded_value.push((shifted_value & 0xFF) as u8);
            shifted_value >>=8;
        }

        encoded_value.reverse();

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        if raw.len() == 0 {
            return Err(Asn1Error::new("Invalid value: Not enough data for type".to_string()));
        }

        if raw.len() > 8 {
            return Err(Asn1Error::new("Invalid value: Too much data for implementation".to_string()));
        }

        let signed_bit = (raw[0] & 0x80) >> 7;
        let mut value: i64 = (signed_bit as i64) * -1;

        for byte in raw.iter() {
            value <<= 8;
            value += (*byte as i64) & 0xFF;
        }

        self.value = value;
        
        return Ok(());
    }
}

impl Asn1InstanciableObject for Integer {
    fn new_default() -> Integer {
        return Integer::new(0);
    }
}


impl Integer {
    pub fn new(value: i64) -> Integer{
        return Integer{
            tag: Integer::type_tag(),
            value
        };
    }

    pub fn value(&self) -> &i64 {
        return &self.value;
    }

    fn _encoded_value_size(&self) -> usize {
        if self.value >= 0 {
            return self._calculate_positive_integer_size() as usize;
        }
        
        return self._calculate_negative_integer_size() as usize;
    }

    fn _calculate_negative_integer_size(&self) -> u8 {
        let mut bytes_count = 1;
        let mut shifted_integer = self.value;

        while shifted_integer < -128 {
            bytes_count += 1;
            shifted_integer >>= 8;
        }

        return bytes_count;
    }

    fn _calculate_positive_integer_size(&self) -> u8 {
        let mut bytes_count = 1;
        let mut shifted_integer = self.value;

        while shifted_integer > 127 {
            bytes_count += 1;
            shifted_integer >>= 8;
        }

        return bytes_count;
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_value_and_tag() {
        let integer1 = Integer::new(78);
        assert_eq!(78, integer1.value);

        assert_eq!(vec![0x02], integer1.tag.encode());
    }

    #[test]
    fn test_encode() {
        assert_eq!(vec![0x2, 0x1, 0x0], Integer::new(0).encode().unwrap());
        assert_eq!(vec![0x2, 0x1, 0x1], Integer::new(1).encode().unwrap());
        assert_eq!(vec![0x2, 0x1, 0xff], Integer::new(-1).encode().unwrap());

        assert_eq!(vec![0x2, 0x1, 0x7F], Integer::new(127).encode().unwrap());
        assert_eq!(vec![0x2, 0x2, 0x00, 0x80], Integer::new(128).encode().unwrap());
        assert_eq!(vec![0x2, 0x2, 0x01, 0x00], Integer::new(256).encode().unwrap());
        assert_eq!(vec![0x2, 0x1, 0x80], Integer::new(-128).encode().unwrap());
        assert_eq!(vec![0x2, 0x2, 0xFF, 0x7F], Integer::new(-129).encode().unwrap());

        assert_eq!(vec![0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8], Integer::new(4165284616).encode().unwrap());
        assert_eq!(vec![0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB], Integer::new(-3310595109).encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(Integer::new(0), _parse(&[0x2, 0x1, 0x0]));
        assert_eq!(Integer::new(1), _parse(&[0x2, 0x1, 0x1]));
        assert_eq!(Integer::new(-1), _parse(&[0x2, 0x1, 0xff]));

        assert_eq!(Integer::new(127), _parse(&[0x2, 0x1, 0x7F]));
        assert_eq!(Integer::new(128), _parse(&[0x2, 0x2, 0x00, 0x80]));
        assert_eq!(Integer::new(256), _parse(&[0x2, 0x2, 0x01, 0x00]));
        assert_eq!(Integer::new(-128), _parse(&[0x2, 0x1, 0x80]));
        assert_eq!(Integer::new(-129), _parse(&[0x2, 0x2, 0xFF, 0x7F]));

        assert_eq!(Integer::new(4165284616), _parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8]));
        assert_eq!(Integer::new(-3310595109), _parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((Integer::new(0), 3), _parse_with_consumed_octets(&[0x2, 0x1, 0x0, 0x22]));
        assert_eq!((Integer::new(1), 3), _parse_with_consumed_octets(&[0x2, 0x1, 0x1, 0x22]));
        assert_eq!((Integer::new(-1), 3), _parse_with_consumed_octets(&[0x2, 0x1, 0xff, 0x22]));

        assert_eq!((Integer::new(127), 3), _parse_with_consumed_octets(&[0x2, 0x1, 0x7F, 0x22]));
        assert_eq!((Integer::new(128), 4), _parse_with_consumed_octets(&[0x2, 0x2, 0x00, 0x80, 0x22]));
        assert_eq!((Integer::new(256), 4), _parse_with_consumed_octets(&[0x2, 0x2, 0x01, 0x00, 0x22]));
        assert_eq!((Integer::new(-128), 3), _parse_with_consumed_octets(&[0x2, 0x1, 0x80, 0x22]));
        assert_eq!((Integer::new(-129), 4), _parse_with_consumed_octets(&[0x2, 0x2, 0xFF, 0x7F, 0x22]));

        assert_eq!((Integer::new(4165284616), 7), _parse_with_consumed_octets(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8, 0x22]));
        assert_eq!((Integer::new(-3310595109), 7), _parse_with_consumed_octets(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB, 0x22]));
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic (expected = "Invalid value: Not enough data for type")]
    #[test]
    fn test_decode_without_enough_value_octets() {
        _parse(&[0x2, 0x0]);
    }

    #[should_panic (expected = "Invalid value: Too much data for implementation")]
    #[test]
    fn test_decode_wit_too_much_value_octets() {
        _parse(&[0x2, 0x9, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9]);
    }

    fn _parse(raw: &[u8]) -> Integer {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (Integer, usize) {
        let mut b = Integer::new(0);
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
