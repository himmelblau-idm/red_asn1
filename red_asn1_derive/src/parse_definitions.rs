use syn::{Ident, PathSegment};
use proc_macro2::TokenStream;

pub struct SequenceDefinition {
    pub name: Ident,
    pub application_tag_number: Option<u8>,
    pub fields: Vec<FieldDefinition>
}

pub struct FieldDefinition {
    pub id: Ident,
    pub kind: PathSegment,
    pub optional: bool,
    pub context_tag_number: Option<u8>
}


impl FieldDefinition {
    pub fn encoder_name(&self) -> Ident {
        let concatenated = format!("encode_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

    pub fn decoder_name(&self) -> Ident {
        let concatenated = format!("decode_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

}

pub struct FieldCode {
    pub encoder: TokenStream,
    pub decoder: TokenStream
}


pub struct SequenceInnerCallsCode {
    pub encode_calls: TokenStream,
    pub decode_calls: TokenStream,
    pub components_unit_functions: TokenStream
}


