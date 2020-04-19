use crate::tag::*;
use crate::error as asn1err;
use crate::length::{encode_length, decode_length};

/// A trait to allow objects to be encoded/decoded from ASN1-DER
pub trait Asn1Object {

    /// Method to retrieve the tag of the object, used to identify each object in ASN1
    fn tag(&self) -> Tag;

    /// To encode the tag to DER, should not be overwritten
    fn encode_tag(&self) -> Vec<u8> {
        return self.tag().encode();
    }

    /// To decode the tag from DER, should not be overwritten
    fn decode_tag(&self, raw_tag: &[u8]) -> asn1err::Result<usize> {
        let (consumed_octets, decoded_tag) = Tag::decode(raw_tag)?;

        if decoded_tag != self.tag() {
            return Err(asn1err::Error::UnmatchedTag(TagClass::Universal))?;
        }
        return Ok(consumed_octets);
    }

    /// Method which indicates how object value must be encoded
    fn encode_value(&self) -> asn1err::Result<Vec<u8>>;

    /// Method which indicates how object value must be decoded
    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()>;


    /// To encode the object to DER, generally does not need to be overwritten.
    /// Usually, just encode_value should be overwritten
    fn encode(&self) -> asn1err::Result<Vec<u8>> {
        let mut encoded = self.encode_tag();
        let mut encoded_value = self.encode_value()?;
        let mut encoded_length = encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return Ok(encoded);
    }

    /// To decode the object from DER, generally does not need to be overwritten.
    /// Usually, just decode_value should be overwritten
    fn decode(&mut self, raw: &[u8]) -> asn1err::Result<usize> {
        let mut consumed_octets = self.decode_tag(raw)?;

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
        fn new() -> TestObject{
            return TestObject{ tag: Tag::new(0, TagType::Primitive, TagClass::Universal)};
        }

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

    #[test]
    fn test_encode_tag() {
        assert_eq!(vec![0x00], _build_tag(Tag::new(0, TagType::Primitive, TagClass::Universal)));
        assert_eq!(vec![0x40], _build_tag(Tag::new(0, TagType::Primitive, TagClass::Application)));
        assert_eq!(vec![0x80], _build_tag(Tag::new(0, TagType::Primitive, TagClass::Context)));
        assert_eq!(vec![0xc0], _build_tag(Tag::new(0, TagType::Primitive, TagClass::Private)));
        assert_eq!(vec![0x20], _build_tag(Tag::new(0, TagType::Constructed, TagClass::Universal)));
        assert_eq!(vec![0x60], _build_tag(Tag::new(0, TagType::Constructed, TagClass::Application)));
        assert_eq!(vec![0xa0], _build_tag(Tag::new(0, TagType::Constructed, TagClass::Context)));
        assert_eq!(vec![0xe0], _build_tag(Tag::new(0, TagType::Constructed, TagClass::Private)));

        assert_eq!(vec![0x1E], _build_tag(Tag::new(30, TagType::Primitive, TagClass::Universal)));
        assert_eq!(vec![0x1F, 0x1F], _build_tag(Tag::new(31, TagType::Primitive, TagClass::Universal)));
        assert_eq!(vec![0x1F, 0x7F], _build_tag(Tag::new(127, TagType::Primitive, TagClass::Universal)));
        assert_eq!(vec![0x1F, 0x80, 0x01], _build_tag(Tag::new(128, TagType::Primitive, TagClass::Universal)));
        assert_eq!(vec![0x1F, 0xFF, 0x01], _build_tag(Tag::new(255, TagType::Primitive, TagClass::Universal)));

        assert_eq!(vec![0xdf, 0xc6, 0x01], _build_tag(Tag::new(198, TagType::Primitive, TagClass::Private)));
        assert_eq!(vec![0xff, 0x6a], _build_tag(Tag::new(106, TagType::Constructed, TagClass::Private)));
        assert_eq!(vec![0x3f, 0x39], _build_tag(Tag::new(57, TagType::Constructed, TagClass::Universal)));
        assert_eq!(vec![0xbf, 0x24], _build_tag(Tag::new(36, TagType::Constructed, TagClass::Context)));
        assert_eq!(vec![0xf4], _build_tag(Tag::new(20, TagType::Constructed, TagClass::Private)));
        assert_eq!(vec![0x6b], _build_tag(Tag::new(11, TagType::Constructed, TagClass::Application)));
    }

    #[test]
    fn test_decode_tag() {
        assert_eq!(1, TestObject::new().decode_tag(&[0x0]).unwrap());
        assert_eq!(1, TestObject::new_tagged(Tag::new_primitive_universal(0x1)).decode_tag(&[0x1]).unwrap());
        assert_eq!(2, TestObject::new_tagged(Tag::new_primitive_universal(0x1F)).decode_tag(&[0x1F, 0x1F]).unwrap());
    }

    #[should_panic(expected="UnmatchedTag")]
    #[test]
    fn test_decode_different_tag() {
        assert_eq!(1, TestObject::new().decode_tag(&[0x1]).unwrap());
    }

    #[test]
    fn test_decode_tag_with_excesive_bytes() {
        assert_eq!(1, TestObject::new().decode_tag(&[0x0, 0x2]).unwrap());
        assert_eq!(1, TestObject::new_tagged(Tag::new_primitive_universal(0x1)).decode_tag(&[0x1, 0x2]).unwrap());
        assert_eq!(2, TestObject::new_tagged(Tag::new_primitive_universal(0x1F)).decode_tag(&[0x1F, 0x1F, 0x2]).unwrap());
    }



    #[should_panic (expected = "NoDataForLength")]
    #[test]
    fn test_decode_with_excesive_length_for_data() {
        TestObject::new_tagged(Tag::new_primitive_universal(1)).decode(&[0x1, 0x3, 0x0]).unwrap();
    }

    fn _build_tag(tag: Tag) -> Vec<u8> {
        return TestObject::new_tagged(tag).encode_tag();
    }

}
