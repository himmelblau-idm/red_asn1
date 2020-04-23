use crate::error as asn1err;
use crate::length::{decode_length, encode_length};
use crate::tag::Tag;
use crate::traits::Asn1Object;

/// Class to encode/decode SequenceOf ASN1
pub type Optional<T> = Option<T>;

impl<T: Asn1Object> Asn1Object for Option<T> {
    fn encode(&self) -> Vec<u8> {
        if let Some(value) = self {
            let mut encoded = Self::tag().encode();
            let mut encoded_value = value.encode_value();
            let mut encoded_length = encode_length(encoded_value.len());
            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }

        return Vec::new();
    }

    fn decode(raw: &[u8]) -> asn1err::Result<(usize, Self)> {
        let mut consumed_octets;
        let decoded_tag;
        match Tag::decode(raw) {
            Err(_) => return Ok((0, None)),
            Ok((octets, tag)) => {
                consumed_octets = octets;
                decoded_tag = tag;
            }
        }

        if decoded_tag != Self::tag() {
            return Ok((0, None));
        }

        let (_, raw_length) = raw.split_at(consumed_octets);

        let (value_length, consumed_octets_by_length) =
            decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;

        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(asn1err::Error::NoDataForLength)?;
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        let mut asn1obj = T::default();
        asn1obj.decode_value(raw_value)?;
        consumed_octets += value_length;

        return Ok((consumed_octets, Some(asn1obj)));
    }

    fn tag() -> Tag {
        return T::tag();
    }

    fn encode_value(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn decode_value(&mut self, _: &[u8]) -> asn1err::Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GeneralString, Integer};

    #[test]
    fn test_encode_optional_some() {
        assert_eq!(vec![0x2, 0x1, 0x1], Some(Integer::from(1)).encode());
        assert_eq!(
            vec![
                0x1b, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73,
                0x61, 0x2e, 0x63, 0x6f, 0x6d
            ],
            Some(GeneralString::from("test1@rsa.com")).encode()
        );
    }

    #[test]
    fn test_encode_none() {
        let o: Option<Integer> = None;
        assert_eq!(Vec::<u8>::new(), o.encode());
    }

    #[test]
    fn test_decode_optional_some() {
        assert_eq!(
            Some(Integer::from(1)),
            Option::<Integer>::decode(&[0x2, 0x1, 0x1]).unwrap().1
        );
        assert_eq!(
            Some(GeneralString::from("test1@rsa.com")),
            Option::<GeneralString>::decode(&[
                0x1b, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x31, 0x40, 0x72, 0x73,
                0x61, 0x2e, 0x63, 0x6f, 0x6d
            ])
            .unwrap()
            .1
        );
    }


    #[test]
    fn test_decode_none_mismatch_tag() {
        let o: Option<Integer> = None;
        assert_eq!(o, Option::<Integer>::decode(&[0x3, 0x0]).unwrap().1);
    }

    #[test]
    fn test_decode_none_no_data() {
        let o: Option<Integer> = None;
        assert_eq!(o, Option::<Integer>::decode(&[]).unwrap().1);
    }

    #[test]
    #[should_panic(expected = "NoDataForLength")]
    fn test_decode_none_no_type_data() {
        Option::<Integer>::decode(&[0x2, 0x3]).unwrap();
    }

    #[test]
    #[should_panic(expected = "NotEnoughLengthOctects")]
    fn test_decode_option_invalid_length() {
        Option::<Integer>::decode(&[0x2, 0xff]).unwrap();
    }
}
