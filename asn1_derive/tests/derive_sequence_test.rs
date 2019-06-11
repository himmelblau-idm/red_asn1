extern crate asn1;
extern crate asn1_derive;


use asn1::*;
use asn1_derive::Asn1Sequence;

#[test]
fn test_define_simple() {

    #[derive(Asn1Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::new(),
        data: SeqField::new()
    };
    seq.set_id(Integer::new(9));
    seq.set_data(OctetString::new(vec![1,2,3,4]));

    assert_eq!(&Integer::new(9), seq.get_id().unwrap());
    assert_eq!(&OctetString::new(vec![1,2,3,4]), seq.get_data().unwrap());
}

#[test]
fn test_define_with_not_all_sequence_fields() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        _flag: u32
    }
}

#[test]
fn test_define_with_inner_sequenceof() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        attrs: SeqField<SequenceOf<Integer>>
    }
}

#[test]
fn test_encode_empty() {
    #[derive(Asn1Sequence)]
    struct TestSequence {}

    let seq = TestSequence{};
    assert_eq!(vec![0x30, 0x0], seq.encode().unwrap());
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
fn test_encode() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_encode_with_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
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
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}

#[test]
fn test_encode_with_optional_without_value_component() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SeqField<Integer>
    }

    let p = Person{
        age: SeqField::new(),
    };

    assert_eq!(vec![0x30, 0x0], p.encode().unwrap());
}

#[should_panic(expected = "Error in Person::age => No value provided")]
#[test]
fn test_encode_without_give_required_values() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SeqField<Integer>
    }

    let p = Person{
        age: SeqField::new(),
    };
    p.encode().unwrap();
}

#[test]
fn test_encode_with_inner_sequence() {
    #[derive(Asn1Sequence)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    impl Asn1InstanciableObject for TestSequence {
        fn new_default() -> Self {
            return TestSequence{};
        }
    }

    #[derive(Asn1Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::new()
    };

    seq.set_inner(TestSequence::new_default());

    assert_eq!(vec![0x30, 0x4, 0x67, 0x2, 0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_encode_with_inner_sequenceof() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        attrs: SeqField<SequenceOf<Integer>>
    }

    let mut seq = TestSequence{ attrs: SeqField::new()};
    let mut seqof_ints: SequenceOf<Integer> = SequenceOf::new();
    seqof_ints.push(Integer::new(1));

    seq.set_attrs(seqof_ints);

    assert_eq!(vec![0x30, 0x5, 0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x1],
               seq.encode().unwrap());
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
        #[seq_comp(context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
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
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_decode_with_optional_with_bad_number_type_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x1, 0xff]).unwrap();
}


#[test]
fn test_decode_with_optional_and_context_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, p.get_age());
}

#[should_panic(expected = "Invalid tag: Empty")]
#[test]
fn test_decode_with_optional_and_context_tag_bad_context_length() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x2, 0xa0, 0x0]).unwrap();
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_bad_decode_optional_context_tag_bad_context_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Error in Person::age => Invalid tag: Not valid tag for type")]
#[test]
fn test_bad_decode_optional_context_tag_bad_type_tag() {

    #[derive(Asn1Sequence)]
    struct Person {
        #[seq_comp(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::new(),
    };
    p.decode(&[0x30, 0x3, 0xa0, 0x1, 0xee]).unwrap();
}

#[test]
fn test_decode_without_context_tags() {

    #[derive(Asn1Sequence)]
    struct Person {
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut p = Person{
        id: SeqField::new(),
        data: SeqField::new(),
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
        #[seq_comp(optional, context_tag = 0)]
        id: SeqField<Integer>,
        #[seq_comp(context_tag = 1)]
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::new(),
        data: SeqField::new(),
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
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::new(),
        data: SeqField::new(),
    };

    seq.decode(&[0x30, 0x6, 
                 OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], seq.get_data().unwrap().value().unwrap());

}


#[should_panic (expected = "Error in TestSequence::id => Invalid tag: Not valid tag for type")]
#[test]
fn test_decode_with_optional_and_context_tag_and_bad_type_tag() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(optional, context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::new(),
    };
    seq.decode(&[0x30, 0x8, 
                 0xa0, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();
}


#[test]
fn test_decode_with_inner_sequence() {
    #[derive(Asn1Sequence, Debug, PartialEq)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    impl Asn1InstanciableObject for TestSequence {
        fn new_default() -> Self {
            return TestSequence{};
        }
    }

    #[derive(Asn1Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::new()
    };

    seq.decode(&[0x30, 0x4, 0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(&TestSequence{}, seq.get_inner().unwrap());
}

#[test]
fn test_decode_unsetting_optional_value() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(optional, context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::new()
    };

    seq.set_id(Integer::new(9));
    seq.decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, seq.get_id());

}

#[test]
fn test_decode_with_inner_sequenceof() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        attrs: SeqField<SequenceOf<Integer>>
    }

    let mut seq = TestSequence{ attrs: SeqField::new()};

    seq.decode(&[0x30, 0x5, 0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x1]).unwrap();

    let seqof_ints = seq.get_attrs().unwrap();
    assert_eq!(1, seqof_ints.len());
    assert_eq!(&Integer::new(1), &seqof_ints[0]);
}


#[should_panic (expected = "Error in TestSequence::id => Invalid tag: Empty tag for context")]
#[test]
fn test_decode_without_required_value() {
    #[derive(Asn1Sequence)]
    struct TestSequence {
        #[seq_comp(context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::new(),
    };
    seq.decode(&[0x30, 0x0]).unwrap();

}

#[should_panic (expected = "Error in SuperTestSequence::inner => Error in TestSequence::id => Invalid tag: Empty tag for context")]

#[test]
fn test_decode_without_required_value_with_inner_sequence() {
    #[derive(Asn1Sequence, Debug, PartialEq)]
    struct TestSequence {
        #[seq_comp(context_tag = 0)]
        id: SeqField<Integer>
    }

    impl Asn1InstanciableObject for TestSequence {
        fn new_default() -> Self {
            return TestSequence{
                id: SeqField::new()
            };
        }
    }

    #[derive(Asn1Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::new()
    };

    seq.decode(&[0x30, 0x2, 0x30, 0x0]).unwrap();
}