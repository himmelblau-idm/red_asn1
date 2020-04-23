use super::parse_definitions::*;
use proc_macro2::TokenStream;
use crate::field_coder::FieldCoder;

pub fn code_sequence_inner_calls(
    sequence: &SequenceDefinition,
) -> SequenceInnerCallsCode {
    let mut components_unit_functions = quote! {};
    let mut encode_calls = quote! {};
    let mut decode_calls = quote! {};
    let sequence_name = &sequence.name;

    for field in &sequence.fields {
        let component_code = FieldCoder::new(field).code_field();
        let encoder_name = field.encoder_name();
        let decoder_name = field.decoder_name();
        let field_name = &field.id;

        if field.optional {
            encode_calls = quote! {
                #encode_calls
                value.append(&mut self.#encoder_name());
            };

            let invalid_tag_errors_handlers;

            if let Some(_) = field.context_tag_number {
                invalid_tag_errors_handlers = quote! {
                    if tag_class != red_asn1::TagClass::Context {
                        return Err(red_asn1::Error::SequenceFieldError(
                            stringify!(#sequence_name).to_string(),
                            stringify!(#field_name).to_string(),
                            Box::new(error.clone())
                        ))?;
                    }
                };
            } else {
                invalid_tag_errors_handlers = quote! {
                    if tag_class == red_asn1::TagClass::Context {
                        return Err(red_asn1::Error::SequenceFieldError(
                            stringify!(#sequence_name).to_string(),
                            stringify!(#field_name).to_string(),
                            Box::new(error.clone())
                        ))?;
                    }
                };
            }

            decode_calls = quote! {
                #decode_calls
                match self.#decoder_name(&raw[consumed_octets..]) {
                    Ok(num_octets) => {
                        consumed_octets += num_octets;
                    },
                    Err(error) => {
                        match error.clone() {
                            Error::EmptyTag(tag_class) => {
                                #invalid_tag_errors_handlers
                            }
                            Error::NotEnoughTagOctets(tag_class) => {
                                #invalid_tag_errors_handlers
                            }
                            Error::UnmatchedTag(tag_class) => {
                                #invalid_tag_errors_handlers
                            }
                            _ => {
                                return Err(red_asn1::Error::SequenceFieldError(
                                    stringify!(#sequence_name).to_string(),
                                    stringify!(#field_name).to_string(),
                                    Box::new(error.clone())
                                    ))?;
                            }
                        }
                    }
                };
            };
        } else {
            encode_calls = quote! {
                #encode_calls
                value.append(&mut self.#encoder_name());
            };

            decode_calls = quote! {
                #decode_calls
                consumed_octets += self.#decoder_name(&raw[consumed_octets..]).or_else(
                    |error| Err(red_asn1::Error::SequenceFieldError(
                                stringify!(#sequence_name).to_string(),
                                stringify!(#field_name).to_string(),
                                Box::new(error.clone())
                                )))?;
            };
        }

        let encoder = &component_code.encoder;
        let decoder = &component_code.decoder;

        components_unit_functions = quote! {
            #components_unit_functions

            #encoder
            #decoder
        };
    }

    return SequenceInnerCallsCode {
        encode_calls,
        decode_calls,
        components_unit_functions,
    };
}

pub fn code_sequence(
    sequence_definition: &SequenceDefinition,
    sequence_inner_calls: &SequenceInnerCallsCode,
) -> TokenStream {
    let name = &sequence_definition.name;
    let encode_calls = &sequence_inner_calls.encode_calls;
    let decode_calls = &sequence_inner_calls.decode_calls;
    let components_unit_functions =
        &sequence_inner_calls.components_unit_functions;

    let encode_value = quote! {
        fn encode_value(&self) -> Vec<u8> {
            let mut value: Vec<u8> = Vec::new();
            #encode_calls
            return value;
        }
    };

    let decode_value = quote! {
        fn decode_value(&mut self, raw: &[u8]) -> red_asn1::Result<()> {
            let mut consumed_octets = 0;
            #decode_calls

            if consumed_octets < raw.len() {
                return Err(red_asn1::Error::SequenceError(
                                stringify!(#name).to_string(),
                                Box::new(red_asn1::Error::from(red_asn1::Error::NoAllDataConsumed))
                        ))?;
            }

            return Ok(());
        }
    };

    let inner_encode = quote! {
        fn _inner_encode(&self) -> Vec<u8> {
            let mut encoded = Self::tag().encode();
            let mut encoded_value = self.encode_value();
            let mut encoded_length = red_asn1::encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }
    };

    let mut inner_decode = quote! {
        fn _inner_decode(&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
            let (mut consumed_octets, decoded_tag) = Tag::decode(raw).or_else( |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#name).to_string(),
                    Box::new(error.clone())
                ))
            )?;

            if decoded_tag != Self::tag() {
                return Err(red_asn1::Error::SequenceError(
                    stringify!(#name).to_string(),
                    Box::new(red_asn1::Error::UnmatchedTag(TagClass::Universal))
                ))
            }

            let (_, raw_length) = raw.split_at(consumed_octets);

            let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length).or_else( |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#name).to_string(),
                    Box::new(error.clone())
                ))
            )?;

            consumed_octets += consumed_octets_by_length;

            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(red_asn1::Error::SequenceError(
                    stringify!(#name).to_string(),
                    Box::new(red_asn1::Error::from(red_asn1::Error::NoDataForLength))
                ))?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            self.decode_value(raw_value)?;
            consumed_octets += value_length;

            return Ok(consumed_octets);
        }
    };

    let encode;
    let decode;

    if let Some(application_tag_number) =
        sequence_definition.application_tag_number
    {
        encode = quote! {
            fn encode(&self) -> Vec<u8> {
                let mut encoded = Tag::new(#application_tag_number,
                                            TagType::Constructed, TagClass::Application).encode();
                let mut encoded_value = self._inner_encode();
                let mut encoded_length = red_asn1::encode_length(encoded_value.len());

                encoded.append(&mut encoded_length);
                encoded.append(&mut encoded_value);

                return encoded;
            }
        };

        inner_decode = quote! {
            #inner_decode

            fn _decode_application_tag(&self, raw_tag: &[u8]) -> red_asn1::Result<usize> {
                let (consumed_octets, decoded_tag) = Tag::decode(raw_tag)?;

                if decoded_tag != Tag::new(#application_tag_number, TagType::Constructed, TagClass::Application) {
                    return Err(red_asn1::Error::UnmatchedTag(TagClass::Application))?;
                }

                return Ok(consumed_octets);
            }
        };

        decode = quote! {

            fn decode(raw: &[u8]) -> red_asn1::Result<(usize, Self)> {
                let mut sequence = Self::default();
                let mut consumed_octets = sequence._decode_application_tag(raw).or_else( |error|
                    Err(red_asn1::Error::SequenceError(
                        stringify!(#name).to_string(),
                        Box::new(error.clone())
                    ))
                )?;
                let (_, raw_length) = raw.split_at(consumed_octets);
                let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length).or_else( |error|
                    Err(red_asn1::Error::SequenceError(
                        stringify!(#name).to_string(),
                        Box::new(error.clone())
                    ))
                )?;
                consumed_octets += consumed_octets_by_length;
                let (_, raw_value) = raw.split_at(consumed_octets);

                if value_length > raw_value.len() {
                    return Err(red_asn1::Error::SequenceError(
                        stringify!(#name).to_string(),
                        Box::new(red_asn1::Error::from(red_asn1::Error::NoDataForLength))
                    ))?;
                }

                let (raw_value, _) = raw_value.split_at(value_length);

                sequence._inner_decode(raw_value)?;
                consumed_octets += value_length;

                return Ok((consumed_octets, sequence));
            }
        };
    } else {
        encode = quote! {
            fn encode(&self) -> Vec<u8> {
                return self._inner_encode();
            }
        };

        decode = quote! {
            fn decode(raw: &[u8]) -> red_asn1::Result<(usize, Self)> {
                let mut sequence = Self::default();
                let size = sequence._inner_decode(raw)?;
                return Ok((size, sequence));
            }
        }
    }

    let total_exp = quote! {
        impl Asn1Object for #name {
            fn tag() -> Tag {
                return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
            }

            #encode
            #decode

            #encode_value
            #decode_value
        }

        impl #name {
            #components_unit_functions
            #inner_encode
            #inner_decode
        }
    };

    return total_exp;
}
