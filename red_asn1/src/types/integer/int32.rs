use super::general::{build_integer_value, parse_integer_value};
use super::INTEGER_TAG_NUMBER;
use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::Asn1Object;
use std::convert::TryInto;

impl Asn1Object for i32 {
    fn tag() -> Tag {
        return Tag::new_primitive_universal(INTEGER_TAG_NUMBER);
    }

    fn build_value(&self) -> Vec<u8> {
        return build_integer_value(*self as i128);
    }

    fn parse_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let value = parse_integer_value(raw, 4)?;
        *self = value.try_into().expect("Error parsing i32, too much data");
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(vec![0x2, 0x1, 0x0], i32::from(0).build());
        assert_eq!(vec![0x2, 0x1, 0x1], i32::from(1).build());
        assert_eq!(vec![0x2, 0x1, 0xff], i32::from(-1).build());

        assert_eq!(vec![0x2, 0x1, 0x7F], i32::from(127).build());
        assert_eq!(vec![0x2, 0x2, 0x00, 0x80], i32::from(128).build());
        assert_eq!(vec![0x2, 0x2, 0x01, 0x00], i32::from(256).build());
        assert_eq!(vec![0x2, 0x1, 0x80], i32::from(-128).build());
        assert_eq!(vec![0x2, 0x2, 0xFF, 0x7F], i32::from(-129).build());
    }

    #[test]
    fn test_parse() {
        assert_eq!(i32::from(0), i32::parse(&[0x2, 0x1, 0x0]).unwrap().1);
        assert_eq!(i32::from(1), i32::parse(&[0x2, 0x1, 0x1]).unwrap().1);
        assert_eq!(i32::from(-1), i32::parse(&[0x2, 0x1, 0xff]).unwrap().1);

        assert_eq!(i32::from(127), i32::parse(&[0x2, 0x1, 0x7F]).unwrap().1);
        assert_eq!(
            i32::from(128),
            i32::parse(&[0x2, 0x2, 0x00, 0x80]).unwrap().1
        );
        assert_eq!(
            i32::from(256),
            i32::parse(&[0x2, 0x2, 0x01, 0x00]).unwrap().1
        );
        assert_eq!(i32::from(-128), i32::parse(&[0x2, 0x1, 0x80]).unwrap().1);
        assert_eq!(
            i32::from(-129),
            i32::parse(&[0x2, 0x2, 0xFF, 0x7F]).unwrap().1
        );
    }

    #[test]
    fn test_parse_with_excesive_bytes() {
        let x: &[u8] = &[0x22];
        assert_eq!(
            (x, i32::from(0)),
            i32::parse(&[0x2, 0x1, 0x0, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(1)),
            i32::parse(&[0x2, 0x1, 0x1, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(-1)),
            i32::parse(&[0x2, 0x1, 0xff, 0x22]).unwrap()
        );

        assert_eq!(
            (x, i32::from(127)),
            i32::parse(&[0x2, 0x1, 0x7F, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(128)),
            i32::parse(&[0x2, 0x2, 0x00, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(256)),
            i32::parse(&[0x2, 0x2, 0x01, 0x00, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(-128)),
            i32::parse(&[0x2, 0x1, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i32::from(-129)),
            i32::parse(&[0x2, 0x2, 0xFF, 0x7F, 0x22]).unwrap()
        );
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_parse_with_invalid_tag() {
        i32::parse(&[0x7, 0x1, 0x0]).unwrap();
    }

    #[should_panic(expected = "IncorrectValue(\"No octets for i32\")")]
    #[test]
    fn test_parse_without_enough_value_octets() {
        i32::parse(&[0x2, 0x0]).unwrap();
    }

    #[should_panic(
        expected = "IncorrectValue(\"Too many octets for i32: 5 octets\")"
    )]
    #[test]
    fn test_parse_wit_too_much_value_octets() {
        i32::parse(&[0x2, 5, 0, 0x1, 0x2, 0x3, 0x4]).unwrap();
    }
}
