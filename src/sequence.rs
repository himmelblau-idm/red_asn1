use super::tag::{Tag, TagClass, TagType};
use super::traits::{Asn1Object, Asn1Tagged};
use super::error::Asn1Error;
use std::result::Result;

pub static SEQUENCE_TAG_NUMBER: u8 = 0x10;

pub struct Sequence<'a, 'b> {
    tag: Tag,
    application_tag: Option<Tag>,
    components: Vec<SequenceComponent<'a, 'b>>
}

impl<'a, 'b> Asn1Tagged for Sequence<'a, 'b> {
    fn type_tag() -> Tag {
        return Tag::new_constructed_universal(SEQUENCE_TAG_NUMBER);
    }
}

impl<'a, 'b> Asn1Object for Sequence<'a, 'b> {

    fn tag(&self) -> &Tag {
        return &self.tag;
    }

    fn encode_value(&self) -> Result<Vec<u8>,Asn1Error> {
        let mut value: Vec<u8> = Vec::new();
        for component in self.components.iter() {
            value.append(&mut component.encode()?);
        }

        return Ok(value);
    }

    fn encode(&self) -> Result<Vec<u8>,Asn1Error> {
        match &self.application_tag {
            Some(_) => {
                return self._application_encode();
            },
            None =>  {
                return self._normal_encode();
            }
        };
    }

    fn decode_value(&mut self, raw: &[u8]) -> Result<(), Asn1Error> {
        let mut consumed_octets = 0;
        for component in self.components.iter_mut() {
            let component_consumed_octets = component.decode(&raw[consumed_octets..])?;
            consumed_octets += component_consumed_octets;
        }
        return Ok(());
    }

    fn decode(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        match &self.application_tag {
            Some(_) => {
                return self._application_decode(raw);
            },
            None => {
                return self._normal_decode(raw);
            }
        }
    }
}

impl<'a, 'b> Sequence<'a, 'b> {
    pub fn new() -> Sequence<'a, 'b> {
        return Sequence{
            tag: Sequence::type_tag(),
            application_tag: None,
            components: Vec::new()
        };
    }

    pub fn new_application(tag_number: u8) -> Sequence<'a, 'b> {
        return Sequence{
            tag: Sequence::type_tag(),
            application_tag: Some(Tag::new(tag_number, TagType::Constructed, TagClass::Application)),
            components: Vec::new()
        };
    }

    pub fn def(&mut self, identifier: &str, context_tag_number: Option<u8>, subtype_tag: Tag) -> Result<(), Asn1Error> {
        self.components.push(SequenceComponent::new(identifier.to_string(), context_tag_number, 
                                subtype_tag, SeqCompOptionality::Required)?);
        return Ok(());
    }

    pub fn def_optional(&mut self, identifier: &str, context_tag_number: Option<u8>, subtype_tag: Tag) -> Result<(), Asn1Error> {
        self.components.push(SequenceComponent::new(identifier.to_string(), context_tag_number, 
                                subtype_tag, SeqCompOptionality::Optional)?);
        return Ok(());
    }

    pub fn def_default(&mut self, identifier: &str, context_tag_number: Option<u8>, subtype_tag: Tag, default: Box<Asn1Object>) -> Result<(), Asn1Error> {
        self.components.push(SequenceComponent::new(identifier.to_string(), context_tag_number, 
                                subtype_tag, SeqCompOptionality::Default(default))?);
        return Ok(());
    }

    pub fn set_value(&mut self, identifier: &str, value: Box<&'a mut (Asn1Object + 'b)>) -> Result<(), Asn1Error> {
        for component in self.components.iter_mut() {
            if identifier == component.identifier() {
                return component.set_value(value);
            }
        }

        return Err(Asn1Error::new("No subcomponent with such identifier".to_string()));
    }
    
    fn _application_encode(&self) -> Result<Vec<u8>, Asn1Error> {
        let mut encoded = self.application_tag.unwrap().encode();
        let mut encoded_value = self._normal_encode()?;
        let mut encoded_length = self.encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return Ok(encoded);
    }

    fn _normal_encode(&self) -> Result<Vec<u8>, Asn1Error> {
        let mut encoded = self.encode_tag();
        let mut encoded_value = self.encode_value()?;
        let mut encoded_length = self.encode_length(encoded_value.len());

        encoded.append(&mut encoded_length);
        encoded.append(&mut encoded_value);

        return Ok(encoded);
    }

    fn _application_decode(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        let mut consumed_octets = self._decode_application_tag(raw)?;
        let (_, raw_length) = raw.split_at(consumed_octets);
        let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;
        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(Asn1Error::new("Invalid value: Not enough data for length".to_string()));
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        self._normal_decode(raw_value)?;
        consumed_octets += value_length;

        return Ok(consumed_octets);
    }

    fn _decode_application_tag(&self, raw_tag: &[u8]) -> Result<usize, Asn1Error> {
        let mut decoded_tag = Tag::new_empty();
        let consumed_octets = decoded_tag.decode(raw_tag)?;

        if &decoded_tag != &self.application_tag.unwrap() {
            return Err(Asn1Error::new("Invalid tag: Not valid tag for type".to_string()));
        }
        return Ok(consumed_octets);
    }


    fn _normal_decode(&mut self, raw: &[u8]) -> Result<usize,Asn1Error> {
        let mut consumed_octets = self.decode_tag(raw)?;

        let (_, raw_length) = raw.split_at(consumed_octets);

        let (value_length, consumed_octets_by_length) = self.decode_length(raw_length)?;
        consumed_octets += consumed_octets_by_length;

        let (_, raw_value) = raw.split_at(consumed_octets);

        if value_length > raw_value.len() {
            return Err(Asn1Error::new("Invalid value: Not enough data for length".to_string()));
        }

        let (raw_value, _) = raw_value.split_at(value_length);

        self.decode_value(raw_value)?;
        consumed_octets += value_length;

        return Ok(consumed_octets);
    }
}

enum SeqCompOptionality {
    Required,
    Optional,
    Default(Box<Asn1Object>)
}


struct SequenceComponent<'a, 'b: 'a> {
    identifier: String,
    context_tag: Option<Tag>,
    subtype_tag: Tag,
    optional: bool,
    subtype_default: Option<Box<Asn1Object>>,
    subtype_instance: Option<Box<&'a mut (Asn1Object + 'b)>>
}

impl<'a, 'b> SequenceComponent<'a, 'b> {

    fn new(identifier: String, context_tag_number: Option<u8>, subtype_tag: Tag, optionality: SeqCompOptionality) -> Result<SequenceComponent<'a, 'b>, Asn1Error> {
        let mut sequence_component = SequenceComponent{
            identifier,
            context_tag: None,
            subtype_tag,
            optional: false,
            subtype_default: None,
            subtype_instance: None
        };
        sequence_component._set_optionality(optionality)?;
        sequence_component._calculate_tag(context_tag_number);

        return Ok(sequence_component);
    }

    fn _set_optionality(&mut self, optionality: SeqCompOptionality) -> Result<(), Asn1Error> {
        match optionality {
            SeqCompOptionality::Default(option) => {
                if option.tag() != &self.subtype_tag {
                    return Err(Asn1Error::new("Invalid type for default".to_string()));
                }

                self.optional = true;
                self.subtype_default = Some(option);
            }
            SeqCompOptionality::Optional => {
                self.optional = true;
                self.subtype_default = None;
            }
            SeqCompOptionality::Required => {
                self.optional = false;
                self.subtype_default = None;
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

    fn identifier(&self) -> &String {
        return &self.identifier;
    }

    fn set_value(&mut self, value: Box<&'a mut (Asn1Object + 'b)>) -> Result<(),Asn1Error> {
        if value.tag() != &self.subtype_tag {
            return Err(Asn1Error::new("Invalid type".to_string()));
        }

        self.subtype_instance = Some(value);
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
        match &mut self.subtype_instance {
            Some(value) => {
                return value.decode(raw);
            }
            None => {
                match &mut self.subtype_default {
                    Some(value) => {
                        return value.decode(raw);
                    }
                    None => {
                        return Err(Asn1Error::new("No value provided for encoding".to_string()));
                    }
                }
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
        match &self.subtype_instance {
            Some(value) => {
                return value.encode();
            }
            None => {
                match &self.subtype_default {
                    Some(value) => {
                        return value.encode();
                    }
                    None => {
                        return Err(Asn1Error::new("No value provided for encoding".to_string()));
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::integer::{Integer, INTEGER_TAG_NUMBER};
    use super::super::octetstring::{OctetString, OCTET_STRING_TAG_NUMBER};
    use super::super::traits::Asn1InstanciableObject;

    #[test]
    fn test_encode_sequence() {
        let mut sequence = Sequence::new();
        
        sequence.def("id", Some(0), Integer::type_tag()).unwrap();
        sequence.def("data", Some(1), OctetString::type_tag()).unwrap();

        let mut inte = Integer::new(9);
        let mut octet_str = OctetString::new(vec![0x1,0x2,0x3,0x4]);

        sequence.set_value("id", Box::new(&mut inte)).unwrap();
        sequence.set_value("data", Box::new(&mut octet_str)).unwrap();

        assert_eq!(vec![0x30, 0xd, 
        0xa0,  0x3, INTEGER_TAG_NUMBER, 0x1, 0x9, 
        0xa1, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4], 
        sequence.encode().unwrap());
    }

    #[test]
    fn test_encode_sequence_without_context_tags() {
        let mut sequence = Sequence::new();
        sequence.def("id", None, Integer::type_tag()).unwrap();
        sequence.def("data", None, OctetString::type_tag()).unwrap();

        let mut inte = Integer::new(9);
        let mut octet_str = OctetString::new(vec![0x1,0x2,0x3,0x4]);

        sequence.set_value("id", Box::new(&mut inte)).unwrap();
        sequence.set_value("data", Box::new(&mut octet_str)).unwrap();

        assert_eq!(vec![0x30, 0x9, INTEGER_TAG_NUMBER, 0x1, 0x9, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4], sequence.encode().unwrap());
    }

    #[test]
    fn test_encode_with_optional() {
        let mut sequence = Sequence::new();
        sequence.def_optional("id", Some(0), Integer::type_tag()).unwrap();

        assert_eq!(vec![0x30, 0x0], sequence.encode().unwrap());
    }

    #[test]
    fn test_encode_with_default() {
        let mut sequence = Sequence::new();
        sequence.def_default("id", Some(0), Integer::type_tag(), Box::new(Integer::new(5))).unwrap();

        assert_eq!(vec![0x30, 0x5, 0xa0, 0x3, INTEGER_TAG_NUMBER , 0x1, 0x5], 
        sequence.encode().unwrap());
    }

    #[should_panic(expected = "Invalid type for default")]
    #[test]
    fn test_create_with_default_of_incorrect_type() {
        let mut sequence = Sequence::new();
        sequence.def_default("id", Some(0), OctetString::type_tag(), Box::new(Integer::new(5))).unwrap();
    }

    #[should_panic(expected = "Invalid type")]
    #[test]
    fn test_set_component_value_of_incorrect_type() {
        let mut sequence = Sequence::new();
        sequence.def("id", Some(0), OctetString::type_tag()).unwrap();

        sequence.set_value("id", Box::new(&mut Integer::new(9))).unwrap();
    }

    #[should_panic(expected = "No value provided")]
    #[test]
    fn test_encode_without_give_required_values() {
        let mut sequence = Sequence::new();
        sequence.def("id", Some(0), Integer::type_tag()).unwrap();
        sequence.encode().unwrap();
    }

    #[test]
    fn test_change_tag_number_in_sequence(){
        let sequence = Sequence::new_application(7);
        assert_eq!(vec![0x67, 0x2, 0x30, 0x0], sequence.encode().unwrap());
    }


    #[test]
    fn test_decode_empty() {
        let mut sequence = Sequence::new();
        let consumed_octets = sequence.decode(&[0x30, 0x0]).unwrap();
        assert_eq!(2, consumed_octets);
    }

    #[test]
    fn test_decode_empty_with_application_tag() {
        let mut sequence = Sequence::new_application(7);
        let consumed_octets = sequence.decode(&[0x67, 0x2, 0x30, 0x0]).unwrap();
        assert_eq!(4, consumed_octets);
    }

    #[test]
    fn test_decode_empty_with_excesive_bytes() {
        let mut sequence = Sequence::new();
        let consumed_octets = sequence.decode(&[0x30, 0x0, 0xff,0xff]).unwrap();
        assert_eq!(2, consumed_octets);
    }

    #[test]
    fn test_decode_empty_with_application_tag_with_excesive_bytes() {
        let mut sequence = Sequence::new_application(7);
        let consumed_octets = sequence.decode(&[0x67, 0x2, 0x30, 0x0, 0xff, 0xff]).unwrap();
        assert_eq!(4, consumed_octets);
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_tag() {
        let mut sequence = Sequence::new();
        sequence.decode(&[0xff, 0x0]).unwrap();
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_application_tag() {
        let mut sequence = Sequence::new();
        sequence.decode(&[0xff, 0x2, 0x30, 0x0]).unwrap();
    }

    #[should_panic (expected = "Invalid tag: Not valid tag for type")]
    #[test]
    fn test_decode_with_invalid_application_inner_tag() {
        let mut sequence = Sequence::new();
        sequence.decode(&[0x67, 0x2, 0xff, 0x0]).unwrap();
    }

    #[test]
    fn test_decode() {
        let mut sequence = Sequence::new();
        pasar de tercer argumento un singleton que proporcione la tag del tipo y también instancias del mismo...
        sequence.def("id", Some(0), Integer::type_tag()).unwrap();
        sequence.def("data", Some(1), OctetString::type_tag()).unwrap();

        let mut inte = Integer::new_default();
        let mut octet_str = OctetString::new_default();

        sequence.set_value("id", Box::new(&mut inte)).unwrap();
        sequence.set_value("data", Box::new(&mut octet_str)).unwrap();

        sequence.decode(&[0x30, 0xd, 
                        0xa0,  0x3, INTEGER_TAG_NUMBER, 0x1, 0x9, 
                        0xa1, 0x6, OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();
        

        let inte = sequence.get_value("id").unwrap();
        let octet_str = sequence.get_value("data").unwrap();

        inte = inte.downcast::

        assert_eq!(&9, inte.value());
        assert_eq!(&[0x1, 0x2, 0x3, 0x4], octet_str.value());
    }

    #[test]
    fn test_decode_without_context_tags() {
        let mut sequence = Sequence::new();
        sequence.def("id", None, Integer::type_tag()).unwrap();
        sequence.def("data", None, OctetString::type_tag()).unwrap();

        let mut inte = Integer::new(9);
        let mut octet_str = OctetString::new(vec![0x1,0x2,0x3,0x4]);

        sequence.set_value("id", Box::new(&mut inte)).unwrap();
        sequence.set_value("data", Box::new(&mut octet_str)).unwrap();

        sequence.decode(&[0x30, 0x9, 
                          INTEGER_TAG_NUMBER, 0x1, 0x9, 
                          OCTET_STRING_TAG_NUMBER, 0x4, 0x1, 0x2, 0x3, 0x4]).unwrap();

        assert_eq!(&9, inte.value());
        assert_eq!(&[0x1, 0x2, 0x3, 0x4], octet_str.value());
    }

}
