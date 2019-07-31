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
        assert_eq!(vec![0x01], Boolean::new_default().tag().encode());
        assert_eq!(vec![0x02], Integer::new_default().tag().encode());
        assert_eq!(vec![0x03], BitSring::new_default().tag().encode());
        assert_eq!(vec![0x04], OctetString::new_default().tag().encode());
        assert_eq!(vec![0x30], SequenceOf::<Integer>::new_default().tag().encode());
        assert_eq!(vec![0x16], IA5String::new_default().tag().encode());
        assert_eq!(vec![0x18], GeneralizedTime::new_default().tag().encode());
    }
}
