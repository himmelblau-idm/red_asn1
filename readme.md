# Red ASN1
A little library to encode/decode ASN1 DER

## Example

Encoding and decoding: 
```rust
/*
Person ::= [APPLICATION 1] SEQUENCE {
    name:       [0] GeneralString,
    age:        [1] Integer,
    address:    [2] GeneralString OPTIONAL,
}
*/

use red_asn1::*;

#[derive(Sequence)]
#[seq(application_tag = 1)]
struct Person {
    #[seq_field(context_tag = 0)]
    name: SeqField<GeneralString>,
    #[seq_field(context_tag = 1)]
    age: SeqField<Integer>,
    #[seq_field(context_tag = 2, optional)]
    address: SeqField<GeneralString>
}

let mut person = Person{
    name: GeneralString::from("John").into(),
    age: Integer::from(18).into(),
    address: SeqField::default()
};

assert_eq!(
    vec![
        0x61, 0xf, 0x30, 0xd,
        0xa0, 0x6, 0x1b, 0x4, 0x4a, 0x6f, 0x68, 0x6e, // "John"
        0xa1, 0x3, 0x2, 0x1, 0x12 // 18
    ]
    , person.encode().unwrap()
);

person.decode(&[
    0x61, 0x1b, 0x30, 0x19,
    0xa0, 0x8, 0x1b, 0x6, 0x52, 0x61, 0x63, 0x68, 0x65, 0x6c, // "Rachel"
    0xa1, 0x3, 0x2, 0x1, 0x1e, // 30
    0xa2, 0x8, 0x1b, 0x6, 0x48, 0x61, 0x77, 0x61, 0x69, 0x69 // "Hawaii"
]).unwrap();

assert_eq!("Rachel", person.get_name().unwrap().value().unwrap());
assert_eq!(30, person.get_age().unwrap().value().unwrap());
assert_eq!("Hawaii", person.get_address().unwrap().value().unwrap());

```