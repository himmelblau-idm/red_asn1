//! A little library to encode/decode ASN1 DER
//! 
//! # Example
//! 
//! ```
//! /*
//! Person ::= [APPLICATION 1] SEQUENCE {
//!     name:       [0] GeneralString,
//!     age:        [1] Integer,
//!     address:    [2] GeneralString OPTIONAL,
//! }
//! */
//! 
//! use red_asn1::*;
//! 
//! #[derive(Sequence, Default)]
//! #[seq(application_tag = 1)]
//! struct Person {
//!     #[seq_field(context_tag = 0)]
//!     pub name: GeneralString,
//!     #[seq_field(context_tag = 1)]
//!     pub age: Integer,
//!     #[seq_field(context_tag = 2)]
//!     pub address: Option<GeneralString>
//! }
//! 
//! let john = Person{
//!     name: GeneralString::from("John").into(),
//!     age: Integer::from(18).into(),
//!     address: None
//! };
//! 
//! assert_eq!(
//!     vec![
//!         0x61, 0xf, 0x30, 0xd,
//!         0xa0, 0x6, 0x1b, 0x4, 0x4a, 0x6f, 0x68, 0x6e, // "John"
//!         0xa1, 0x3, 0x2, 0x1, 0x12 // 18
//!     ]
//!     , john.encode()
//! );
//! 
//! let (_, rachel) = Person::decode(&[
//!     0x61, 0x1b, 0x30, 0x19,
//!     0xa0, 0x8, 0x1b, 0x6, 0x52, 0x61, 0x63, 0x68, 0x65, 0x6c, // "Rachel"
//!     0xa1, 0x3, 0x2, 0x1, 0x1e, // 30
//!     0xa2, 0x8, 0x1b, 0x6, 0x48, 0x61, 0x77, 0x61, 0x69, 0x69 // "Hawaii"
//! ]).unwrap();
//! 
//! assert_eq!("Rachel", rachel.name);
//! assert_eq!(30, rachel.age);
//! assert_eq!(Some("Hawaii".to_string()), rachel.address);
//! 
//! ```
//! 


mod error;
pub use error::*;
pub use error::Error;

mod tag;
pub use tag::*;

mod traits;
pub use traits::*;

mod types;
pub use types::*;

mod length;
pub use length::{build_length, parse_length};

pub use red_asn1_derive::*;
