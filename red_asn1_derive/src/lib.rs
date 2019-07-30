#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

mod parse_error;
mod parser;
mod parse_definitions;
mod code_components;

use parser::*;
use code_components::*;

#[proc_macro_derive(Asn1Sequence, attributes(seq, seq_comp))]
pub fn sequence_macro_derive(input: TokenStream) -> TokenStream {    
    let ast = parse_macro_input!(input as DeriveInput);

    let sequence_definition = extract_sequence_definition(&ast).unwrap();
    let sequence_inner_calls = code_sequence_inner_calls(&sequence_definition);
    let sequence_code = code_sequence(&sequence_definition, &sequence_inner_calls);

    return TokenStream::from(sequence_code);
}

