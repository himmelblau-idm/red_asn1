use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::Asn1Object;
use super::INTEGER_TAG_NUMBER;
use super::general::{build_integer_value,parse_integer_value};

impl Asn1Object for i128 {
    fn tag() -> Tag {
        return Tag::new_primitive_universal(INTEGER_TAG_NUMBER);
    }

    fn build_value(&self) -> Vec<u8> {
        return build_integer_value(*self);
    }

    fn parse_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let value = parse_integer_value(raw, 16)?;
        *self = value;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        assert_eq!(vec![0x2, 0x1, 0x0], i128::from(0).build());
        assert_eq!(vec![0x2, 0x1, 0x1], i128::from(1).build());
        assert_eq!(vec![0x2, 0x1, 0xff], i128::from(-1).build());

        assert_eq!(vec![0x2, 0x1, 0x7F], i128::from(127).build());
        assert_eq!(vec![0x2, 0x2, 0x00, 0x80], i128::from(128).build());
        assert_eq!(vec![0x2, 0x2, 0x01, 0x00], i128::from(256).build());
        assert_eq!(vec![0x2, 0x1, 0x80], i128::from(-128).build());
        assert_eq!(vec![0x2, 0x2, 0xFF, 0x7F], i128::from(-129).build());

        assert_eq!(
            vec![0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8],
            i128::from(4165284616i64).build()
        );
        assert_eq!(
            vec![0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB],
            i128::from(-3310595109i64).build()
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(i128::from(0), i128::parse(&[0x2, 0x1, 0x0]).unwrap().1);
        assert_eq!(i128::from(1), i128::parse(&[0x2, 0x1, 0x1]).unwrap().1);
        assert_eq!(i128::from(-1), i128::parse(&[0x2, 0x1, 0xff]).unwrap().1);

        assert_eq!(i128::from(127), i128::parse(&[0x2, 0x1, 0x7F]).unwrap().1);
        assert_eq!(
            i128::from(128),
            i128::parse(&[0x2, 0x2, 0x00, 0x80]).unwrap().1
        );
        assert_eq!(
            i128::from(256),
            i128::parse(&[0x2, 0x2, 0x01, 0x00]).unwrap().1
        );
        assert_eq!(i128::from(-128), i128::parse(&[0x2, 0x1, 0x80]).unwrap().1);
        assert_eq!(
            i128::from(-129),
            i128::parse(&[0x2, 0x2, 0xFF, 0x7F]).unwrap().1
        );

        assert_eq!(
            i128::from(4165284616i64),
            i128::parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8])
                .unwrap()
                .1
        );
        assert_eq!(
            i128::from(-3310595109i64),
            i128::parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB])
                .unwrap()
                .1
        );
    }

    #[test]
    fn test_parse_with_excesive_bytes() {
        let x: &[u8] = &[0x22];
        assert_eq!(
            (x, i128::from(0)),
            i128::parse(&[0x2, 0x1, 0x0, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(1)),
            i128::parse(&[0x2, 0x1, 0x1, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(-1)),
            i128::parse(&[0x2, 0x1, 0xff, 0x22]).unwrap()
        );

        assert_eq!(
            (x, i128::from(127)),
            i128::parse(&[0x2, 0x1, 0x7F, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(128)),
            i128::parse(&[0x2, 0x2, 0x00, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(256)),
            i128::parse(&[0x2, 0x2, 0x01, 0x00, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(-128)),
            i128::parse(&[0x2, 0x1, 0x80, 0x22]).unwrap()
        );
        assert_eq!(
            (x, i128::from(-129)),
            i128::parse(&[0x2, 0x2, 0xFF, 0x7F, 0x22]).unwrap()
        );

        assert_eq!(
            (x, i128::from(4165284616i64)),
            i128::parse(&[0x2, 0x5, 0x00, 0xF8, 0x45, 0x33, 0x8, 0x22])
                .unwrap()
        );
        assert_eq!(
            (x, i128::from(-3310595109i64)),
            i128::parse(&[0x2, 0x5, 0xFF, 0x3A, 0xAC, 0x53, 0xDB, 0x22])
                .unwrap()
        );
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_parse_with_invalid_tag() {
        i128::parse(&[0x7, 0x1, 0x0]).unwrap();
    }

    #[should_panic(expected = "NoDataForType")]
    #[test]
    fn test_parse_without_enough_value_octets() {
        i128::parse(&[0x2, 0x0]).unwrap();
    }

    #[should_panic(expected = "ImplementationError")]
    #[test]
    fn test_parse_wit_too_much_value_octets() {
        i128::parse(&[
            0x2, 20, 0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0, 1, 2,
            3, 4, 5, 6, 7, 8, 9,
        ])
        .unwrap();
    }
}
