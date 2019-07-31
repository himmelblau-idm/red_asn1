use crate::tag::*;
use crate::error as asn1err;

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
        let mut decoded_tag = Tag::new(0, TagType::Primitive, TagClass::Universal);
        let consumed_octets = decoded_tag.decode(raw_tag)?;

        if decoded_tag != self.tag() {
            return Err(asn1err::ErrorKind::InvalidTypeTagUnmatched)?;
        }
        return Ok(consumed_octets);
    }

    /// To encode the object value length to DER, should not be overwritten
    fn encode_length(&self, value_size: usize) -> Vec<u8> {
        if value_size < 128 {
            return vec![value_size as u8];
        }

        let mut shifted_length = value_size;
        let mut octets_count: u8 = 0;
        let mut encoded_length : Vec<u8> = Vec::new();

        while shifted_length > 0 {
            octets_count += 1;
            encoded_length.push(shifted_length as u8);
            shifted_length >>= 8;
        }

        encoded_length.push(octets_count | 0b10000000);
        
        encoded_length.reverse();


        return encoded_length;
    }

    /// To decode the object value length from DER, should not be overwritten
    fn decode_length(&self, raw_length: &[u8]) -> asn1err::Result<(usize, usize)> {
        let raw_length_length = raw_length.len();
        if raw_length_length == 0 {
            return Err(asn1err::ErrorKind::InvalidLengthEmpty)?;
        }

        let mut consumed_octets: usize = 1;
        let is_short_form = (raw_length[0] & 0x80) == 0;
        if is_short_form {
            return Ok(((raw_length[0] & 0x7F) as usize, consumed_octets));
        }

        let length_of_length = (raw_length[0] & 0x7F) as usize;
        if length_of_length >= raw_length_length {
            return Err(asn1err::ErrorKind::InvalidLengthOfLength)?;
        }

        let mut length: usize = 0;
        for i in 1..(length_of_length + 1) {
            length <<= 8;
            length += raw_length[i] as usize;
        }
        consumed_octets += length_of_length;

        return Ok((length, consumed_octets));
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
        let mut encoded_length = self.encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return Ok(encoded);
    }

    /// To decode the object from DER, generally does not need to be overwritten.
    /// Usually, just decode_value should be overwritten
    fn decode(&mut self, raw: &[u8]) -> asn1err::Result<usize> {
        let mut consumed_octets = self.decode_tag(raw)?;

        let (_, raw_length) = raw.split_at(consumed_octets);

        let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;

        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(asn1err::ErrorKind::NoDataForLength)?;
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

    #[should_panic(expected="Invalid type tag: Not match with expected tag")]
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

    #[test]
    fn encode_length() {
        assert_eq!(vec![0x0], _build_length(0));
        assert_eq!(vec![0x1], _build_length(1));
        assert_eq!(vec![0x7F], _build_length(127));
        assert_eq!(vec![0x81, 0x80], _build_length(128));
        assert_eq!(vec![0x81, 0xFF], _build_length(255));
        assert_eq!(vec![0x82, 0x01, 0x00], _build_length(256));
        assert_eq!(vec![0x82, 0xFF, 0xFF], _build_length(65535));
        assert_eq!(vec![0x83, 0x01, 0x00, 0x00], _build_length(65536));

        assert_eq!(vec![0x84, 0x10, 0xf3, 0x91, 0xbd], _build_length(0x10f391bd));
        assert_eq!(vec![0x84, 0x0f, 0xc4, 0x69, 0x89], _build_length(0xfc46989));
        assert_eq!(vec![0x84, 0x31, 0xb2, 0x50, 0x42], _build_length(0x31b25042));
        assert_eq!(vec![0x84, 0x13, 0x93, 0xaa, 0x93], _build_length(0x1393aa93));
        assert_eq!(vec![0x84, 0x05, 0x71, 0x6f, 0xa9], _build_length(0x5716fa9));
    }

    #[test]
    fn decode_length() {
        assert_eq!((0, 1), _parse_length(vec![0x0]));
        assert_eq!((1, 1), _parse_length(vec![0x1]));
        assert_eq!((127, 1), _parse_length(vec![0x7F]));
        assert_eq!((128, 2), _parse_length(vec![0x81, 0x80]));
        assert_eq!((255, 2), _parse_length(vec![0x81, 0xFF]));
        assert_eq!((256, 3), _parse_length(vec![0x82, 0x01, 0x00]));
        assert_eq!((65535, 3), _parse_length(vec![0x82, 0xFF, 0xFF]));
        assert_eq!((65536, 4), _parse_length(vec![0x83, 0x01, 0x00, 0x00]));

        assert_eq!((0x10f391bd, 5), _parse_length(vec![0x84, 0x10, 0xf3, 0x91, 0xbd]));
        assert_eq!((0xfc46989, 5), _parse_length(vec![0x84, 0x0f, 0xc4, 0x69, 0x89]));
        assert_eq!((0x31b25042, 5), _parse_length(vec![0x84, 0x31, 0xb2, 0x50, 0x42]));
        assert_eq!((0x1393aa93, 5), _parse_length(vec![0x84, 0x13, 0x93, 0xaa, 0x93]));
        assert_eq!((0x5716fa9, 5), _parse_length(vec![0x84, 0x05, 0x71, 0x6f, 0xa9]));
    }

    #[should_panic (expected = "Invalid value: Not enough data for length")]
    #[test]
    fn test_decode_with_excesive_length_for_data() {
        TestObject::new_tagged(Tag::new_primitive_universal(1)).decode(&[0x1, 0x3, 0x0]).unwrap();
    }

    fn _build_length(length: usize) -> Vec<u8> {
        return TestObject::new().encode_length(length);
    }

    fn _parse_length(raw_length: Vec<u8>) -> (usize, usize) {
        return TestObject::new().decode_length(&raw_length).unwrap();
    }

    fn _build_tag(tag: Tag) -> Vec<u8> {
        return TestObject::new_tagged(tag).encode_tag();
    }

}