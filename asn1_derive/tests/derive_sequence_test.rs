extern crate asn1;
extern crate asn1_derive;


use asn1::*;
use asn1_derive::Asn1Sequence;

#[test]
fn test_simple_sequence_definition() {

    #[derive(Asn1Sequence)]
    struct TestSequence {
        id: SequenceComponent2<Integer>,
        data: SequenceComponent2<OctetString>
    }

    let mut seq = TestSequence{
        id: SequenceComponent2::new(),
        data: SequenceComponent2::new()
    };
    seq.set_id(Integer::new(9));
    seq.set_data(OctetString::new(vec![1,2,3,4]));

    assert_eq!(&Integer::new(9), seq.get_id().unwrap());
    assert_eq!(&OctetString::new(vec![1,2,3,4]), seq.get_data().unwrap());
}

#[test]
fn test_encode_empty() {
    #[derive(Asn1Sequence)]
    struct TestSequence {}

    let seq = TestSequence{};
    assert_eq!(vec![0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_encode() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_encode_with_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x5, 
                    0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_encode_with_optional_component() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}

#[test]
fn test_encode_with_optional_without_value_component() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SequenceComponent2<Integer>
    }

    let p = Person{
        age: SequenceComponent2::new(),
    };

    assert_eq!(vec![0x30, 0x0], p.encode().unwrap());
}

#[should_panic(expected = "No value provided")]
#[test]
fn test_encode_without_give_required_values() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let p = Person{
        age: SequenceComponent2::new(),
    };
    p.encode().unwrap();
}

#[test]
fn test_encode_empty_with_application_tag() {
    #[derive(Asn1Sequence)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    let seq = TestSequence{};
    assert_eq!(vec![0x67, 0x2, 0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_decode_empty() {
    #[derive(Asn1Sequence)]
    struct Person {
    }

    let mut p = Person{};
    let consumed_octets = p.decode(&[0x30, 0x0]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[test]
fn test_decode_empty_with_application_tag() {
    #[derive(Asn1Sequence)]
    #[seq(application_tag = 7)]
    struct TestSequence { }

    let mut seq = TestSequence{};
    let consumed_octets = seq.decode(&[0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(4, consumed_octets);
}

#[test]
fn test_decode_empty_with_excesive_bytes() {
    #[derive(Asn1Sequence)]
    struct Person {}

    let mut p = Person{};
    let consumed_octets = p.decode(&[0x30, 0x0, 0xff, 0xff]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[should_panic (expected = "Invalid tag: Not valid tag for type")]
#[test]
fn test_decode_with_invalid_tag() {
    #[derive(Asn1Sequence)]
    struct Person {}

    let mut p = Person{};
    p.decode(&[0xff, 0x0]).unwrap();
}

#[test]
fn test_decode_with_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9]).unwrap();

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_decode_with_optional_with_bad_type_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_decode_with_optional_with_bad_number_type_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x1, 0xff]).unwrap();
}


#[test]
fn test_decode_with_optional_and_context_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, p.get_age());
}

#[should_panic(expected = "Invalid tag: Empty")]
#[test]
fn test_decode_with_optional_and_context_tag_bad_context_length() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x2, 0xa0, 0x0]).unwrap();
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_bad_decode_optional_context_tag_bad_context_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Invalid tag: Not valid tag for type")]
#[test]
fn test_bad_decode_optional_context_tag_bad_type_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, tag_number = 0)]
        age: SequenceComponent2<Integer>
    }

    let mut p = Person{
        age: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x3, 0xa0, 0x1, 0xee]).unwrap();
}

#[test]
fn test_decode_without_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        id: SequenceComponent2<Integer>,
        data: SequenceComponent2<OctetString>
    }

    let mut p = Person{
        id: SequenceComponent2::new(),
        data: SequenceComponent2::new(),
    };
    p.decode(&[0x30, 0x9, 
               INTEGER_TAG_NUMBER, 0x1, 0x9, 
               OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(&9, p.get_id().unwrap().value().unwrap());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], p.get_data().unwrap().value().unwrap());
}


#[test]
fn test_decode_with_optional() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(optional, tag_number = 0)]
        id: SequenceComponent2<Integer>,
        #[seq_comp(tag_number = 1)]
        data: SequenceComponent2<OctetString>
    }

    let mut seq = TestSequence{
        id: SequenceComponent2::new(),
        data: SequenceComponent2::new(),
    };
    seq.decode(&[0x30, 0x8, 
                 0xa1, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], seq.get_data().unwrap().value().unwrap());

}


#[test]
fn test_decode_with_optional_without_context_tag() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(optional)]
        id: SequenceComponent2<Integer>,
        data: SequenceComponent2<OctetString>
    }

    let mut seq = TestSequence{
        id: SequenceComponent2::new(),
        data: SequenceComponent2::new(),
    };

    seq.decode(&[0x30, 0x6, 
                 OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], seq.get_data().unwrap().value().unwrap());

}


#[should_panic (expected = "Invalid tag: Not valid tag for type")]
#[test]
fn test_decode_with_optional_and_context_tag_and_bad_type_tag() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(optional, tag_number = 0)]
        id: SequenceComponent2<Integer>
    }

    let mut seq = TestSequence{
        id: SequenceComponent2::new(),
    };
    seq.decode(&[0x30, 0x8, 
                 0xa0, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();
}
