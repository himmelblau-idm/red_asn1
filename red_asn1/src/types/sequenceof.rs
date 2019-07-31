use crate::tag::{Tag};
use crate::traits::*;
use crate::error as asn1err;
use std::ops::{Deref, DerefMut};

pub static SEQUENCE_TAG_NUMBER: u8 = 0x10;


/// Class to encode/decode SequenceOf ASN1
#[derive(Debug, Default)]
pub struct SequenceOf<T: Asn1Object + Default> {
    components: Vec<T>
}

impl<T: Asn1Object + Default> Deref for SequenceOf<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.components
    }
}

impl<T: Asn1Object + Default> DerefMut for SequenceOf<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.components
    }
}

impl<T: Asn1Object + Default> SequenceOf<T> {

    pub fn new() -> SequenceOf<T> {
        return SequenceOf{
            components: Vec::<T>::new()
        };
    }

    pub fn value(&self) -> &Vec<T> {
        return &self.components;
    }

}

impl <T: Asn1Object + Default> Asn1Object for SequenceOf<T> {

    fn tag(&self) -> Tag {
        return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        let mut value: Vec<u8> = Vec::new();
        for item in self.components.iter() {
            value.append(&mut item.encode()?)
        }
        return Ok(value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        let mut components: Vec<T> = Vec::new();
        let mut consumed_octets = 0;

        while consumed_octets < raw.len() {
            let mut component = T::default();
            let component_consumed_octets = component.decode(&raw[consumed_octets..])?;

            consumed_octets += component_consumed_octets;
            components.push(component);
        }

        self.components = components;
        return Ok(());
    }

    fn unset_value(&mut self) {
        self.components = Vec::new();
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::integer::{Integer, INTEGER_TAG_NUMBER};

    #[test]
    fn test_create() {
        let seq_of: SequenceOf<Integer> = SequenceOf::new();
        assert_eq!(&Vec::<Integer>::new(), seq_of.value());
    }

    #[test]
    fn test_create_default() {
        let seq_of: SequenceOf<Integer> = SequenceOf::default();
        assert_eq!(&Vec::<Integer>::new(), seq_of.value());
    }

    #[test]
    fn test_unset_value() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.push(Integer::new(9));
        seq_of.unset_value();
        assert_eq!(&Vec::<Integer>::new(), seq_of.value());
    }

    #[test]
    fn test_encode_sequence_of_integers(){
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.push(Integer::new(9));
        seq_of.push(Integer::new(1000));

        assert_eq!(vec![0x30, 0x7, 
                        INTEGER_TAG_NUMBER, 0x1, 0x9, 
                        INTEGER_TAG_NUMBER, 0x2, 0x3, 0xe8], seq_of.encode().unwrap());

    }

    #[test]
    fn test_encode_empty_sequence_of(){
        let seq_of: SequenceOf<Integer> = SequenceOf::new();

        assert_eq!(vec![0x30, 0x0], seq_of.encode().unwrap());
    }

    #[test]
    fn test_decode_sequence_of_integers() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0x30, 0x7, 
                        INTEGER_TAG_NUMBER, 0x1, 0x9, 
                        INTEGER_TAG_NUMBER, 0x2, 0x3, 0xe8]).unwrap();

        assert_eq!(Integer::new(9), seq_of[0]);
        assert_eq!(Integer::new(1000), seq_of[1]);

    }

    #[test]
    fn test_decode_empty_sequence() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0x30, 0x0]).unwrap();
        assert_eq!(0, seq_of.len());
    }

    #[test]
    fn test_decode_integers_with_excesive_bytes() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        let consumed_octets = seq_of.decode(&[0x30, 0x7, 
                                            INTEGER_TAG_NUMBER, 0x1, 0x9, 
                                            INTEGER_TAG_NUMBER, 0x2, 0x3, 0xe8, 
                                            0xff, 0xff]).unwrap();
        
        assert_eq!(9, consumed_octets);
        assert_eq!(Integer::new(9), seq_of[0]);
        assert_eq!(Integer::new(1000), seq_of[1]);
    }

    #[should_panic(expected = "Invalid type tag: Not match with expected tag")]
    #[test]
    fn test_decode_with_invalid_sequence_of_tag() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0xff, 0x0]).unwrap();
    }

    #[should_panic(expected = "Invalid type tag: Not match with expected tag")]
    #[test]
    fn test_decode_with_invalid_inner_type_tag() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0x30, 0x3, 0xff, 0x1, 0x9]).unwrap();
    }

}
