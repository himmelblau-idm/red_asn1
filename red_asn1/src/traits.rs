use crate::tag::*;
use crate::error as asn1err;
use crate::length::{encode_length, decode_length};

/// A trait to allow objects to be encoded/decoded from ASN1-DER
pub trait Asn1Object: Sized + Default {

    /// Method to retrieve the tag of the object, used to identify each object in ASN1
    fn tag() -> Tag;

    /// Method which indicates how object value must be encoded
    fn encode_value(&self) -> Vec<u8>;

    /// Method which indicates how object value must be decoded
    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()>;

    /// To encode the object to DER, generally does not need to be overwritten.
    /// Usually, just encode_value should be overwritten
    fn encode(&self) -> Vec<u8> {
        let mut encoded = Self::tag().encode();
        let mut encoded_value = self.encode_value();
        let mut encoded_length = encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return encoded;
    }

    /// To decode the object from DER, generally does not need to be overwritten.
    /// Usually, just decode_value should be overwritten
    fn decode(raw: &[u8]) -> asn1err::Result<(usize, Self)> {
        let (mut consumed_octets, decoded_tag) = Tag::decode(raw)?;
        if decoded_tag != Self::tag() {
            return Err(asn1err::Error::UnmatchedTag(TagClass::Universal))?;
        }

        let (_, raw_length) = raw.split_at(consumed_octets);

        let (value_length, consumed_octets_by_length) = decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;

        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(asn1err::Error::NoDataForLength)?;
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        let mut asn1obj = Self::default();
        asn1obj.decode_value(raw_value)?;
        consumed_octets += value_length;

        return Ok((consumed_octets, asn1obj));
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestObject {
    }

    impl TestObject {}

    impl Asn1Object for TestObject {
        fn tag() -> Tag {
            return Tag::default();
        }
        fn encode_value(&self) -> Vec<u8> {
            return vec![];
        }

        fn decode_value(&mut self, _raw: &[u8]) -> asn1err::Result<()> {
            return Ok(());
        }

    }

    #[should_panic (expected = "NoDataForLength")]
    #[test]
    fn test_decode_with_excesive_length_for_data() {
        TestObject::decode(&[0x0, 0x3, 0x0]).unwrap();
    }

}
