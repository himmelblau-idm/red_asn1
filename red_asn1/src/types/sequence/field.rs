use crate::traits::*;
use crate::error as asn1err;

/// Class to represent a field of a Sequence
#[derive(Debug, PartialEq, Default)]
pub struct SeqField<T: Asn1Object> {
    pub value: T
}

impl<T: Asn1Object> SeqField<T> {

    pub fn encode(&self) -> Vec<u8> {
        return self.encode_value();
    }

    fn encode_value(&self) -> Vec<u8> {
        return self.value.encode();
    }

    pub fn decode(raw: &[u8]) -> asn1err::Result<(usize, Self)> {
        let mut s = Self::default();
        let (size, new_subtype) = T::decode(raw)?;
        s.value = new_subtype;
        return Ok((size, s));
    }

}

impl<T: Asn1Object + Default> From<T> for SeqField<T> {
    fn from(value: T) -> Self {
        Self { value: value}
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::*;

    #[test]
    fn create_seq_field() {
        assert_eq!(
            SeqField{value: Integer::from(1)},
            SeqField::from(Integer::from(1))
        );

        assert_eq!(
            SeqField{value: OctetString::from(vec![1,2,3,4])},
            SeqField::from(OctetString::from(vec![1,2,3,4]))
        );
    }

}
