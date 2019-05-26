extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

use parse_error::*;

struct ComponentDefinition {
    id: Ident,
    kind: Ident,
    optional: bool,
    context_tag_number: Option<u8>
}



#[proc_macro_derive(Asn1Sequence, attributes(seq_comp))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let mut expanded_getters = quote! {};
    let mut encode_calls = quote! {};
    let mut decode_calls = quote! {};
    let components : Vec<ComponentDefinition>;

    if let Data::Struct(data_struct) = &ast.data {
        components = extract_components_definitions(data_struct);

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

                fn #decoder_name (&mut self, raw: &[u8]) -> Result<()> {
                    return self.#field_name.decode(raw);
                }

                fn #encoder_name (&self) -> Result<Vec<u8>> {
                    return self.#field_name.encode();
                }
            };

            encode_calls = quote! {
                #encode_calls
                match self.#encoder_name() {
                    Ok(ref mut bytes) => {
                        value.append(bytes);
                    },
                    Err(error) => {
                        return error;
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
                        return error;
                    }
                };
            };

        }
    }

    let encode_value = quote! {
        fn encode_value(&self) -> Vec<u8> {
            let mut value: Vec<u8> = Vec::new();
            #encode_calls
            return value;
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
            #expanded_getters
            #encode_value
            #decode_value
        }
    };

    // println!("{}", total_exp);

    TokenStream::from(total_exp)
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
        components_definitions.push(parse_structure_field(&field));
    }
    return components_definitions;
}

seguir incluyendo ParseComponentResult
fn parse_structure_field(field : &Field) -> ComponentDefinition {
    let field_name;
    if let Some(name) = &field.ident {
        field_name = name;
    }else {
        panic!();
    }
 
    let field_type = extract_component_type(&field.ty);
    let mut context_tag_number = None;
    let mut optional = false;

    if let Some((opt, tag_number)) = parse_field_attrs(&field.attrs){
        optional = opt;
        context_tag_number = tag_number;
    }

    return ComponentDefinition{
        id: field_name.clone(),
        kind: field_type,
        optional,
        context_tag_number
    };
}

fn extract_component_type(field_type: &Type) -> Ident {
    if let Type::Path(path) = &field_type {
        println!("Field is {}", path.path.segments[0].ident);
        if path.path.segments[0].ident == "SequenceComponent" {
            if let PathArguments::AngleBracketed(brack_argument) = &path.path.segments[0].arguments {
                if let GenericArgument::Type(ty) = &brack_argument.args[0] {
                    if let Type::Path(path) = ty {
                        return path.path.segments[0].ident.clone();
                    }
                }
            }
        }
    }
    panic!();
}

fn parse_field_attrs(attrs: &Vec<Attribute>) -> Option<(bool, Option<u8>)> {
    for attr in attrs {
        if attr.path.segments.len() > 0 && attr.path.segments[0].ident == "seq_comp" {
            return Some(parse_component_attr(attr));
        }
    }
    return None;
}

fn parse_component_attr(attr: &Attribute) -> (bool, Option<u8>) {
    let mut optional = false;
    let mut tag_number = None;

    if let Ok(Meta::List(ref meta)) = attr.parse_meta() {
        let subattrs : Vec<syn::NestedMeta> = meta.nested.iter().cloned().collect();
        for subattr in subattrs {
            if let syn::NestedMeta::Meta(ref a) = subattr {
                match a {
                    Meta::NameValue(name_value) => {
                        if name_value.ident == "tag_number" {
                            match name_value.lit {
                                syn::Lit::Int(ref value) => {
                                    let int_value = value.value();
                                    if int_value >= 256 {
                                        panic!()
                                    }
                                    tag_number = Some(int_value as u8);
                                },
                                _ => {
                                    panic!();
                                }
                            }
                        }else {
                            panic!();
                        }
                    },
                    Meta::Word(ident) => {
                        if ident == "optional" {
                            optional = true;
                        }else {
                            panic!();
                        }
                    },
                    _ => {
                        panic!();
                    }
                };
            }
        }
    }

    return (optional,tag_number);
}