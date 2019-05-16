
use super::tag::Tag;
use super::traits::{Asn1Object, Asn1Tagged};
use super::error::*;

pub static BIT_STRING_TAG_NUMBER: u8 = 0x3;

#[derive(Debug, PartialEq)]
pub struct BitSring {
    tag: Tag,
    _value: Option<BitSringValue>
}

#[derive(Debug, PartialEq)]
pub struct BitSringValue {
    bytes: Vec<u8>,
    padding_length: u8
}

impl Asn1Tagged for BitSring {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(BIT_STRING_TAG_NUMBER);
    }
}

impl Asn1Object for BitSring {

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Asn1Result<Vec<u8>> {
        let bitstring_value;

        match &self._value {
            Some(value) => {
                bitstring_value = value;
            },
            None => {
                return Err(Asn1ErrorKind::NoValue)?;
            }
        };

        let mut encoded_value = vec![bitstring_value.padding_length];

        let mut values = Vec::new();
        for i in 0..bitstring_value.bytes.len() {
            values.push(bitstring_value.bytes[i])
        }
        encoded_value.append(&mut values);

        return Ok(encoded_value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Asn1Result<()> {
        if raw.len() == 0 {
            return Err(Asn1ErrorKind::NoDataForType)?;
        }

        let (padding_length, raw_value) = raw.split_at(1);

        self._value = Some(BitSringValue{
            bytes: raw_value.to_vec(),
            padding_length: padding_length[0]
        });
        self._padded_value_with_0();

        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

add Asn1InstanciableObject trait...

impl BitSring {
    pub fn new(bytes: Vec<u8>, padding_length: u8) -> BitSring{
        let mut bs = BitSring {
            tag: BitSring::type_tag(),
            _value: Some(BitSringValue{
                            bytes,
                            padding_length: padding_length % 8
                    })
        };
        bs._padded_value_with_0();
        return bs;
    }

    pub fn new_empty() -> BitSring {
        return BitSring {
            tag: BitSring::type_tag(),
            _value: None
        };
    }

    fn _padded_value_with_0(&mut self) {
        if let Some(ref mut bitstring_value) = self._value {
            match bitstring_value.bytes.pop() {
                Some(last_item) => {
                    bitstring_value.bytes.push(Self::_set_0_padding(last_item, bitstring_value.padding_length));
                },
                None => {}
            }
        }
    }

    fn _set_0_padding(mut item: u8, padding_length: u8) -> u8 {
        item >>= padding_length;
        item <<= padding_length;
        return item;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = BitSring::new(vec![0x0], 0);
        assert_eq!(&BitSringValue{ bytes: vec![0x0], padding_length: 0}, b.value().unwrap());
    }

    #[test]
    fn test_create_empty() {
        let b = BitSring::new_empty();
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

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
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
}
