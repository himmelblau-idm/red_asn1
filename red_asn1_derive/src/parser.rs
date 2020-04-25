use super::parse_definitions::*;
use super::parse_error::{ParseError, ParseResult};
use syn::*;

pub fn extract_sequence_definition(
    ast: &DeriveInput,
) -> ParseResult<SequenceDefinition> {
    if let Data::Struct(data_struct) = &ast.data {
        let name = &ast.ident;
        let fields = extract_components_definitions(data_struct)?;
        let mut application_tag_number: Option<u8> = None;

        match parse_sequence_attrs(&ast.attrs) {
            Ok(tag_number) => {
                application_tag_number = tag_number;
            }
            Err(parse_error) => match parse_error {
                ParseError::NotFoundAttributeTag => {}
                _ => {
                    return Err(parse_error);
                }
            },
        }

        return Ok(SequenceDefinition {
            name: name.clone(),
            application_tag_number: application_tag_number,
            fields,
        });
    } else {
        return Err(ParseError::NotStruct);
    }
}

fn extract_components_definitions(
    data_struct: &DataStruct,
) -> ParseResult<Vec<FieldDefinition>> {
    if let Fields::Named(fields_named) = &data_struct.fields {
        return parse_structure_fields(fields_named);
    }
    unreachable!()
}

fn parse_structure_fields(
    fields: &FieldsNamed,
) -> ParseResult<Vec<FieldDefinition>> {
    let mut components_definitions: Vec<FieldDefinition> = Vec::new();
    for field in fields.named.iter() {
        match parse_structure_field(&field) {
            Ok(component_definition) => {
                components_definitions.push(component_definition);
            }
            Err(parse_error) => {
                match parse_error {
                    ParseError::InvalidFieldType => {}
                    _ => {
                        return Err(parse_error);
                    }
                };
            }
        };
    }
    return Ok(components_definitions);
}

fn parse_structure_field(
    field: &Field,
) -> ParseResult<FieldDefinition> {
    let field_name;
    if let Some(name) = &field.ident {
        field_name = name;
    } else {
        unreachable!();
    }

    let field_type = extract_field_type(&field.ty)?;
    let mut context_tag_number = None;
    let mut optional = false;

    match parse_field_attrs(&field.attrs) {
        Ok((opt, tag_number)) => {
            optional = opt;
            context_tag_number = tag_number;
        }
        Err(parse_error) => match parse_error {
            ParseError::NotFoundAttributeTag => {}
            _ => {
                return Err(parse_error);
            }
        },
    }

    return Ok(FieldDefinition {
        id: field_name.clone(),
        kind: field_type,
        optional,
        context_tag_number,
    });
}

fn extract_field_type(field_type: &Type) -> ParseResult<PathSegment> {
    if let Type::Path(path) = &field_type {
        if path.path.segments[0].ident == SEQUENCE_COMPONENT_TYPE_NAME {
            if let PathArguments::AngleBracketed(brack_argument) =
                &path.path.segments[0].arguments
            {
                if let GenericArgument::Type(ty) = &brack_argument.args[0] {
                    if let Type::Path(path) = ty {
                        return Ok(path.path.segments[0].clone());
                    }
                }
            }
        } else {
            return Err(ParseError::InvalidFieldType);
        }
    }
    unreachable!();
}

fn parse_field_attrs(
    attrs: &Vec<Attribute>,
) -> ParseResult<(bool, Option<u8>)> {
    for attr in attrs {
        if attr.path.segments.len() > 0
            && attr.path.segments[0].ident == ASN1_SEQ_FIELD_ATTR
        {
            return parse_component_attr(attr);
        }
    }
    return Err(ParseError::NotFoundAttributeTag);
}

fn parse_component_attr(
    attr: &Attribute,
) -> ParseResult<(bool, Option<u8>)> {
    let mut optional = false;
    let mut tag_number = None;

    if let Ok(Meta::List(ref meta)) = attr.parse_meta() {
        let subattrs: Vec<syn::NestedMeta> =
            meta.nested.iter().cloned().collect();
        for subattr in subattrs {
            if let syn::NestedMeta::Meta(ref a) = subattr {
                match a {
                    Meta::NameValue(name_value) => {
                        if name_value.ident == TAG_NUMBER_ATTR {
                            match name_value.lit {
                                syn::Lit::Int(ref value) => {
                                    let int_value = value.value();
                                    if int_value >= 256 {
                                        return Err(ParseError::InvalidTagNumberValue);
                                    }
                                    tag_number = Some(int_value as u8);
                                }
                                _ => {
                                    return Err(ParseError::InvalidTagNumberValue);
                                }
                            }
                        } else {
                            return Err(ParseError::UnknownAttribute);
                        }
                    }
                    Meta::Word(ident) => {
                        if ident == OPTIONAL_ATTR {
                            optional = true;
                        } else {
                            return Err(ParseError::UnknownAttribute);
                        }
                    }
                    _ => {
                        return Err(ParseError::UnknownAttribute);
                    }
                };
            }
        }
    }

    return Ok((optional, tag_number));
}

fn parse_sequence_attrs(
    attrs: &Vec<Attribute>,
) -> ParseResult<Option<u8>> {
    for attr in attrs {
        if attr.path.segments.len() > 0
            && attr.path.segments[0].ident == ASN1_SEQ_ATTR
        {
            return parse_seq_attr(attr);
        }
    }
    return Err(ParseError::NotFoundAttributeTag);
}

fn parse_seq_attr(attr: &Attribute) -> ParseResult<Option<u8>> {
    let mut tag_number = None;

    if let Ok(Meta::List(ref meta)) = attr.parse_meta() {
        let subattrs: Vec<syn::NestedMeta> =
            meta.nested.iter().cloned().collect();
        for subattr in subattrs {
            if let syn::NestedMeta::Meta(ref a) = subattr {
                match a {
                    Meta::NameValue(name_value) => {
                        if name_value.ident == APPLICATION_TAG_ATTR {
                            match name_value.lit {
                                syn::Lit::Int(ref value) => {
                                    let int_value = value.value();
                                    if int_value >= 256 {
                                        return Err(ParseError::InvalidTagNumberValue);
                                    }
                                    tag_number = Some(int_value as u8);
                                }
                                _ => {
                                    return Err(ParseError::InvalidTagNumberValue);
                                }
                            }
                        } else {
                            return Err(ParseError::UnknownAttribute);
                        }
                    }
                    _ => {
                        return Err(ParseError::UnknownAttribute);
                    }
                };
            }
        }
    }

    return Ok(tag_number);
}
