use super::tag::{Tag};
use super::traits::{Asn1Object, Asn1InstanciableObject, Asn1Tagged};
use super::sequence::{SEQUENCE_TAG_NUMBER};
use super::error::Asn1Error;

#[derive(Debug)]
pub struct SequenceOf<T: Asn1InstanciableObject> {
    tag: Tag,
    components: Vec<T>
}

impl<T: Asn1InstanciableObject> Asn1Tagged for SequenceOf<T> {
    fn type_tag() -> Tag {
        return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
    }
}

impl <T: Asn1InstanciableObject> Asn1Object for SequenceOf<T> {

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
        let mut value: Vec<u8> = Vec::new();
        for item in self.components.iter() {
            value.append(&mut item.encode()?)
        }
        return Ok(value);
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        let mut components: Vec<T> = Vec::new();
        let mut consumed_octets = 0;

        while consumed_octets < raw.len() {
            let mut component = T::new_default();
            let component_consumed_octets = component.decode(&raw[consumed_octets..])?;

            consumed_octets += component_consumed_octets;
            components.push(component);
        }

        self.components = components;
        return Ok(());
    }
}

impl<T: Asn1InstanciableObject> SequenceOf<T> {

    pub fn new() -> SequenceOf<T> {
        let components: Vec<T> = Vec::new();
        return SequenceOf{
            tag: SequenceOf::<T>::type_tag(),
            components
        };
    }

    pub fn add(&mut self, item : T) {
        self.components.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::integer::{Integer, INTEGER_TAG_NUMBER};

    #[test]
    fn test_encode_sequence_of_integers(){
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.add(Integer::new(9));
        seq_of.add(Integer::new(1000));

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

        assert_eq!(Integer::new(9), seq_of.components[0]);
        assert_eq!(Integer::new(1000), seq_of.components[1]);

    }

    #[test]
    fn test_decode_empty_sequence() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0x30, 0x0]).unwrap();
        assert_eq!(0, seq_of.components.len());
    }

    #[test]
    fn test_decode_integers_with_excesive_bytes() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        let consumed_octets = seq_of.decode(&[0x30, 0x7, 
                                            INTEGER_TAG_NUMBER, 0x1, 0x9, 
                                            INTEGER_TAG_NUMBER, 0x2, 0x3, 0xe8, 
                                            0xff, 0xff]).unwrap();
        
        assert_eq!(9, consumed_octets);
        assert_eq!(Integer::new(9), seq_of.components[0]);
        assert_eq!(Integer::new(1000), seq_of.components[1]);
    }

    #[should_panic(expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_sequence_of_tag() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0xff, 0x0]).unwrap();
    }

    #[should_panic(expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_inner_type_tag() {
        let mut seq_of: SequenceOf<Integer> = SequenceOf::new();
        seq_of.decode(&[0x30, 0x3, 0xff, 0x1, 0x9]).unwrap();
    }

}
