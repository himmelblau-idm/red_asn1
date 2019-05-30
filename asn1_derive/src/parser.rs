use super::parse_error::*;
use syn::*;
use super::parse_definitions::*;

pub fn extract_sequence_definition(ast: &DeriveInput) -> ParseComponentResult<SequenceDefinition> {
    if let Data::Struct(data_struct) = &ast.data {
        let name = &ast.ident;
        let components = extract_components_definitions(data_struct)?;
        let mut application_tag_number: Option<u8> = None;
        
        match parse_sequence_attrs(&ast.attrs) {
            Ok(tag_number) => {
                application_tag_number = tag_number;
            },
            Err(parse_error) => {
                match parse_error.kind() {
                    ParseComponentErrorKind::NotFoundAttributeTag => {
                    },
                    _ => {
                        return Err(parse_error);
                    }
                }
            }
        }

        return Ok(SequenceDefinition{
            name: name.clone(),
            application_tag_number: application_tag_number,
            components: components
        });
    } else {
        return Err(ParseComponentErrorKind::NotStruct)?;
    }
}

fn extract_components_definitions(data_struct : &DataStruct) -> ParseComponentResult<Vec<ComponentDefinition>> {
    if let Fields::Named(fields_named) = &data_struct.fields {
        return parse_structure_fields(fields_named);
    }
    unreachable!()
}

fn parse_structure_fields(fields : &FieldsNamed) -> ParseComponentResult<Vec<ComponentDefinition>> {
    let mut components_definitions: Vec<ComponentDefinition> = Vec::new();
    for field in fields.named.iter() {
        components_definitions.push(parse_structure_field(&field)?);
    }
    return Ok(components_definitions);
}


fn parse_structure_field(field : &Field) -> ParseComponentResult<ComponentDefinition> {
    let field_name;
    if let Some(name) = &field.ident {
        field_name = name;
    }else {
        unreachable!();
    }
 
    let field_type = extract_component_type(&field.ty)?;
    let mut context_tag_number = None;
    let mut optional = false;


    match parse_field_attrs(&field.attrs) {
        Ok((opt, tag_number)) => {
            optional = opt;
            context_tag_number = tag_number;
        },
        Err(parse_error) => {
            match parse_error.kind() {
                ParseComponentErrorKind::NotFoundAttributeTag => {
                },
                _ => {
                    return Err(parse_error);
                }
            }
        }
    }

    return Ok(ComponentDefinition{
        id: field_name.clone(),
        kind: field_type,
        optional,
        context_tag_number
    });
}

fn extract_component_type(field_type: &Type) -> ParseComponentResult<Ident> {
    if let Type::Path(path) = &field_type {
        if path.path.segments[0].ident == SEQUENCE_COMPONENT_TYPE_NAME {
            if let PathArguments::AngleBracketed(brack_argument) = &path.path.segments[0].arguments {
                if let GenericArgument::Type(ty) = &brack_argument.args[0] {
                    if let Type::Path(path) = ty {
                        return Ok(path.path.segments[0].ident.clone());
                    }
                }
            }
        } else {
            return Err(ParseComponentErrorKind::InvalidFieldType)?;
        }
    }
    unreachable!();
}

fn parse_field_attrs(attrs: &Vec<Attribute>) -> ParseComponentResult<(bool, Option<u8>)> {
    for attr in attrs {
        if attr.path.segments.len() > 0 && attr.path.segments[0].ident == ASN1_SEQ_COMP_ATTR {
            return parse_component_attr(attr);
        }
    }
    return Err(ParseComponentErrorKind::NotFoundAttributeTag)?;
}

fn parse_component_attr(attr: &Attribute) -> ParseComponentResult<(bool, Option<u8>)> {
    let mut optional = false;
    let mut tag_number = None;

    if let Ok(Meta::List(ref meta)) = attr.parse_meta() {
        let subattrs : Vec<syn::NestedMeta> = meta.nested.iter().cloned().collect();
        for subattr in subattrs {
            if let syn::NestedMeta::Meta(ref a) = subattr {
                match a {
                    Meta::NameValue(name_value) => {
                        if name_value.ident == TAG_NUMBER_ATTR {
                            match name_value.lit {
                                syn::Lit::Int(ref value) => {
                                    let int_value = value.value();
                                    if int_value >= 256 {
                                        return Err(ParseComponentErrorKind::InvalidTagNumberValue)?;
                                    }
                                    tag_number = Some(int_value as u8);
                                },
                                _ => {
                                    return Err(ParseComponentErrorKind::InvalidTagNumberValue)?;
                                }
                            }
                        }else {
                            return Err(ParseComponentErrorKind::UnknownAttribute)?;
                        }
                    },
                    Meta::Word(ident) => {
                        if ident == OPTIONAL_ATTR {
                            optional = true;
                        }else {
                            return Err(ParseComponentErrorKind::UnknownAttribute)?;
                        }
                    },
                    _ => {
                        return Err(ParseComponentErrorKind::UnknownAttribute)?;
                    }
                };
            }
        }
    }

    return Ok((optional,tag_number));
}

fn parse_sequence_attrs(attrs: &Vec<Attribute>) -> ParseComponentResult<Option<u8>> {
    for attr in attrs {
        if attr.path.segments.len() > 0 && attr.path.segments[0].ident == ASN1_SEQ_ATTR {
            return parse_seq_attr(attr);
        }
    }
    return Err(ParseComponentErrorKind::NotFoundAttributeTag)?;
}

fn parse_seq_attr(attr: &Attribute) -> ParseComponentResult<Option<u8>> {
    let mut tag_number = None;

    if let Ok(Meta::List(ref meta)) = attr.parse_meta() {
        let subattrs : Vec<syn::NestedMeta> = meta.nested.iter().cloned().collect();
        for subattr in subattrs {
            if let syn::NestedMeta::Meta(ref a) = subattr {
                match a {
                    Meta::NameValue(name_value) => {
                        if name_value.ident == APPLICATION_TAG_ATTR {
                            match name_value.lit {
                                syn::Lit::Int(ref value) => {
                                    let int_value = value.value();
                                    if int_value >= 256 {
                                        return Err(ParseComponentErrorKind::InvalidTagNumberValue)?;
                                    }
                                    tag_number = Some(int_value as u8);
                                },
                                _ => {
                                    return Err(ParseComponentErrorKind::InvalidTagNumberValue)?;
                                }
                            }
                        }else {
                            return Err(ParseComponentErrorKind::UnknownAttribute)?;
                        }
                    },
                    _ => {
                        return Err(ParseComponentErrorKind::UnknownAttribute)?;
                    }
                };
            }
        }
    }

    return Ok(tag_number);
}
