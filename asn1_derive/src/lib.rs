#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

mod parse_error;

use parse_error::*;

struct ComponentDefinition {
    id: Ident,
    kind: Ident,
    optional: bool,
    context_tag_number: Option<u8>
}

static SEQUENCE_COMPONENT_TYPE_NAME: &str = "SequenceComponent2";
static ASN1_SEQ_COMP_ATTR: &str = "seq_comp";
static OPTIONAL_ATTR: &str = "optional";
static TAG_NUMBER_ATTR: &str = "tag_number";

#[proc_macro_derive(Asn1Sequence, attributes(seq_comp))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let mut expanded_getters = quote! {};
    let mut encode_calls = quote! {};
    let mut decode_calls = quote! {};
    let mut new_fields = quote! {};
    let components : Vec<ComponentDefinition>;

    if let Data::Struct(data_struct) = &ast.data {
        components = extract_components_definitions(data_struct).unwrap();

        for component in components {
            let field_name = component.id;
            let inner_type = component.kind;

            let concatenated = format!("get_{}", field_name);
            let getter_name = Ident::new(&concatenated, field_name.span());

            let concatenated = format!("set_{}", field_name);
            let setter_name = Ident::new(&concatenated, field_name.span());

            let concatenated = format!("unset_{}", field_name);
            let unsetter_name = Ident::new(&concatenated, field_name.span());

            let concatenated = format!("decode_{}", field_name);
            let decoder_name = Ident::new(&concatenated, field_name.span());

            let concatenated = format!("encode_{}", field_name);
            let encoder_name = Ident::new(&concatenated, field_name.span());

            expanded_getters = quote! {
                #expanded_getters

                fn #getter_name (&self) -> Option<&#inner_type> {
                    return self.#field_name.get_inner_value();
                }
                
                fn #setter_name (&mut self, value: #inner_type) {
                    return self.#field_name.set_inner_value(value);
                }

                fn #unsetter_name (&mut self) {
                    return self.#field_name.unset_inner_value();
                }

                

                /*
                fn #decoder_name (&mut self, raw: &[u8]) -> Result<()> {
                    return self.#field_name.decode(raw);
                }

                */
            };

            if let Some(context_tag_number) = component.context_tag_number {
                expanded_getters = quote! {
                    #expanded_getters
                    
                    fn #encoder_name (&self) -> Asn1Result<Vec<u8>> {
                        let tag = Tag::new(#context_tag_number, TagType::Constructed, TagClass::Context);
                        let mut encoded = tag.encode();
                        let mut encoded_value = self.#field_name.encode()?;
                        let mut encoded_length = self.encode_length(encoded_value.len());

                        encoded.append(&mut encoded_length);
                        encoded.append(&mut encoded_value);

                        return Ok(encoded);
                    }
                }
            }else {
                expanded_getters = quote! {
                    #expanded_getters
                    
                    fn #encoder_name (&self) -> Asn1Result<Vec<u8>> {
                        return self.#field_name.encode();
                    }
                }
            }

            encode_calls = quote! {
                #encode_calls
                match self.#encoder_name() {
                    Ok(ref mut bytes) => {
                        value.append(bytes);
                    },
                    Err(error) => {
                        return Err(error);
                    }
                };
            };

            

            decode_calls = quote! {
                #decode_calls
                match self.#decoder_name(&raw[consumed_octets..]) {
                    Ok(num_octets) => {
                        consumed_octets += num_octets;
                    },
                    Err(error) => {
                        return Err(error);
                    }
                };
            };

            new_fields = quote! {
                #new_fields
                #field_name: SequenceComponent2::new()
            }

        }
    }
    
    let encode_value = quote! {
        fn encode_value(&self) -> Asn1Result<Vec<u8>> {
            let mut value: Vec<u8> = Vec::new();
            #encode_calls
            return Ok(value);
        }
    };

    
    let decode_value = quote! {
        fn decode_value(&mut self, raw: &[u8]) -> Result<()> {
            let mut consumed_octets = 0;
            #decode_calls
        }
    };


    let total_exp = quote! {
        impl #name {
            fn new() -> #name {
                return #name {
                    #new_fields
                };
            }

            fn encode(&self) -> Asn1Result<Vec<u8>> {
                let mut encoded = self.encode_tag();
                let mut encoded_value = self.encode_value()?;
                let mut encoded_length = self.encode_length(encoded_value.len());

                encoded.append(&mut encoded_length);
                encoded.append(&mut encoded_value);

                return Ok(encoded);
            }

            fn encode_tag(&self) -> Vec<u8> {
                return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER).encode();
            }

            
            fn encode_length(&self, value_size: usize) -> Vec<u8> {
                if value_size < 128 {
                    return vec![value_size as u8];
                }

                let mut shifted_length = value_size;
                let mut octets_count: u8 = 0;
                let mut encoded_length : Vec<u8> = Vec::new();

                while shifted_length > 0 {
                    octets_count += 1;
                    encoded_length.push(shifted_length as u8);
                    shifted_length >>= 8;
                }

                encoded_length.push(octets_count | 0b10000000);
                
                encoded_length.reverse();

                return encoded_length;
            }

            #expanded_getters
            #encode_value
            // #decode_value
        }
    };


    return TokenStream::from(total_exp);
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