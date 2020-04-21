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

mod sequence;
pub use sequence::*;

mod sequenceof;
pub use sequenceof::*;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::*;

    #[test]
    fn test_encode_common_tags() {
        assert_eq!(vec![0x01], Boolean::default().tag().encode());
        assert_eq!(vec![0x02], Integer::default().tag().encode());
        assert_eq!(vec![0x03], BitString::default().tag().encode());
        assert_eq!(vec![0x04], OctetString::default().tag().encode());
        assert_eq!(vec![0x30], SequenceOf::<Integer>::default().tag().encode());
        assert_eq!(vec![0x16], IA5String::default().tag().encode());
        assert_eq!(vec![0x18], GeneralizedTime::default().tag().encode());
    }
}
