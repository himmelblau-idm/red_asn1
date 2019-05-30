use syn::*;
use proc_macro2::TokenStream;

pub static SEQUENCE_COMPONENT_TYPE_NAME: &str = "SequenceComponent2";
pub static ASN1_SEQ_ATTR: &str = "seq";
pub static ASN1_SEQ_COMP_ATTR: &str = "seq_comp";
pub static OPTIONAL_ATTR: &str = "optional";
pub static TAG_NUMBER_ATTR: &str = "tag_number";
pub static APPLICATION_TAG_ATTR: &str = "application_tag";

pub struct SequenceDefinition {
    pub name: Ident,
    pub application_tag_number: Option<u8>,
    pub components: Vec<ComponentDefinition>
}

pub struct ComponentDefinition {
    pub id: Ident,
    pub kind: Ident,
    pub optional: bool,
    pub context_tag_number: Option<u8>
}


impl ComponentDefinition {
    pub fn encoder_name(&self) -> Ident {
        let concatenated = format!("encode_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

    pub fn decoder_name(&self) -> Ident {
        let concatenated = format!("decode_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

    pub fn getter_name(&self) -> Ident {
        let concatenated = format!("get_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

    pub fn setter_name(&self) -> Ident {
        let concatenated = format!("set_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }

    pub fn unsetter_name(&self) -> Ident {
        let concatenated = format!("unset_{}", self.id);
        return Ident::new(&concatenated, self.id.span());
    }
}

pub struct ComponentCode {
    pub getter: TokenStream,
    pub setter: TokenStream,
    pub unsetter: TokenStream,
    pub encoder: TokenStream,
    pub decoder: TokenStream
}


pub struct SequenceInnerCallsCode {
    pub encode_calls: TokenStream,
    pub decode_calls: TokenStream,
    pub new_fields: TokenStream,
    pub components_unit_functions: TokenStream
}


