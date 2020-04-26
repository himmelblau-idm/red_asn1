use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::Asn1Object;

pub static INTEGER_TAG_NUMBER: u8 = 0x2;

/// Class to build/parse Integer ASN1
pub type Integer = i128;

impl Asn1Object for i128 {
    fn tag() -> Tag {
        return Tag::new_primitive_universal(INTEGER_TAG_NUMBER);
    }

    fn build_value(&self) -> Vec<u8> {
        let mut shifted_value = *self;
        let length = calculate_integer_size(*self);

        let mut encoded_value: Vec<u8> = Vec::new();

        for _ in 0..length {
            encoded_value.push((shifted_value & 0xFF) as u8);
            shifted_value >>= 8;
        }

        encoded_value.reverse();

        return encoded_value;
    }

    fn parse_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() == 0 {
            return Err(asn1err::Error::NoDataForType)?;
        }

        if raw.len() > 16 {
            return Err(asn1err::Error::ImplementationError(
                "Too much data for implementation".to_string(),
            ))?;
        }

        let signed_bit = (raw[0] & 0x80) >> 7;
        let mut value = (signed_bit as i128) * -1;

        for byte in raw.iter() {
            value <<= 8;
            value += (*byte as i128) & 0xFF;
        }

        *self = value;

        return Ok(());
    }
}

fn calculate_integer_size(int: i128) -> usize {
    if int >= 0 {
        return calculate_positive_integer_size(int) as usize;
    }
    return calculate_negative_integer_size(int) as usize;
}

fn calculate_negative_integer_size(int: i128) -> u8 {
    let mut bytes_count = 1;
    let mut shifted_integer = int;

    while shifted_integer < -128 {
        bytes_count += 1;
        shifted_integer >>= 8;
    }

    return bytes_count;
}

fn calculate_positive_integer_size(int: i128) -> u8 {
    let mut bytes_count = 1;
    let mut shifted_integer = int;

    while shifted_integer > 127 {
        bytes_count += 1;
        shifted_integer >>= 8;
    }

    return bytes_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(vec![0x2, 0x1, 0x0], Integer::from(0).build());
        assert_eq!(vec![0x2, 0x1, 0x1], Integer::from(1).build());
        assert_eq!(vec![0x2, 0x1, 0xff], Integer::from(-1).build());

        assert_eq!(vec![0x2, 0x1, 0x7F], Integer::from(127).build());
        assert_eq!(vec![0x2, 0x2, 0x00, 0x80], Integer::from(128).build());
        assert_eq!(vec![0x2, 0x2, 0x01, 0x00], Integer::from(256).build());
        assert_eq!(vec![0x2, 0x1, 0x80], Integer::from(-128).build());
        assert_eq!(vec![0x2, 0x2, 0xFF, 0x7F], Integer::from(-129).build());

        assert_eq!(
            vec![0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8],
            Integer::from(4165284616i64).build()
        );
        assert_eq!(
            vec![0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB],
            Integer::from(-3310595109i64).build()
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Integer::from(0),
            Integer::parse(&[0x2, 0x1, 0x0]).unwrap().1
        );
        assert_eq!(
            Integer::from(1),
            Integer::parse(&[0x2, 0x1, 0x1]).unwrap().1
        );
        assert_eq!(
            Integer::from(-1),
            Integer::parse(&[0x2, 0x1, 0xff]).unwrap().1
        );

        assert_eq!(
            Integer::from(127),
            Integer::parse(&[0x2, 0x1, 0x7F]).unwrap().1
        );
        assert_eq!(
            Integer::from(128),
            Integer::parse(&[0x2, 0x2, 0x00, 0x80]).unwrap().1
        );
        assert_eq!(
            Integer::from(256),
            Integer::parse(&[0x2, 0x2, 0x01, 0x00]).unwrap().1
        );
        assert_eq!(
            Integer::from(-128),
            Integer::parse(&[0x2, 0x1, 0x80]).unwrap().1
        );
        assert_eq!(
            Integer::from(-129),
            Integer::parse(&[0x2, 0x2, 0xFF, 0x7F]).unwrap().1
        );

        assert_eq!(
            Integer::from(4165284616i64),
            Integer::parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8])
                .unwrap()
                .1
        );
        assert_eq!(
            Integer::from(-3310595109i64),
            Integer::parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB])
                .unwrap()
                .1
        );
    }

    #[test]
    fn test_parse_with_excesive_bytes() {
        let x: &[u8] = &[0x22];
        assert_eq!(
            (x, Integer::from(0)),
            Integer::parse(&[0x2, 0x1, 0x0, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(1)),
            Integer::parse(&[0x2, 0x1, 0x1, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(-1)),
            Integer::parse(&[0x2, 0x1, 0xff, 0x22]).unwrap()
        );

        assert_eq!(
            (x, Integer::from(127)),
            Integer::parse(&[0x2, 0x1, 0x7F, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(128)),
            Integer::parse(&[0x2, 0x2, 0x00, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(256)),
            Integer::parse(&[0x2, 0x2, 0x01, 0x00, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(-128)),
            Integer::parse(&[0x2, 0x1, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, Integer::from(-129)),
            Integer::parse(&[0x2, 0x2, 0xFF, 0x7F, 0x22]).unwrap()
        );

        assert_eq!(
            (x, Integer::from(4165284616i64)),
            Integer::parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8, 0x22])
                .unwrap()
        );
        assert_eq!(
            (x, Integer::from(-3310595109i64)),
            Integer::parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB, 0x22])
                .unwrap()
        );
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_parse_with_invalid_tag() {
        Integer::parse(&[0x7, 0x1, 0x0]).unwrap();
    }

    #[should_panic(expected = "NoDataForType")]
    #[test]
    fn test_parse_without_enough_value_octets() {
        Integer::parse(&[0x2, 0x0]).unwrap();
    }

    #[should_panic(expected = "ImplementationError")]
    #[test]
    fn test_parse_wit_too_much_value_octets() {
        Integer::parse(&[
            0x2, 20, 0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0, 1, 2,
            3, 4, 5, 6, 7, 8, 9,
        ])
        .unwrap();
    }
}
