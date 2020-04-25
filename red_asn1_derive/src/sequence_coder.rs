use super::parse_definitions::*;
use crate::field_coder::code_field;
use proc_macro2::TokenStream;
use syn::Ident;

/// Function to write the code of the methods to encode/decode a Sequence
/// used by Asn1Object.
pub fn code_sequence(sequence: &SequenceDefinition) -> TokenStream {
    let seq_name = &sequence.name;

    let seq_inner_calls = code_sequence_inner_calls(sequence);
    let encode_calls = &seq_inner_calls.encode_calls;
    let decode_calls = &seq_inner_calls.decode_calls;
    let components_unit_functions = &seq_inner_calls.components_unit_functions;

    let encode_value = code_encode_value(encode_calls);
    let decode_value = code_decode_value(decode_calls, seq_name);
    let inner_encode = code_inner_encode();
    let mut inner_decode = code_inner_decode(seq_name);

    let encode;
    let decode;

    if let Some(app_tag_number) = sequence.application_tag_number {
        encode = code_encode_with_application_tag(app_tag_number);
        inner_decode = quote! {
            #inner_decode

            fn _decode_application_tag(&self, raw_tag: &[u8]) -> red_asn1::Result<usize> {
                let (consumed_octets, decoded_tag) = Tag::decode(raw_tag)?;

                if decoded_tag != Tag::new(#app_tag_number, TagType::Constructed, TagClass::Application) {
                    return Err(red_asn1::Error::UnmatchedTag(TagClass::Application))?;
                }

                return Ok(consumed_octets);
            }
        };

        decode = code_decode_with_application_tag(seq_name);
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
        impl Asn1Object for #seq_name {
            fn tag() -> Tag {
                return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
            }

            #encode
            #decode

            #encode_value
            #decode_value
        }

        impl #seq_name {
            #components_unit_functions
            #inner_encode
            #inner_decode
        }
    };

    return total_exp;
}

fn code_encode_value(encode_calls: &TokenStream) -> TokenStream {
    return quote! {
        fn encode_value(&self) -> Vec<u8> {
            let mut value: Vec<u8> = Vec::new();
            #encode_calls
            return value;
        }
    };
}

/// To write the `decode_value` function of Asn1Object for Sequence.
/// In `decode_value` all the decode functions of the members of
/// the Sequence are called.
fn code_decode_value(
    decode_calls: &TokenStream,
    seq_name: &Ident,
) -> TokenStream {
    return quote! {
        fn decode_value(&mut self, raw: &[u8]) -> red_asn1::Result<()> {
            let mut consumed_octets = 0;
            #decode_calls

            if consumed_octets < raw.len() {
                return Err(red_asn1::Error::SequenceError(
                                stringify!(#seq_name).to_string(),
                                Box::new(red_asn1::Error::from(red_asn1::Error::NoAllDataConsumed))
                        ))?;
            }

            return Ok(());
        }
    };
}

fn code_inner_encode() -> TokenStream {
    return quote! {
        fn _inner_encode(&self) -> Vec<u8> {
            let mut encoded = Self::tag().encode();
            let mut encoded_value = self.encode_value();
            let mut encoded_length = red_asn1::encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }
    };
}

/// Function to write the `_inner_decode` function (called from `decode`) of
/// the structure, which decodes the structure tag and length, and calls
/// decode_value. In case of an application tag in the structure, this
/// is decoded in the `decode` function
fn code_inner_decode(seq_name: &Ident) -> TokenStream {
    return quote! {
        fn _inner_decode(&mut self, raw: &[u8]) -> red_asn1::Result<usize> {
            let (mut consumed_octets, decoded_tag) = Tag::decode(raw).or_else( |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(error.clone())
                ))
            )?;

            if decoded_tag != Self::tag() {
                return Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(red_asn1::Error::UnmatchedTag(TagClass::Universal))
                ))
            }

            let (_, raw_length) = raw.split_at(consumed_octets);

            let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length).or_else( |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(error.clone())
                ))
            )?;

            consumed_octets += consumed_octets_by_length;

            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(red_asn1::Error::from(red_asn1::Error::NoDataForLength))
                ))?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            self.decode_value(raw_value)?;
            consumed_octets += value_length;

            return Ok(consumed_octets);
        }
    };
}

/// Function to write the code of the Asn1Object `encode` function for Sequence
/// in case of having an application tag defined by the seq tag
fn code_encode_with_application_tag(app_tag_number: u8) -> TokenStream {
    return quote! {
        fn encode(&self) -> Vec<u8> {
            let mut encoded = Tag::new(
                #app_tag_number,
                TagType::Constructed,
                TagClass::Application
            ).encode();
            
            let mut encoded_value = self._inner_encode();
            let mut encoded_length = red_asn1::encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return encoded;
        }
    };
}

/// Function to write the code of the Asn1Object decode function for Sequence
/// in case of having an application tag defined by the seq tag
fn code_decode_with_application_tag(seq_name: &Ident) -> TokenStream {
    return quote! {
        fn decode(raw: &[u8]) -> red_asn1::Result<(usize, Self)> {
            let mut sequence = Self::default();
            let mut consumed_octets = sequence._decode_application_tag(raw).or_else(
                |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(error.clone())
                ))
            )?;
            let (_, raw_length) = raw.split_at(consumed_octets);
            let (value_length, consumed_octets_by_length) = red_asn1::decode_length(raw_length).or_else(
                |error|
                Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(error.clone())
                ))
            )?;
            consumed_octets += consumed_octets_by_length;
            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(red_asn1::Error::SequenceError(
                    stringify!(#seq_name).to_string(),
                    Box::new(red_asn1::Error::from(red_asn1::Error::NoDataForLength))
                ))?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            sequence._inner_decode(raw_value)?;
            consumed_octets += value_length;

            return Ok((consumed_octets, sequence));
        }
    };
}

pub fn code_sequence_inner_calls(
    sequence: &SequenceDefinition,
) -> SequenceInnerCallsCode {
    let mut components_unit_functions = quote! {};
    let mut encode_calls = quote! {};
    let mut decode_calls = quote! {};
    let seq_name = &sequence.name;

    for field in &sequence.fields {
        let encoder_name = field.encoder_name();
        let decoder_name = field.decoder_name();
        let field_name = &field.id;

        encode_calls = quote! {
            #encode_calls
            value.append(&mut self.#encoder_name());
        };

        decode_calls = quote! {
            #decode_calls
            consumed_octets += self.#decoder_name(&raw[consumed_octets..]).or_else(
                |error| Err(red_asn1::Error::SequenceFieldError(
                    stringify!(#seq_name).to_string(),
                    stringify!(#field_name).to_string(),
                    Box::new(error.clone())
                )))?;
        };

        let field_code = code_field(field);
        let encoder = &field_code.encoder;
        let decoder = &field_code.decoder;

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
