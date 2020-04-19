use crate::error::{Error, Result};

pub fn encode_length(value_size: usize) -> Vec<u8> {
    if value_size < 128 {
        return vec![value_size as u8];
    }

    let mut shifted_length = value_size;
    let mut octets_count: u8 = 0;
    let mut encoded_length: Vec<u8> = Vec::new();

    while shifted_length > 0 {
        octets_count += 1;
        encoded_length.push(shifted_length as u8);
        shifted_length >>= 8;
    }

    encoded_length.push(octets_count | 0b10000000);

    encoded_length.reverse();

    return encoded_length;
}

/// To decode the object value length from DER, should not be overwritten
pub fn decode_length(raw_length: &[u8]) -> Result<(usize, usize)> {
    let raw_length_length = raw_length.len();
    if raw_length_length == 0 {
        return Err(Error::LengthEmpty)?;
    }

    let mut consumed_octets: usize = 1;
    let is_short_form = (raw_length[0] & 0x80) == 0;
    if is_short_form {
        return Ok(((raw_length[0] & 0x7F) as usize, consumed_octets));
    }

    let length_of_length = (raw_length[0] & 0x7F) as usize;
    if length_of_length >= raw_length_length {
        return Err(Error::NotEnoughLengthOctects)?;
    }

    let mut length: usize = 0;
    for i in 1..(length_of_length + 1) {
        length <<= 8;
        length += raw_length[i] as usize;
    }
    consumed_octets += length_of_length;

    return Ok((length, consumed_octets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_length() {
        assert_eq!(vec![0x0], encode_length(0));
        assert_eq!(vec![0x1], encode_length(1));
        assert_eq!(vec![0x7F], encode_length(127));
        assert_eq!(vec![0x81, 0x80], encode_length(128));
        assert_eq!(vec![0x81, 0xFF], encode_length(255));
        assert_eq!(vec![0x82, 0x01, 0x00], encode_length(256));
        assert_eq!(vec![0x82, 0xFF, 0xFF], encode_length(65535));
        assert_eq!(vec![0x83, 0x01, 0x00, 0x00], encode_length(65536));

        assert_eq!(vec![0x84, 0x10, 0xf3, 0x91, 0xbd], encode_length(0x10f391bd));
        assert_eq!(vec![0x84, 0x0f, 0xc4, 0x69, 0x89], encode_length(0xfc46989));
        assert_eq!(vec![0x84, 0x31, 0xb2, 0x50, 0x42], encode_length(0x31b25042));
        assert_eq!(vec![0x84, 0x13, 0x93, 0xaa, 0x93], encode_length(0x1393aa93));
        assert_eq!(vec![0x84, 0x05, 0x71, 0x6f, 0xa9], encode_length(0x5716fa9));
    }

    #[test]
    fn test_decode_length() {
        assert_eq!((0, 1), decode_length(&[0x0]).unwrap());
        assert_eq!((1, 1), decode_length(&[0x1]).unwrap());
        assert_eq!((127, 1), decode_length(&[0x7F]).unwrap());
        assert_eq!((128, 2), decode_length(&[0x81, 0x80]).unwrap());
        assert_eq!((255, 2), decode_length(&[0x81, 0xFF]).unwrap());
        assert_eq!((256, 3), decode_length(&[0x82, 0x01, 0x00]).unwrap());
        assert_eq!((65535, 3), decode_length(&[0x82, 0xFF, 0xFF]).unwrap());
        assert_eq!((65536, 4), decode_length(&[0x83, 0x01, 0x00, 0x00]).unwrap());

        assert_eq!((0x10f391bd, 5), decode_length(&[0x84, 0x10, 0xf3, 0x91, 0xbd]).unwrap());
        assert_eq!((0xfc46989, 5), decode_length(&[0x84, 0x0f, 0xc4, 0x69, 0x89]).unwrap());
        assert_eq!((0x31b25042, 5), decode_length(&[0x84, 0x31, 0xb2, 0x50, 0x42]).unwrap());
        assert_eq!((0x1393aa93, 5), decode_length(&[0x84, 0x13, 0x93, 0xaa, 0x93]).unwrap());
        assert_eq!((0x5716fa9, 5), decode_length(&[0x84, 0x05, 0x71, 0x6f, 0xa9]).unwrap());
    }
}
