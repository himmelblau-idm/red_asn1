use super::super::tag::{Tag, TagClass, TagType};
use super::super::traits::{Asn1Object, Asn1Factory};
use super::super::error::Asn1Error;
use std::result::Result;

pub enum SeqCompOptionality {
    Required,
    Optional
}


pub struct SequenceComponent<'a, 'b: 'a> {
    identifier: String,
    context_tag: Option<Tag>,
    subtype_tag: Tag,
    subtype: Box<&'a (Asn1Factory + 'b)>,
    optional: bool,
    subtype_instance: Option<Box<Asn1Object>>,
    subtype_value: Option<Box<&'a (Asn1Object + 'b)>>
}

impl<'a, 'b> SequenceComponent<'a, 'b> {

    pub fn new(identifier: String, context_tag_number: Option<u8>, subtype: Box<&'a (Asn1Factory + 'b)>, optionality: SeqCompOptionality) 
    -> Result<SequenceComponent<'a, 'b>, Asn1Error> {
        let mut sequence_component = SequenceComponent{
            identifier,
            context_tag: None,
            subtype_tag: subtype.type_tag(),
            subtype,
            optional: false,
            subtype_instance: None,
            subtype_value: None
        };
        sequence_component._set_optionality(optionality)?;
        sequence_component._calculate_tag(context_tag_number);

        return Ok(sequence_component);
    }

    fn _set_optionality(&mut self, optionality: SeqCompOptionality) -> Result<(), Asn1Error> {
        match optionality {
            SeqCompOptionality::Optional => {
                self.optional = true;
            }
            SeqCompOptionality::Required => {
                self.optional = false;
            }
        }
        return Ok(());
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

    pub fn set_value(&mut self, value: Box<&'a (Asn1Object + 'b)>) -> Result<(),Asn1Error> {
        if value.tag() != &self.subtype_tag {
            return Err(Asn1Error::new("Invalid type".to_string()));
        }

        self.subtype_value = Some(value);
        return Ok(());
    }


    fn _decode_context(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        let mut consumed_octets = self._decode_context_tag(raw)?;
        let (_, raw_length) = raw.split_at(consumed_octets);
        let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;
        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(Asn1Error::new("Invalid value: Not enough data for length".to_string()));
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        self._decode_inner(raw_value)?;
        consumed_octets += value_length;

        return Ok(consumed_octets);
    }

    fn _decode_context_tag(&self, raw_tag: &[u8]) -> Result<usize, Asn1Error> {
        let mut decoded_tag = Tag::new_empty();
        let consumed_octets = decoded_tag.decode(raw_tag)?;

        if &decoded_tag != &self.context_tag.unwrap() {
            return Err(Asn1Error::new("Invalid tag: Not valid tag for type".to_string()));
        }
        return Ok(consumed_octets);
    }


    fn _decode_inner(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        match self.subtype_instance {
            Some(ref value) => {
                let consumed_octets = value.decode(raw)?;
                // obter dalgunha forma a referencia ao interior da Box
                self.subtype_value = Some(*(&value));
                return Ok(consumed_octets);
            }
            None => {
                let mut subtype_instance = self.subtype.new_asn1();
                let consumed_octets = subtype_instance.decode(raw)?;
                self.subtype_instance = Some(subtype_instance);
                self.subtype_value = Some()
                return Ok(consumed_octets);
            }
        }
    }

}

impl<'a, 'b> Asn1Object for SequenceComponent<'a, 'b> {
    fn tag(&self) -> &Tag {
        match self.context_tag {
            Some(ref tag) => tag,
            None => &self.subtype_tag
        }
    }

    fn encode(&self) -> Result<Vec<u8>,Asn1Error> {
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


    fn encode_value(&self) -> Result<Vec<u8>, Asn1Error> {
        match &self.subtype_value {
            Some(value) => {
                return value.encode();
            }
            None => {
                return Err(Asn1Error::new("No value provided for encoding".to_string()));
            }
        }
    }

    fn decode_value(&mut self, _raw: &[u8]) -> Result<(), Asn1Error> {
        return Ok(());
    }

    fn decode(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        
        match self.context_tag {
            Some(_) => {
                return self._decode_context(raw);
            },
            None => {
                return self._decode_inner(raw);
            }
        };
    }
}
