use super::tag::Tag;
use super::traits::Asn1Tagged;

pub static GENERALSTRING_TAG_NUMBER: u8 = 0x1b;

pub struct GeneralString {
}

impl Asn1Tagged for GeneralString {
    fn type_tag() -> Tag {
        return Tag::new_primitive_universal(GENERALSTRING_TAG_NUMBER);
    }
}