#![recursion_limit="256"]
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

mod parse_error;
mod parser;
mod parse_definitions;

use parser::*;

#[proc_macro_derive(Asn1Sequence, attributes(seq, seq_comp))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {    
    let ast = parse_macro_input!(input as DeriveInput);

    let mut expanded_getters = quote! {};
    let mut encode_calls = quote! {};
    let mut decode_calls = quote! {};
    let mut new_fields = quote! {};

    let sequence_definition = extract_sequence_definition(&ast).unwrap();

    let name = &sequence_definition.name;

    for component in sequence_definition.components {
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

                fn #decoder_name (&mut self, raw: &[u8]) -> Asn1Result<usize> {
                    let mut decoded_tag = Tag::new_empty();
                    let mut consumed_octets = 0;

                    match decoded_tag.decode(raw) {
                        Ok(octets_count) => {
                            consumed_octets += octets_count;
                        },
                        Err(error) => {
                            match error.kind() {
                                Asn1ErrorKind::InvalidTagNumber => {
                                    return Err(Asn1ErrorKind::InvalidContextTagNumber)?;
                                },
                                Asn1ErrorKind::InvalidTagEmpty => {
                                    return Err(Asn1ErrorKind::InvalidContextTagEmpty)?;
                                },
                                _ => {
                                    return Err(error);
                                }
                            }
                        }
                    }

                    if decoded_tag != Tag::new(#context_tag_number, TagType::Constructed, TagClass::Context) {
                        return Err(Asn1ErrorKind::InvalidContextTag)?;
                    }

                    let (_, raw_length) = raw.split_at(consumed_octets);

                    let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
                    consumed_octets += consumed_octets_by_length;
                    let (_, raw_value) = raw.split_at(consumed_octets);

                    if value_length > raw_value.len() {
                        return Err(Asn1ErrorKind::NoDataForLength)?;
                    }

                    let (raw_value, _) = raw_value.split_at(value_length);

                    self.#field_name.decode(raw_value)?;
                    consumed_octets += value_length;

                    return Ok(consumed_octets);
                }
            }
        }else {
            expanded_getters = quote! {
                #expanded_getters
                
                fn #encoder_name (&self) -> Asn1Result<Vec<u8>> {
                    return self.#field_name.encode();
                }

                fn #decoder_name (&mut self, raw: &[u8]) -> Asn1Result<usize> {
                    return self.#field_name.decode(raw);
                }
            }
        }

        if component.optional {
            encode_calls = quote! {
                #encode_calls
                match self.#encoder_name() {
                    Ok(ref mut bytes) => {
                        value.append(bytes);
                    },
                    Err(error) => {
                        match error.kind() {
                            Asn1ErrorKind::NoValue => {
                            }
                            _ => {
                                return Err(error);
                            }
                        }
                    }
                };
            };

            if let Some(_) = component.context_tag_number {
                decode_calls = quote! {
                    #decode_calls
                    match self.#decoder_name(&raw[consumed_octets..]) {
                        Ok(num_octets) => {
                            consumed_octets += num_octets;
                        },
                        Err(error) => {
                            match error.kind() {
                                Asn1ErrorKind::InvalidContextTagEmpty => {},
                                Asn1ErrorKind::InvalidContextTagNumber => {},
                                Asn1ErrorKind::InvalidContextTag => {},
                                _ => {
                                    return Err(error);
                                }
                            }
                        }
                    };
                };
            }else{
                decode_calls = quote! {
                    #decode_calls
                    match self.#decoder_name(&raw[consumed_octets..]) {
                        Ok(num_octets) => {
                            consumed_octets += num_octets;
                        },
                        Err(error) => {
                            match error.kind() {
                                Asn1ErrorKind::InvalidTagEmpty => {},
                                Asn1ErrorKind::InvalidTypeTag => {},
                                Asn1ErrorKind::InvalidTagNumber => {},
                                _ => {
                                    return Err(error);
                                }
                            }
                        }
                    };
                };
            }

            
        } else {
            encode_calls = quote! {
                #encode_calls
                value.append(&mut self.#encoder_name()?);
            };

            decode_calls = quote! {
                #decode_calls
                consumed_octets += self.#decoder_name(&raw[consumed_octets..])?;
            };
        }

        new_fields = quote! {
            #new_fields
            #field_name: SequenceComponent2::new(),
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
        fn decode_value(&mut self, raw: &[u8]) -> Asn1Result<()> {
            let mut consumed_octets = 0;
            #decode_calls

            if consumed_octets < raw.len() {
                return Err(Asn1ErrorKind::NoAllDataConsumed)?;
            }

            return Ok(());
        }
    };

    let mut encode = quote! {
        fn _inner_encode(&self) -> Asn1Result<Vec<u8>> {
            let mut encoded = self.encode_tag();
            let mut encoded_value = self.encode_value()?;
            let mut encoded_length = self.encode_length(encoded_value.len());

            encoded.append(&mut encoded_length);
            encoded.append(&mut encoded_value);

            return Ok(encoded);
        }
    };

    let mut decode = quote! {
        fn _inner_decode(&mut self, raw: &[u8]) -> Asn1Result<usize> {
            let mut consumed_octets = self.decode_tag(raw)?;

            let (_, raw_length) = raw.split_at(consumed_octets);

            let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
            consumed_octets += consumed_octets_by_length;

            let (_, raw_value) = raw.split_at(consumed_octets);

            if value_length > raw_value.len() {
                return Err(Asn1ErrorKind::NoDataForLength)?;
            }

            let (raw_value, _) = raw_value.split_at(value_length);

            self.decode_value(raw_value)?;
            consumed_octets += value_length;

            return Ok(consumed_octets);
        }
    };

    if let Some(application_tag_number) = sequence_definition.application_tag_number {

        encode = quote! {
            fn encode(&self) -> Asn1Result<Vec<u8>> {
                let mut encoded = Tag::new(#application_tag_number, 
                                            TagType::Constructed, TagClass::Application).encode();
                let mut encoded_value = self._inner_encode()?;
                let mut encoded_length = self.encode_length(encoded_value.len());

                encoded.append(&mut encoded_length);
                encoded.append(&mut encoded_value);

                return Ok(encoded);
            }

            #encode
        };

        decode = quote! {

            fn _decode_application_tag(&self, raw_tag: &[u8]) -> Asn1Result<usize> {
                let mut decoded_tag = Tag::new_empty();
                let consumed_octets = decoded_tag.decode(raw_tag)?;

                if decoded_tag != Tag::new(#application_tag_number, TagType::Constructed, TagClass::Application) {
                    return Err(Asn1ErrorKind::InvalidTypeTag)?;
                }

                return Ok(consumed_octets);
            }

            fn decode(&mut self, raw: &[u8]) -> Asn1Result<usize> {
                let mut consumed_octets = self._decode_application_tag(raw)?;
                let (_, raw_length) = raw.split_at(consumed_octets);
                let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
                consumed_octets += consumed_octets_by_length;
                let (_, raw_value) = raw.split_at(consumed_octets);

                if value_length > raw_value.len() {
                    return Err(Asn1ErrorKind::NoDataForLength)?;
                }

                let (raw_value, _) = raw_value.split_at(value_length);

                self._inner_decode(raw_value)?;
                consumed_octets += value_length;

                return Ok(consumed_octets);
            }

            #decode
        };

    } else {
        encode = quote! {
            fn encode(&self) -> Asn1Result<Vec<u8>> {
                return self._inner_encode();
            }
            #encode
        };

        decode = quote! {
            fn decode(&mut self, raw: &[u8]) -> Asn1Result<usize> {
                return self._inner_decode(raw);
            }

            #decode
        }
    }


    let total_exp = quote! {
        impl #name {
            fn new() -> #name {
                return #name {
                    #new_fields
                };
            }

            fn tag(&self) -> Tag {
                return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
            } 

            fn encode_tag(&self) -> Vec<u8> {
                return self.tag().encode();
            }

            fn decode_tag(&self, raw_tag: &[u8]) -> Asn1Result<usize> {
                let mut decoded_tag = Tag::new_empty();
                let consumed_octets = decoded_tag.decode(raw_tag)?;

                if decoded_tag != self.tag() {
                    return Err(Asn1ErrorKind::InvalidTypeTag)?;
                }
                return Ok(consumed_octets);
            }

            #encode
            
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

            fn decode_length(&self, raw_length: &[u8]) -> Asn1Result<(usize, usize)> {
                let raw_length_length = raw_length.len();
                if raw_length_length == 0 {
                    return Err(Asn1ErrorKind::InvalidLengthEmpty)?;
                }

                let mut consumed_octets: usize = 1;
                let is_short_form = (raw_length[0] & 0x80) == 0;
                if is_short_form {
                    return Ok(((raw_length[0] & 0x7F) as usize, consumed_octets));
                }

                let length_of_length = (raw_length[0] & 0x7F) as usize;
                if length_of_length >= raw_length_length {
                    return Err(Asn1ErrorKind::InvalidLengthOfLength)?;
                }

                let mut length: usize = 0;
                for i in 1..(length_of_length + 1) {
                    length <<= 8;
                    length += raw_length[i] as usize;
                }
                consumed_octets += length_of_length;

                return Ok((length, consumed_octets));
            }

            #decode

            #expanded_getters
            #encode_value
            #decode_value
        }
    };


    return TokenStream::from(total_exp);
}

