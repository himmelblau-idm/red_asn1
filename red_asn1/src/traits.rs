use crate::tag::*;
use crate::error as asn1err;
use crate::length::{encode_length, decode_length};

/// A trait to allow objects to be encoded/decoded from ASN1-DER
pub trait Asn1Object {

    /// Method to retrieve the tag of the object, used to identify each object in ASN1
    fn tag(&self) -> Tag;

    /// Method which indicates how object value must be encoded
    fn encode_value(&self) -> asn1err::Result<Vec<u8>>;

    /// Method which indicates how object value must be decoded
    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()>;

    /// To encode the object to DER, generally does not need to be overwritten.
    /// Usually, just encode_value should be overwritten
    fn encode(&self) -> asn1err::Result<Vec<u8>> {
        let mut encoded = self.tag().encode();
        let mut encoded_value = self.encode_value()?;
        let mut encoded_length = encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return Ok(encoded);
    }

    /// To decode the object from DER, generally does not need to be overwritten.
    /// Usually, just decode_value should be overwritten
    fn decode(&mut self, raw: &[u8]) -> asn1err::Result<usize> {
        let (mut consumed_octets, decoded_tag) = Tag::decode(raw)?;

        if decoded_tag != self.tag() {
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

        self.decode_value(raw_value)?;
        consumed_octets += value_length;

        return Ok(consumed_octets);
    }

    /// Method to reset the object value
    fn unset_value(&mut self);
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        tag: Tag
    }

    impl TestObject {
        fn new_tagged(tag: Tag) -> TestObject {
            return TestObject { tag };
        }
    }

    impl Asn1Object for TestObject {
        fn tag(&self) -> Tag {
            return self.tag.clone();
        }
        fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
            return Ok(vec![]);
        }

        fn decode_value(&mut self, _raw: &[u8]) -> asn1err::Result<()> {
            return Ok(());
        }

        fn unset_value(&mut self){
        }
    }

    #[should_panic (expected = "NoDataForLength")]
    #[test]
    fn test_decode_with_excesive_length_for_data() {
        TestObject::new_tagged(Tag::new_primitive_universal(1)).decode(&[0x1, 0x3, 0x0]).unwrap();
    }

}
