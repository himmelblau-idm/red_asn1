
use proc_macro2::TokenStream;
use super::parse_definitions::*;

pub fn code_component(comp_def: &ComponentDefinition) -> ComponentCode {
      return ComponentCode {
        getter: code_getter(comp_def),
        setter: code_setter(comp_def),
        unsetter: code_unsetter(comp_def),
        encoder: code_encoder(comp_def),
        decoder: code_decoder(comp_def)
    };
}

fn code_getter(comp_def: &ComponentDefinition) -> TokenStream {
    let getter_name = comp_def.getter_name();
    let inner_type = &comp_def.kind;
    let field_name = &comp_def.id;

    return quote! {
        fn #getter_name (&self) -> Option<&#inner_type> {
            return self.#field_name.get_inner_value();
        }
    };
}

fn code_setter(comp_def: &ComponentDefinition) -> TokenStream {
    let setter_name = comp_def.setter_name();
    let inner_type = &comp_def.kind;
    let field_name = &comp_def.id;

    return quote! {
        fn #setter_name (&mut self, value: #inner_type) {
            return self.#field_name.set_inner_value(value);
        }
    };
}

fn code_unsetter(comp_def: &ComponentDefinition) -> TokenStream {
    let unsetter_name = comp_def.unsetter_name();
    let field_name = &comp_def.id;

    return quote! {
        fn #unsetter_name (&mut self) {
            return self.#field_name.unset_inner_value();
        }
    };
}

fn code_decoder(comp_def: &ComponentDefinition) -> TokenStream {
    let decoder_name = comp_def.decoder_name();
    let field_name = &comp_def.id;

    if let Some(context_tag_number) = comp_def.context_tag_number {
        return quote! {
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
        return quote! {
            fn #decoder_name (&mut self, raw: &[u8]) -> Asn1Result<usize> {
                return self.#field_name.decode(raw);
            }
        }
    }
}

fn code_encoder(comp_def: &ComponentDefinition) -> TokenStream {
    let encoder_name = comp_def.encoder_name();
    let field_name = &comp_def.id;

    if let Some(context_tag_number) = comp_def.context_tag_number {
        return quote! {
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
        return quote! {
            fn #encoder_name (&self) -> Asn1Result<Vec<u8>> {
                return self.#field_name.encode();
            }
        }
    }

}
