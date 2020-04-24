use crate::parse_definitions::{FieldCode, FieldDefinition};
use proc_macro2::TokenStream;

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
    let decoder_name = field.decoder_name();
    let field_name = &field.id;
    let field_type = &field.kind;

    if let Some(context_tag_number) = field.context_tag_number {
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

                let (_, field) = SeqField::<#field_type>::decode(raw_value)?;
                consumed_octets += value_length;
                self.#field_name = field;

                return Ok(consumed_octets);
            }
        };
    } else {
        return quote! {
            fn #decoder_name (&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
                let (size, field) = SeqField::<#field_type>::decode(raw)?;
                self.#field_name = field;
                return Ok(size);
            }
        };
    }
}

/// Method to create the code of the encode method of a
/// structure field
fn code_field_encoder(field: &FieldDefinition) -> TokenStream {
    let encoder_name = field.encoder_name();
    let field_name = &field.id;

    if let Some(context_tag_number) = field.context_tag_number {
        return quote! {
            fn #encoder_name (&self) -> Vec<u8> {
                let tag = Tag::new(#context_tag_number, TagType::Constructed, TagClass::Context);
                let mut encoded = tag.encode();
                let mut encoded_value = self.#field_name.encode();
                let mut encoded_length = red_asn1::encode_length(encoded_value.len());

                encoded.append(&mut encoded_length);
                encoded.append(&mut encoded_value);

                return encoded;
            }
        };
    } else {
        return quote! {
            fn #encoder_name (&self) -> Vec<u8> {
                return self.#field_name.encode();
            }
        };
    }
}
