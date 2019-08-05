use chrono::prelude::*;
use crate::tag::Tag;
use crate::traits::*;
use crate::error as asn1err;
use std::str;
use super::TimeFormat;

pub static GENERALIZED_TIME_TAG_NUMBER: u8 = 0x18;

/// Class to encode/decode GeneralizedTime ASN1
#[derive(Debug, PartialEq, Default)]
pub struct GeneralizedTime {
    _value: Option<DateTime<Utc>>,
    format: TimeFormat
}

impl GeneralizedTime {
    pub fn new(value: DateTime<Utc>) -> GeneralizedTime {
        return GeneralizedTime {
            _value: Some(value),
            format: TimeFormat::default()
        };
    }

    pub fn set_format(&mut self, format: TimeFormat) {
        self.format = format;
    }

    pub fn value(&self) -> Option<&DateTime<Utc>> {
        match &self._value {
            Some(ref value) => {
                return Some(value);
            }
            None => {
                return None;
            }
        };
    }

    fn _format_datetime_as_string(&self, datetime: &DateTime<Utc>) -> String {
        return self.format.format_to_string(datetime);
    }

}

impl Asn1Object for GeneralizedTime {

    fn tag(&self) -> Tag {
        return Tag::new_primitive_universal(GENERALIZED_TIME_TAG_NUMBER);
    }

    fn encode_value(&self) -> asn1err::Result<Vec<u8>> {
        match self._value {
            Some(value) => {
                return Ok(self._format_datetime_as_string(&value).into_bytes());
            },
            None => {
                return Err(asn1err::ErrorKind::NoValue)?;
            }
        }
    }

    fn decode_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() < 15 {
            return Err(asn1err::ValueErrorKind::NoDataForType)?;
        }

        let year_str = str::from_utf8(&raw[0..4])?;
        let month_str = str::from_utf8(&raw[4..6])?;
        let day_str = str::from_utf8(&raw[6..8])?;
        let hour_str = str::from_utf8(&raw[8..10])?;
        let minute_str = str::from_utf8(&raw[10..12])?;
        let second_str = str::from_utf8(&raw[12..14])?;

        let year: i32 = year_str.parse()?;
        let month: u32 = month_str.parse()?;
        let day: u32 = day_str.parse()?;
        let hour: u32 = hour_str.parse()?;
        let minute: u32 = minute_str.parse()?;
        let second: u32 = second_str.parse()?;
        let mut decisecond: u32 = 0;

        if raw.len() >= 17 {
            let decisecond_str = str::from_utf8(&raw[15..16])?;
            decisecond = decisecond_str.parse()?;
        }

        let is_utc: bool = raw[raw.len() - 1] == 'Z' as u8;

        if is_utc {
            self._value = Some(Utc.ymd(year, month, day).and_hms_nano(hour, minute, second, decisecond * 100000000));
        }else {
            return Err(asn1err::ValueErrorKind::ImplementationError("Local time decode is not implemented yet".to_string()))?;
        }

        return Ok(());
    }

    fn unset_value(&mut self) {
        self._value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000));
        assert_eq!(&Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000), b.value().unwrap());
    }

    #[test]
    fn test_create_default() {
        assert_eq!(
            GeneralizedTime {
                _value: None,
                format: TimeFormat::default()
            },
            GeneralizedTime::default()
        )
    }

    #[test]
    fn test_unset_value() {
        let mut b = GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000));
        b.unset_value();
        assert_eq!(None, b.value());
    }

    #[test]
    fn test_encode_generalized_time() {
        assert_eq!(vec![0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
                        0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a],
        GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)).encode().unwrap());
    }

    #[test]
    fn test_encode_generalized_time_without_deciseconds() {
        let mut gen_time = GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000));
        gen_time.set_format(TimeFormat::YYYYmmddHHMMSSZ);
        assert_eq!(vec![0x18, 0xf, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
                        0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x5a],
        gen_time.encode().unwrap());
    }

    #[test]
    fn test_decode() {
        assert_eq!(GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)),
            _parse(&[0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
            0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a]));
    }

    #[test]
    fn test_encode_without_deciseconds() {
        assert_eq!(GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms(21, 6, 27)),
                    _parse(&[0x18, 0xf, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
                    0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x5a]));
    }

    #[test]
    fn test_decode_with_excesive_bytes() {
        assert_eq!((GeneralizedTime::new(Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)), 19),
            _parse_with_consumed_octets(&[0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
            0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a, 
            0x22, 0x22, 0x22]));
    }

    #[should_panic (expected = "Invalid value: Not enough data for type")]
    #[test]
    fn test_decode_without_enough_value_octets() {
        _parse(&[0x18, 0x0e, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
            0x32, 0x31, 0x30, 0x36, 0x32, 0x37]);
    }

    #[should_panic (expected = "Invalid universal tag: Not match with expected tag")]
    #[test]
    fn test_decode_with_invalid_tag() {
        _parse(&[0x7, 0x1, 0x0]);
    }

    #[should_panic(expected="Invalid value: Error parsing to int")]
    #[test]
    fn test_decode_with_no_number_characters(){
        _parse(&[0x18, 0x11, 0x41, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
            0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a]);
    }

    #[should_panic(expected="Invalid value: Local time decode is not implemented yet")]
    #[test]
    fn test_decode_local_time(){
        _parse(&[0x18, 0x10, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 
            0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33]);
    }


    fn _parse(raw: &[u8]) -> GeneralizedTime {
        return _parse_with_consumed_octets(raw).0;
    }

    fn _parse_with_consumed_octets(raw: &[u8]) -> (GeneralizedTime, usize) {
        let mut b = GeneralizedTime::new(Utc::now());
        let consumed_octets = b.decode(raw).unwrap();
        return (b, consumed_octets);
    }
}
