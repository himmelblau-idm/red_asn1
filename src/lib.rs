mod boolean;
mod integer;
mod bitstring;
mod octetstring;
mod ia5string;
mod generalstring;
mod generalizedtime;
mod sequence;
mod sequenceof;
mod tag;
mod traits;
mod error;

pub use traits::{Asn1Object, Asn1InstanciableObject, Asn1Tagged};
pub use boolean::{Boolean, BOOLEAN_TAG_NUMBER};
pub use integer::{Integer, INTEGER_TAG_NUMBER};
pub use bitstring::{BitSring, BIT_STRING_TAG_NUMBER};
pub use octetstring::{OctetString, OCTET_STRING_TAG_NUMBER};
pub use sequence::{Sequence, SEQUENCE_TAG_NUMBER, SequenceComponent2};
pub use ia5string::{IA5String, IA5STRING_TAG_NUMBER};
pub use generalstring::{GeneralString, GENERALSTRING_TAG_NUMBER};
pub use generalizedtime::{GeneralizedTime, GENERALIZED_TIME_TAG_NUMBER, TimeFormat};
pub use sequenceof::{SequenceOf};
pub use error::Asn1Error;
pub use tag::{Tag, TagClass, TagType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_common_tags() {
        assert_eq!(vec![0x01], Boolean::type_tag().encode());
        assert_eq!(vec![0x02], Integer::type_tag().encode());
        assert_eq!(vec![0x03], BitSring::type_tag().encode());
        assert_eq!(vec![0x04], OctetString::type_tag().encode());
        assert_eq!(vec![0x30], Sequence::type_tag().encode());
        assert_eq!(vec![0x30], SequenceOf::<Integer>::type_tag().encode());
        assert_eq!(vec![0x16], IA5String::type_tag().encode());
        assert_eq!(vec![0x18], GeneralizedTime::type_tag().encode());
    }
}