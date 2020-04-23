use syn::*;
use proc_macro2::TokenStream;

pub static SEQUENCE_COMPONENT_TYPE_NAME: &str = "SeqField";
pub static ASN1_SEQ_ATTR: &str = "seq";
pub static ASN1_SEQ_FIELD_ATTR: &str = "seq_field";
pub static OPTIONAL_ATTR: &str = "optional";
pub static TAG_NUMBER_ATTR: &str = "context_tag";
pub static APPLICATION_TAG_ATTR: &str = "application_tag";

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


