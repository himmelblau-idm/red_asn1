use red_asn1::*;

#[test]
fn test_define_simple() {

    #[derive(Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::default(),
        data: SeqField::default()
    };
    seq.set_id(Integer::new(9));
    seq.set_data(OctetString::new(vec![1,2,3,4]));

    assert_eq!(&Integer::new(9), seq.get_id().unwrap());
    assert_eq!(&OctetString::new(vec![1,2,3,4]), seq.get_data().unwrap());
}

#[test]
fn test_define_with_not_all_sequence_fields() {
    #[derive(Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        _flag: u32
    }
}

#[test]
fn test_define_with_inner_sequenceof() {
    #[derive(Sequence)]
    struct TestSequence {
        id: SeqField<Integer>,
        attrs: SeqField<SequenceOf<Integer>>
    }
}

#[test]
fn test_encode_empty() {
    #[derive(Sequence)]
    struct TestSequence {}

    let seq = TestSequence{};
    assert_eq!(vec![0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_encode_empty_with_application_tag() {
    #[derive(Sequence)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    let seq = TestSequence{};
    assert_eq!(vec![0x67, 0x2, 0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_encode() {

    #[derive(Sequence)]
    struct Person {
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_encode_with_context_tags() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(vec![0x30, 0x5, 
                    0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode().unwrap());
}

#[test]
fn test_encode_with_optional_component() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.set_age(Integer::new(9));

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}

#[test]
fn test_encode_with_optional_without_value_component() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>
    }

    let p = Person{
        age: SeqField::default(),
    };

    assert_eq!(vec![0x30, 0x0], p.encode().unwrap());
}

#[should_panic(expected = "Person::age => No value provided")]
#[test]
fn test_encode_without_give_required_values() {

    #[derive(Sequence)]
    struct Person {
        age: SeqField<Integer>
    }

    let p = Person{
        age: SeqField::default(),
    };
    p.encode().unwrap();
}

#[test]
fn test_encode_with_inner_sequence() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    #[derive(Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::default()
    };

    seq.set_inner(TestSequence::default());

    assert_eq!(vec![0x30, 0x4, 0x67, 0x2, 0x30, 0x0], seq.encode().unwrap());
}

#[test]
fn test_encode_with_inner_sequenceof() {
    #[derive(Sequence)]
    struct TestSequence {
        attrs: SeqField<SequenceOf<Integer>>
    }

    let mut seq = TestSequence{ attrs: SeqField::default()};
    let mut seqof_ints: SequenceOf<Integer> = SequenceOf::default();
    seqof_ints.push(Integer::new(1));

    seq.set_attrs(seqof_ints);

    assert_eq!(vec![0x30, 0x5, 0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x1],
               seq.encode().unwrap());
}

#[test]
fn test_decode_empty() {
    #[derive(Sequence)]
    struct Person {
    }

    let mut p = Person{};
    let consumed_octets = p.decode(&[0x30, 0x0]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[test]
fn test_decode_empty_with_application_tag() {
    #[derive(Sequence)]
    #[seq(application_tag = 7)]
    struct TestSequence { }

    let mut seq = TestSequence{};
    let consumed_octets = seq.decode(&[0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(4, consumed_octets);
}

#[test]
fn test_decode_empty_with_excesive_bytes() {
    #[derive(Sequence)]
    struct Person {}

    let mut p = Person{};
    let consumed_octets = p.decode(&[0x30, 0x0, 0xff, 0xff]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
#[test]
fn test_decode_with_invalid_tag() {
    #[derive(Sequence)]
    struct Person {}

    let mut p = Person{};
    p.decode(&[0xff, 0x0]).unwrap();
}

#[test]
fn test_decode_with_context_tags() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9]).unwrap();

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_decode_with_optional_with_bad_type_tag() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Invalid value: Not all octects were consumed")]
#[test]
fn test_decode_with_optional_with_bad_number_type_tag() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x1, 0xff]).unwrap();
}


#[test]
fn test_decode_with_optional_and_context_tag() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, p.get_age());
}

#[should_panic(expected = "Invalid universal tag: Empty")]
#[test]
fn test_decode_with_optional_and_context_tag_bad_context_length() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x2, 0xa0, 0x0]).unwrap();
}

#[should_panic(expected =  "Person => Invalid value: Not all octects were consumed")]
#[test]
fn test_bad_decode_optional_context_tag_bad_context_tag() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Person => Invalid universal tag: Not match with expected tag")]
#[test]
fn test_decode_bad_sequence_type_tag() {

    #[derive(Sequence)]
    struct Person {
    }

    let mut p = Person{};
    p.decode(&[0x33, 0x0]).unwrap();
}

#[should_panic(expected =  "Person => Invalid length: Invalid length of length")]
#[test]
fn test_decode_bad_sequence_length() {

    #[derive(Sequence)]
    struct Person {
    }

    let mut p = Person{};
    p.decode(&[0x30, 0x81]).unwrap();
}

#[should_panic(expected =  "Person => Invalid application tag: Not match with expected tag")]
#[test]
fn test_decode_bad_sequence_application_tag() {

    #[derive(Sequence)]
    #[seq(application_tag = 0)]
    struct Person {
    }

    let mut p = Person{};
    p.decode(&[0x61, 0x0]).unwrap();
}

#[should_panic(expected =  "Person => Invalid length: Invalid length of length")]
#[test]
fn test_decode_sequence_application_tag_bad_length() {

    #[derive(Sequence)]
    #[seq(application_tag = 0)]
    struct Person {
    }

    let mut p = Person{};
    p.decode(&[0x60, 0x81]).unwrap();
}

#[should_panic(expected =  "Person::age => Invalid universal tag: Not match with expected tag")]
#[test]
fn test_bad_decode_optional_context_tag_bad_type_tag() {

    #[derive(Sequence)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>
    }

    let mut p = Person{
        age: SeqField::default(),
    };
    p.decode(&[0x30, 0x3, 0xa0, 0x1, 0xee]).unwrap();
}

#[should_panic(expected =  "Person => Invalid value: Not enough data for length")]
#[test]
fn test_bad_decode_not_enough_data_for_length () {

    #[derive(Sequence)]
    struct Person {
    }

    let mut p = Person{
    };
    p.decode(&[0x30, 0x1]).unwrap();
}

#[should_panic(expected =  "Person => Invalid value: Not enough data for length")]
#[test]
fn test_bad_decode_not_enough_data_for_length_with_application_tag () {

    #[derive(Sequence)]
    #[seq(application_tag = 0)]
    struct Person {
    }

    let mut p = Person{
    };
    p.decode(&[0x60, 0x1]).unwrap();
}

#[test]
fn test_decode_without_context_tags() {

    #[derive(Sequence)]
    struct Person {
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut p = Person{
        id: SeqField::default(),
        data: SeqField::default(),
    };
    p.decode(&[0x30, 0x9, 
               INTEGER_TAG_NUMBER, 0x1, 0x9, 
               OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(&9, p.get_id().unwrap().value().unwrap());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], p.get_data().unwrap().value().unwrap());
}


#[test]
fn test_decode_with_optional() {
    #[derive(Sequence)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>,
        #[seq_field(context_tag = 1)]
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::default(),
        data: SeqField::default(),
    };
    seq.decode(&[0x30, 0x8, 
                 0xa1, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], seq.get_data().unwrap().value().unwrap());

}


#[test]
fn test_decode_with_optional_without_context_tag() {
    #[derive(Sequence)]
    struct TestSequence {
        #[seq_field(optional)]
        id: SeqField<Integer>,
        data: SeqField<OctetString>
    }

    let mut seq = TestSequence{
        id: SeqField::default(),
        data: SeqField::default(),
    };

    seq.decode(&[0x30, 0x6, 
                 OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(&vec![0x1, 0x2, 0x3, 0x4], seq.get_data().unwrap().value().unwrap());

}


#[should_panic (expected = "TestSequence::id => Invalid universal tag: Not match with expected tag")]
#[test]
fn test_decode_with_optional_and_context_tag_and_bad_type_tag() {
    #[derive(Sequence)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::default(),
    };
    seq.decode(&[0x30, 0x8, 
                 0xa0, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();
}


#[test]
fn test_decode_with_inner_sequence() {
    #[derive(Sequence, Debug, PartialEq, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    #[derive(Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::default()
    };

    seq.decode(&[0x30, 0x4, 0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(&TestSequence{}, seq.get_inner().unwrap());
}

#[test]
fn test_decode_unsetting_optional_value() {
    #[derive(Sequence)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::default()
    };

    seq.set_id(Integer::new(9));
    seq.decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, seq.get_id());

}

#[test]
fn test_decode_with_inner_sequenceof() {
    #[derive(Sequence)]
    struct TestSequence {
        attrs: SeqField<SequenceOf<Integer>>
    }

    let mut seq = TestSequence{ attrs: SeqField::default()};

    seq.decode(&[0x30, 0x5, 0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x1]).unwrap();

    let seqof_ints = seq.get_attrs().unwrap();
    assert_eq!(1, seqof_ints.len());
    assert_eq!(&Integer::new(1), &seqof_ints[0]);
}


#[should_panic (expected = "TestSequence::id => Invalid context tag: Empty")]
#[test]
fn test_decode_without_required_value() {
    #[derive(Sequence)]
    struct TestSequence {
        #[seq_field(context_tag = 0)]
        id: SeqField<Integer>
    }

    let mut seq = TestSequence{
        id: SeqField::default(),
    };
    seq.decode(&[0x30, 0x0]).unwrap();

}

#[should_panic (expected = "SuperTestSequence::inner => TestSequence::id => Invalid context tag: Empty")]
#[test]
fn test_decode_without_required_value_with_inner_sequence() {
    #[derive(Sequence, Debug, PartialEq, Default)]
    struct TestSequence {
        #[seq_field(context_tag = 0)]
        id: SeqField<Integer>
    }

    #[derive(Sequence)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>
    }

    let mut seq = SuperTestSequence{
        inner: SeqField::default()
    };

    seq.decode(&[0x30, 0x2, 0x30, 0x0]).unwrap();
}