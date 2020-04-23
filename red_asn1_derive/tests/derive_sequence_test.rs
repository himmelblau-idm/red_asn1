use red_asn1::*;

#[test]
fn test_define_simple() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        id: SeqField<Integer>,
        data: SeqField<OctetString>,
    }

    let mut seq = TestSequence::default();
    *seq.id = 9;
    *seq.data = vec![1, 2, 3, 4];

    assert_eq!(Integer::from(9), *seq.id);
    assert_eq!(OctetString::from(vec![1, 2, 3, 4]), *seq.data);
}

#[test]
fn test_define_with_not_all_sequence_fields() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        id: SeqField<Integer>,
        _flag: u32,
    }
}

#[test]
fn test_define_with_inner_sequenceof() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        id: SeqField<Integer>,
        attrs: SeqField<SequenceOf<Integer>>,
    }
}

#[test]
fn test_encode_empty() {
    #[derive(Sequence, Default)]
    struct TestSequence {}

    let seq = TestSequence {};
    assert_eq!(vec![0x30, 0x0], seq.encode());
}

#[test]
fn test_encode_empty_with_application_tag() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    let seq = TestSequence {};
    assert_eq!(vec![0x67, 0x2, 0x30, 0x0], seq.encode());
}

#[test]
fn test_encode() {
    #[derive(Sequence, Default)]
    struct Person {
        age: SeqField<Integer>,
    }

    let mut p = Person::default();
    *p.age = 9;

    assert_eq!(vec![0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9], p.encode());
}

#[test]
fn test_encode_with_context_tags() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(context_tag = 0)]
        age: SeqField<Integer>,
    }

    let mut p = Person::default();
    *p.age = 9;

    assert_eq!(
        vec![0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9],
        p.encode()
    );
}

#[test]
fn test_encode_with_optional_component() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>,
    }

    let mut p = Person {
        age: SeqField::default(),
    };
    *p.age = Integer::from(9);

    assert_eq!(9, *p.age);
}

#[test]
fn test_encode_with_optional_without_value_component() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>,
    }

    let p = Person {
        age: SeqField::default(),
    };

    assert_eq!(vec![0x30, 0x0], p.encode());
}

#[test]
fn test_encode_with_inner_sequence() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    #[derive(Sequence, Default)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>,
    }

    let mut seq = SuperTestSequence {
        inner: SeqField::default(),
    };

    *seq.inner = TestSequence::default();

    assert_eq!(vec![0x30, 0x4, 0x67, 0x2, 0x30, 0x0], seq.encode());
}

#[test]
fn test_encode_with_inner_sequenceof() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        attrs: SeqField<SequenceOf<Integer>>,
    }

    let mut seq = TestSequence {
        attrs: SeqField::default(),
    };
    let mut seqof_ints: SequenceOf<Integer> = SequenceOf::default();
    seqof_ints.push(Integer::from(1));

    *seq.attrs = seqof_ints;

    assert_eq!(
        vec![0x30, 0x5, 0x30, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x1],
        seq.encode()
    );
}

#[test]
fn test_decode_empty() {
    #[derive(Sequence, Default)]
    struct Person {}

    let (consumed_octets, _) = Person::decode(&[0x30, 0x0]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[test]
fn test_decode_empty_with_application_tag() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    let (consumed_octets, _) =
        TestSequence::decode(&[0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(4, consumed_octets);
}

#[test]
fn test_decode_empty_with_excesive_bytes() {
    #[derive(Sequence, Default)]
    struct Person {}

    let (consumed_octets, _) =
        Person::decode(&[0x30, 0x0, 0xff, 0xff]).unwrap();
    assert_eq!(2, consumed_octets);
}

#[should_panic(expected = "SequenceError(\"Person\", UnmatchedTag(Universal))")]
#[test]
fn test_decode_with_invalid_tag() {
    #[derive(Sequence, Default)]
    struct Person {}
    Person::decode(&[0xff, 0x0]).unwrap();
}

#[test]
fn test_decode_with_context_tags() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(context_tag = 0)]
        age: SeqField<Integer>,
    }

    let (_, p) =
        Person::decode(&[0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER, 0x1, 0x9])
            .unwrap();

    assert_eq!(9, *p.age);
}

#[should_panic(expected = "SequenceError(\"Person\", NoAllDataConsumed)")]
#[test]
fn test_decode_with_optional_with_bad_type_tag() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>,
    }

    Person::decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NoAllDataConsumed)")]
#[test]
fn test_decode_with_optional_with_bad_number_type_tag() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional)]
        age: SeqField<Integer>,
    }

    Person::decode(&[0x30, 0x1, 0xff]).unwrap();
}

#[test]
fn test_decode_with_optional_and_context_tag() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>,
    }

    let (_, p) = Person::decode(&[0x30, 0x0]).unwrap();
    assert_eq!(None, p.get_age());
}

#[should_panic(
    expected = "SequenceFieldError(\"Person\", \"age\", EmptyTag(Universal))"
)]
#[test]
fn test_decode_with_optional_and_context_tag_bad_context_length() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>,
    }

    Person::decode(&[0x30, 0x2, 0xa0, 0x0]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NoAllDataConsumed)")]
#[test]
fn test_bad_decode_optional_context_tag_bad_context_tag() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>,
    }

    Person::decode(&[0x30, 0x1, 0xee]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", UnmatchedTag(Universal))")]
#[test]
fn test_decode_bad_sequence_type_tag() {
    #[derive(Sequence, Default)]
    struct Person {}

    Person::decode(&[0x33, 0x0]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NotEnoughLengthOctects)")]
#[test]
fn test_decode_bad_sequence_length() {
    #[derive(Sequence, Default)]
    struct Person {}

    Person::decode(&[0x30, 0x81]).unwrap();
}

#[should_panic(
    expected = "SequenceError(\"Person\", UnmatchedTag(Application))"
)]
#[test]
fn test_decode_bad_sequence_application_tag() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 0)]
    struct Person {}

    Person::decode(&[0x61, 0x0]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NotEnoughLengthOctects)")]
#[test]
fn test_decode_sequence_application_tag_bad_length() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 0)]
    struct Person {}

    Person::decode(&[0x60, 0x81]).unwrap();
}

#[should_panic(
    expected = "SequenceFieldError(\"Person\", \"age\", UnmatchedTag(Universal))"
)]
#[test]
fn test_bad_decode_optional_context_tag_bad_type_tag() {
    #[derive(Sequence, Default)]
    struct Person {
        #[seq_field(optional, context_tag = 0)]
        age: SeqField<Integer>,
    }

    Person::decode(&[0x30, 0x3, 0xa0, 0x1, 0xee]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NoDataForLength)")]
#[test]
fn test_bad_decode_not_enough_data_for_length() {
    #[derive(Sequence, Default)]
    struct Person {}

    Person::decode(&[0x30, 0x1]).unwrap();
}

#[should_panic(expected = "SequenceError(\"Person\", NoDataForLength)")]
#[test]
fn test_bad_decode_not_enough_data_for_length_with_application_tag() {
    #[derive(Sequence, Default)]
    #[seq(application_tag = 0)]
    struct Person {}

    Person::decode(&[0x60, 0x1]).unwrap();
}

#[test]
fn test_decode_without_context_tags() {
    #[derive(Sequence, Default)]
    struct Person {
        id: SeqField<Integer>,
        data: SeqField<OctetString>,
    }

    let (_, p) = Person::decode(&[
        0x30,
        0x9,
        INTEGER_TAG_NUMBER,
        0x1,
        0x9,
        OCTET_STRING_TAG_NUMBER,
        0x4,
        0x1,
        0x2,
        0x3,
        0x4,
    ])
    .unwrap();

    assert_eq!(9, *p.id);
    assert_eq!(vec![0x1, 0x2, 0x3, 0x4], *p.data);
}

#[test]
fn test_decode_with_optional() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>,
        #[seq_field(context_tag = 1)]
        data: SeqField<OctetString>,
    }

    let (_, seq) = TestSequence::decode(&[
        0x30,
        0x8,
        0xa1,
        0x6,
        OCTET_STRING_TAG_NUMBER,
        0x4,
        0x1,
        0x2,
        0x3,
        0x4,
    ])
    .unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(vec![0x1, 0x2, 0x3, 0x4], *seq.data);
}

#[test]
fn test_decode_with_optional_without_context_tag() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        #[seq_field(optional)]
        id: SeqField<Integer>,
        data: SeqField<OctetString>,
    }

    let (_, seq) = TestSequence::decode(&[
        0x30,
        0x6,
        OCTET_STRING_TAG_NUMBER,
        0x4,
        0x1,
        0x2,
        0x3,
        0x4,
    ])
    .unwrap();

    assert_eq!(None, seq.get_id());
    assert_eq!(vec![0x1, 0x2, 0x3, 0x4], *seq.data);
}

#[should_panic(
    expected = "SequenceFieldError(\"TestSequence\", \"id\", UnmatchedTag(Universal))"
)]
#[test]
fn test_decode_with_optional_and_context_tag_and_bad_type_tag() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>,
    }

    TestSequence::decode(&[
        0x30,
        0x8,
        0xa0,
        0x6,
        OCTET_STRING_TAG_NUMBER,
        0x4,
        0x1,
        0x2,
        0x3,
        0x4,
    ])
    .unwrap();
}

#[test]
fn test_decode_with_inner_sequence() {
    #[derive(Sequence, Debug, PartialEq, Default)]
    #[seq(application_tag = 7)]
    struct TestSequence {}

    #[derive(Sequence, Default)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>,
    }

    let (_, seq) =
        SuperTestSequence::decode(&[0x30, 0x4, 0x67, 0x2, 0x30, 0x0]).unwrap();
    assert_eq!(TestSequence {}, *seq.inner);
}

#[test]
fn test_decode_unsetting_optional_value() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        #[seq_field(optional, context_tag = 0)]
        id: SeqField<Integer>,
    }

    let (_, seq) = TestSequence::decode(&[0x30, 0x0]).unwrap();

    assert_eq!(None, seq.get_id());
}

#[test]
fn test_decode_with_inner_sequenceof() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        pub attrs: SeqField<SequenceOf<Integer>>,
    }

    let (_, seq) = TestSequence::decode(&[
        0x30,
        0x5,
        0x30,
        0x3,
        INTEGER_TAG_NUMBER,
        0x1,
        0x1,
    ])
    .unwrap();

    let seqof_ints = seq.attrs;
    assert_eq!(1, seqof_ints.len());
    assert_eq!(&Integer::from(1), &seqof_ints[0]);
}

#[should_panic(
    expected = "SequenceFieldError(\"TestSequence\", \"id\", EmptyTag(Context))"
)]
#[test]
fn test_decode_without_required_value() {
    #[derive(Sequence, Default)]
    struct TestSequence {
        #[seq_field(context_tag = 0)]
        id: SeqField<Integer>,
    }

    TestSequence::decode(&[0x30, 0x0]).unwrap();
}

#[should_panic(
    expected = "SequenceFieldError(\"SuperTestSequence\", \"inner\", SequenceFieldError(\"TestSequence\", \"id\", EmptyTag(Context)))"
)]
#[test]
fn test_decode_without_required_value_with_inner_sequence() {
    #[derive(Sequence, Debug, PartialEq, Default)]
    struct TestSequence {
        #[seq_field(context_tag = 0)]
        id: SeqField<Integer>,
    }

    #[derive(Sequence, Default)]
    struct SuperTestSequence {
        inner: SeqField<TestSequence>,
    }

    SuperTestSequence::decode(&[0x30, 0x2, 0x30, 0x0]).unwrap();
}
