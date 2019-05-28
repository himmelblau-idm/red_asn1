extern crate asn1;
extern crate asn1_derive;


use asn1::*;
use asn1_derive::Asn1Sequence;

#[test]
fn test_simple_sequence_definition() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.set_age(Integer::new(9));

    assert_eq!(&Integer::new(9), p.get_age());
}


#[test]
fn test_simple_sequence_encoding() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_sequence_with_context_tags_encoding() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x5, 
                    0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_sequence_with_optional_component_encoding() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SequenceComponent2<Integer>
    }

    let p = Person::new();

    assert_eq!(vec![0x30, 0x0], p.encode().unwrap());
}

#[test]
fn test_simple_sequence_decoding() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.decode(&[0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9]).unwrap();

    assert_eq!(&Integer::new(9), p.get_age());
}


#[test]
fn test_decoding_sequence_with_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.decode(&[0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9]).unwrap();

    assert_eq!(&Integer::new(9), p.get_age());
}