use crate::traits::*;
use crate::error as asn1err;

#[derive(Debug, PartialEq, Default)]
pub struct SeqField<T: Asn1Object + Default> {
    value: Option<T>
}

impl<T: Asn1Object + Default> SeqField<T> {

    pub fn new(value: T) -> Self {
        return Self {
            value: Some(value)
        }
    }

    pub fn get_inner_value(&self) -> Option<&T> {
        match self.value {
            Some(ref subtype) => {
                return Some(&subtype);
            },
            None => {
                return None;
            }
        }
    }

    pub fn set_inner_value(&mut self, value: T) {
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
                return Err(asn1err::ErrorKind::NoValue)?;
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
            SeqField{value: Some(Integer::new(1))},
            SeqField::new(Integer::new(1))
        );

        assert_eq!(
            SeqField{value: Some(OctetString::new(vec![1,2,3,4]))},
            SeqField::new(OctetString::new(vec![1,2,3,4]))
        );
    }


}