use crate::traits::*;
use crate::error as asn1err;

/// Class to represent a field of a Sequence
#[derive(Debug, PartialEq, Default)]
pub struct SeqField<T: Asn1Object + Default> {
    value: Option<T>
}

impl<T: Asn1Object + Default> SeqField<T> {

    pub fn get_value(&self) -> Option<&T> {
        match self.value {
            Some(ref subtype) => {
                return Some(&subtype);
            },
            None => {
                return None;
            }
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn unset_inner_value(&mut self) {
        self.value = None;
    }

    pub fn encode(&self) -> asn1err::Result<Vec<u8>> {
        let encoded_value = self.encode_value()?;
        return Ok(encoded_value);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        match &self.value {
            Some(value) => {
                return value.encode();
            }
            None => {
                return Err(asn1err::Error::NoValue)?;
            }
        };
    }

    pub fn decode(&mut self, raw: &[u8]) -> asn1err::Result<usize> {
        let mut new_subtype = T::default();
        let size = new_subtype.decode(raw)?;
        self.value = Some(new_subtype);
        return Ok(size);
    }

}

impl<T: Asn1Object + Default> From<T> for SeqField<T> {
    fn from(value: T) -> Self {
        Self { value: Some(value)}
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::*;

    #[test]
    fn create_default_seq_field() {
        assert_eq!(
            SeqField{value: None},
            SeqField::<Integer>::default()
        );
    }

    #[test]
    fn create_seq_field() {
        assert_eq!(
            SeqField{value: Some(Integer::from(1))},
            SeqField::from(Integer::from(1))
        );

        assert_eq!(
            SeqField{value: Some(OctetString::from(vec![1,2,3,4]))},
            SeqField::from(OctetString::from(vec![1,2,3,4]))
        );
    }


    #[test]
    fn get_value() {
        assert_eq!(
            Some(&Integer::from(1)),
            SeqField::from(Integer::from(1)).get_value()
        );

        assert_eq!(
            Some(&OctetString::from(vec![1,2,3,4])),
            SeqField::from(OctetString::from(vec![1,2,3,4])).get_value()
        );

        assert_eq!(
            None,
            SeqField::<Integer>::default().get_value()
        );
    }

    #[test]
    fn set_value() {
        let mut field = SeqField::default();

        field.set_value(Integer::from(1));
        assert_eq!(
            Some(&Integer::from(1)),
            field.get_value()
        );

        let mut field = SeqField::default();
        field.set_value(OctetString::from(vec![1,2,3,4]));
        assert_eq!(
            Some(&OctetString::from(vec![1,2,3,4])),
            field.get_value()
        );
    }

}
