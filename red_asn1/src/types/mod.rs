mod bitstring;
pub use bitstring::*;

mod boolean;
pub use boolean::*;

mod generalizedtime;
pub use generalizedtime::*;

mod generalstring;
pub use generalstring::*;

mod ia5string;
pub use ia5string::*;

mod integer;
pub use integer::*;

mod octetstring;
pub use octetstring::*;

mod sequenceof;
pub use sequenceof::*;

mod optional;
pub use optional::Optional;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::*;

    #[test]
    fn test_encode_common_tags() {
        assert_eq!(vec![0x01], Boolean::tag().encode());
        assert_eq!(vec![0x02], Integer::tag().encode());
        assert_eq!(vec![0x03], BitString::tag().encode());
        assert_eq!(vec![0x04], OctetString::tag().encode());
        assert_eq!(vec![0x30], SequenceOf::<Integer>::tag().encode());
        assert_eq!(vec![0x16], IA5String::tag().encode());
        assert_eq!(vec![0x18], GeneralizedTime::tag().encode());
    }
}
