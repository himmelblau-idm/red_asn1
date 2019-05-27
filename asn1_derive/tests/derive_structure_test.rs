extern crate asn1;
extern crate asn1_derive;


use asn1::*;
use asn1_derive::Asn1Sequence;

#[test]
fn simple_sequence() {

    #[derive(Asn1Sequence)]
    struct Person {
        age: SequenceComponent2<Integer>
    }

    let mut p = Person::new();
    p.set_age(Integer::new(9));

    assert_eq!(&Integer::new(9), p.get_age().unwrap());
}