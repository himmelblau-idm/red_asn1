use super::super::tag::{Tag, TagClass, TagType};
use super::super::traits::{Asn1Object, Asn1Tagged};
use super::super::error::*;

pub enum SeqCompOptionality {
    Required,
    Optional
}

pub struct SequenceComponent<'a, 'b: 'a> {
    identifier: String,
    context_tag: Option<Tag>,
    optional: bool,
    subtype_tag: Tag,
    subtype_ref: Option<Box<&'a mut (Asn1Object + 'b)>>,
    last_value_was_decoded: bool
}

impl<'a, 'b> SequenceComponent<'a, 'b> {

    pub fn new<T: Asn1Tagged>(identifier: String, context_tag_number: Option<u8>, optionality: SeqCompOptionality) 
    -> SequenceComponent<'a, 'b> {
        let mut sequence_component = SequenceComponent{
            identifier,
            context_tag: None,
            subtype_tag: T::type_tag(),
            optional: false,
            subtype_ref: None,
            last_value_was_decoded: false
        };
        sequence_component._set_optionality(optionality);
        sequence_component._calculate_tag(context_tag_number);

        return sequence_component;
    }

    fn _set_optionality(&mut self, optionality: SeqCompOptionality) {
        match optionality {
            SeqCompOptionality::Optional => {
                self.optional = true;
            }
            SeqCompOptionality::Required => {
                self.optional = false;
            }
        }
    }

    fn _calculate_tag(&mut self, context_tag_number: Option<u8>) {
        self.context_tag = match context_tag_number {
            Some(tag_number) => {
                let new_tag = Tag::new(tag_number, TagType::Constructed, TagClass::Context);
                Some(new_tag)
            }
            None => {
                None
            }
        };
    }

    pub fn identifier(&self) -> &String {
        return &self.identifier;
    }

    pub fn is_optional(&self) -> bool {
        return self.optional;
    }

    pub fn has_context_tag(&self) -> bool {
        return self.context_tag != None;
    }

    pub fn set_ref(&mut self, value: Box<&'a mut (Asn1Object + 'b)>) -> Asn1Result<()> {
        if value.tag() != &self.subtype_tag {
            return Err(Asn1ErrorKind::InvalidTypeTag)?;
        }

        self.subtype_ref = Some(value);
        self.last_value_was_decoded = false;
        return Ok(());
    }

    fn unset_subtype_ref_value(&mut self) {
        match &mut self.subtype_ref {
            Some(subtype_ref) => {
                subtype_ref.unset_value();
            },
            None => {}
        };
    }

    fn _decode_context(&mut self, raw: &[u8]) -> Asn1Result<usize> {
        let mut consumed_octets = self._decode_context_tag(raw)?;
        let (_, raw_length) = raw.split_at(consumed_octets);
        let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;
        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(Asn1ErrorKind::NoDataForLength)?;
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        self._decode_inner(raw_value)?;
        consumed_octets += value_length;

        return Ok(consumed_octets);
    }

    fn _decode_context_tag(&self, raw_tag: &[u8]) -> Asn1Result<usize> {
        let mut decoded_tag = Tag::new_empty();
        let consumed_octets = decoded_tag.decode(raw_tag)?;

        if &decoded_tag != &self.context_tag.unwrap() {
            return Err(Asn1ErrorKind::InvalidContextTag)?;
        }
        return Ok(consumed_octets);
    }


    fn _decode_inner(&mut self, raw: &[u8]) -> Asn1Result<usize> {
        match &mut self.subtype_ref {
            Some(value) => {
                let consumed_octets = value.decode(raw)?;
                return Ok(consumed_octets);
            },
            None => {
                return Err(Asn1ErrorKind::NoValue)?;
            }
        };
    }

}

impl<'a, 'b> Asn1Object for SequenceComponent<'a, 'b> {
    fn tag(&self) -> &Tag {
        match self.context_tag {
            Some(ref tag) => tag,
            None => &self.subtype_tag
        }
    }

    fn encode(&self) -> Asn1Result<Vec<u8>> {
        let mut encoded_value;

        match self.encode_value() {
            Ok(value) => {
                encoded_value = value;
            },
            Err(error) => {
                if self.optional {
                    return Ok(vec![]);
                }
                else{
                    return Err(error);
                }
            }
        }

        match self.context_tag {
            Some(tag) => {
                let mut encoded = tag.encode();
                let mut encoded_length = self.encode_length(encoded_value.len());

                encoded.append(&mut encoded_length);
                encoded.append(&mut encoded_value);

                return Ok(encoded);
            }
            None => {
                return Ok(encoded_value)
            }
        }
    }

    fn encode_value(&self) -> Asn1Result<Vec<u8>> {
        match &self.subtype_ref {
            Some(value) => {
                return value.encode();
            }
            None => {
                return Err(Asn1ErrorKind::NoValue)?;
            }
        }
    }

    fn decode_value(&mut self, _raw: &[u8]) -> Asn1Result<()> {
        return Ok(());
    }

    fn decode(&mut self, raw: &[u8]) -> Asn1Result<usize> {
        let result = match self.context_tag {
            Some(_) => {
                self._decode_context(raw)
            },
            None => {
                self._decode_inner(raw)
            }
        };

        match result {
            Ok(value) => {
                return Ok(value);
            },
            Err(err) => {
                self.unset_subtype_ref_value();
                return Err(err);
            }
        };
    }

    fn unset_value(&mut self) {
        self.unset_subtype_ref_value();
    }
}


pub struct SequenceComponent2<T> {
    identifier: String,
    context_tag: Option<Tag>,
    optional: bool,
    subtype_tag: Tag,
    subtype: Option<T>,
    last_value_was_decoded: bool
}