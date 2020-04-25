use crate::parse_definitions::{FieldCode, FieldDefinition};
use proc_macro2::TokenStream;
use syn::{Ident, PathSegment};

/// Method to create the code for the encode/decode methods
/// for a field of the structure
pub fn code_field(field: &FieldDefinition) -> FieldCode {
    return FieldCode {
        encoder: code_field_encoder(field),
        decoder: code_field_decoder(field),
    };
}

/// Method to create the code for the decode method of a
/// structure field
fn code_field_decoder(field: &FieldDefinition) -> TokenStream {
    match field.context_tag_number {
        Some(ctx_tag) => match field.optional {
            true => {
                code_optional_field_decoder_with_context_tag(field, ctx_tag)
            }
            false => {
                code_required_field_decoder_with_context_tag(field, ctx_tag)
            }
        },

        None => code_field_decoder_without_context_tag(field),
    }
}

fn code_required_field_decoder_with_context_tag(
    field: &FieldDefinition,
    context_tag_number: u8,
) -> TokenStream {
    let decoder_name = field.decoder_name();
    let field_name = &field.id;
    let field_type = compose_field_type(&field.kind, &field.sub_kinds);

    return quote! {
        fn #decoder_name (&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
            let mut consumed_octets = 0;
            let decoded_tag;

            match Tag::decode(raw) {
                Ok((octets_count, tag)) => {
                    consumed_octets += octets_count;
                    decoded_tag = tag;
                },
                Err(error) => {
                    match error.clone() {
                        red_asn1::Error::NotEnoughTagOctets(_) => {
                            return Err(red_asn1::Error::NotEnoughTagOctets(TagClass::Context))?;
                        }
                        red_asn1::Error::EmptyTag(_) => {
                            return Err(red_asn1::Error::EmptyTag(TagClass::Context))?;
                        }
                        _ => {
                            return Err(error);
                        }
                    }
                }
            }

            if decoded_tag != Tag::new(#context_tag_number, TagType::Constructed, TagClass::Context) {
                return Err(red_asn1::Error::UnmatchedTag(TagClass::Context))?;
            }

            let (_, raw_length) = raw.split_at(consumed_octets);

            let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length)?;
            consumed_octets += consumed_octets_by_length;
            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(red_asn1::Error::NoDataForLength)?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            let (_, field) = #field_type::decode(raw_value)?;
            consumed_octets += value_length;
            self.#field_name = field;

            return Ok(consumed_octets);
        }
    };
}

/// Write the code for decode a field in case of having a context tag
/// and being optional. In this case the decode fails in case the
/// context tag matchs but the type tag is incorrect, or the type
/// data is invalid. However is context tag doesn't match, then,
/// the field is set to None.
fn code_optional_field_decoder_with_context_tag(
    field: &FieldDefinition,
    context_tag_number: u8,
) -> TokenStream {
    let decoder_name = field.decoder_name();
    let field_name = &field.id;
    let field_type = compose_field_type(&field.kind, &field.sub_kinds);

    return quote! {
        fn #decoder_name (&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
            let mut consumed_octets = 0;
            let decoded_tag;

            match Tag::decode(raw) {
                Ok((octets_count, tag)) => {
                    consumed_octets += octets_count;
                    decoded_tag = tag;
                },
                Err(error) => {
                    self.#field_name = None;
                    return Ok(0);
                }
            }

            if decoded_tag != Tag::new(#context_tag_number, TagType::Constructed, TagClass::Context) {
                self.#field_name = None;
                return Ok(0);
            }

            let (_, raw_length) = raw.split_at(consumed_octets);

            let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length)?;
            consumed_octets += consumed_octets_by_length;
            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(red_asn1::Error::NoDataForLength)?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            let (_, type_tag) = Tag::decode(raw_value)?;
            if type_tag != #field_type::tag() {
                return Err(red_asn1::Error::UnmatchedTag(TagClass::Universal));
            }


            let (_, field) = #field_type::decode(raw_value)?;
            consumed_octets += value_length;

            self.#field_name = field;

            return Ok(consumed_octets);
        }
    };
}

fn code_field_decoder_without_context_tag(
    field: &FieldDefinition,
) -> TokenStream {
    let decoder_name = field.decoder_name();
    let field_name = &field.id;
    let field_type = compose_field_type(&field.kind, &field.sub_kinds);
    return quote! {
        fn #decoder_name (&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
            let (size, field) = #field_type::decode(raw)?;
            self.#field_name = field;
            return Ok(size);
        }
    };
}

/// Method to create the code of the encode method of a
/// structure field
fn code_field_encoder(field: &FieldDefinition) -> TokenStream {
    match field.context_tag_number {
        Some(context_tag_number) => match field.optional {
            true => code_optional_field_encoder_with_context_tag(
                field,
                context_tag_number,
            ),
            false => code_required_field_encoder_with_context_tag(
                field,
                context_tag_number,
            ),
        },
        None => code_field_encoder_without_context_tag(field),
    }
}

fn code_optional_field_encoder_with_context_tag(
    field: &FieldDefinition,
    ctx_tag: u8,
) -> TokenStream {
    let encoder_name = field.encoder_name();
    let field_name = &field.id;

    return quote! {
        fn #encoder_name (&self) -> Vec<u8> {
            if self.#field_name == None {
                return Vec::new();
            }

            let tag = Tag::new(
                #ctx_tag,
                TagType::Constructed,
                TagClass::Context
            );
            let mut encoded = tag.encode();
            let mut encoded_value = self.#field_name.encode();
            let mut encoded_length = red_asn1::encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }
    };
}

fn code_required_field_encoder_with_context_tag(
    field: &FieldDefinition,
    ctx_tag: u8,
) -> TokenStream {
    let encoder_name = field.encoder_name();
    let field_name = &field.id;

    return quote! {
        fn #encoder_name (&self) -> Vec<u8> {
            let tag = Tag::new(
                #ctx_tag,
                TagType::Constructed,
                TagClass::Context
            );
            let mut encoded = tag.encode();
            let mut encoded_value = self.#field_name.encode();
            let mut encoded_length = red_asn1::encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }
    };
}

fn code_field_encoder_without_context_tag(
    field: &FieldDefinition,
) -> TokenStream {
    let encoder_name = field.encoder_name();
    let field_name = &field.id;

    return quote! {
        fn #encoder_name (&self) -> Vec<u8> {
            return self.#field_name.encode();
        }
    };
}

/// Function to compose the path to call Self functions. Simple types
/// call this functions with Type::function(), but other types, like
/// Option, required to call Self functions in the way
/// Option::<SubType>::function().
fn compose_field_type(
    field_kind: &Ident,
    field_sub_kinds: &Option<PathSegment>,
) -> TokenStream {
    match field_sub_kinds {
        Some(field_sub_types) => {
            quote! {#field_kind::<#field_sub_types>}
        }
        None => {
            quote! {#field_kind}
        }
    }
}
