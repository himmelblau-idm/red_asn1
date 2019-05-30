use syn::*;

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
