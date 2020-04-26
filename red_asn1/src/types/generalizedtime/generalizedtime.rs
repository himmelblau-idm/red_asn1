use super::TimeFormat;
use crate::error as asn1err;
use crate::tag::Tag;
use crate::traits::Asn1Object;
use chrono::prelude::*;
use std::str;

pub static GENERALIZED_TIME_TAG_NUMBER: u8 = 0x18;

/// Class to build/parse GeneralizedTime ASN1
#[derive(Debug, PartialEq)]
pub struct GeneralizedTime {
    pub time: DateTime<Utc>,
    pub format: TimeFormat,
}

impl Asn1Object for GeneralizedTime {
    fn tag() -> Tag {
        return Tag::new_primitive_universal(GENERALIZED_TIME_TAG_NUMBER);
    }

    fn build_value(&self) -> Vec<u8> {
        return self.format.format_to_string(&self.time).into_bytes();
    }

    fn parse_value(&mut self, raw: &[u8]) -> asn1err::Result<()> {
        if raw.len() < 15 {
            return Err(asn1err::Error::NoDataForType)?;
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
            self.time = Utc.ymd(year, month, day).and_hms_nano(
                hour,
                minute,
                second,
                decisecond * 100000000,
            );
        } else {
            return Err(asn1err::Error::ImplementationError(
                "Local time parse is not implemented yet".to_string(),
            ))?;
        }

        return Ok(());
    }
}

impl Default for GeneralizedTime {
    fn default() -> Self {
        return Self {
            time: Utc.timestamp(0, 0),
            format: TimeFormat::default(),
        };
    }
}

impl From<DateTime<Utc>> for GeneralizedTime {
    fn from(time: DateTime<Utc>) -> Self {
        return Self {
            time,
            format: TimeFormat::default(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = GeneralizedTime::from(
            Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000),
        );
        assert_eq!(
            &Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000),
            &b.time
        );
    }

    #[test]
    fn test_create_default() {
        assert_eq!(
            GeneralizedTime {
                time: Utc.timestamp(0, 0),
                format: TimeFormat::default()
            },
            GeneralizedTime::default()
        )
    }

    #[test]
    fn test_build_generalized_time() {
        assert_eq!(
            vec![
                0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36,
                0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a
            ],
            GeneralizedTime::from(
                Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)
            )
            .build()
        );
    }

    #[test]
    fn test_build_generalized_time_without_deciseconds() {
        let mut gen_time = GeneralizedTime::from(
            Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000),
        );
        gen_time.format = TimeFormat::YYYYmmddHHMMSSZ;
        assert_eq!(
            vec![
                0x18, 0xf, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36,
                0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x5a
            ],
            gen_time.build()
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            GeneralizedTime::from(
                Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)
            ),
            GeneralizedTime::parse(&[
                0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36,
                0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a
            ])
            .unwrap()
            .1
        );
    }

    #[test]
    fn test_build_without_deciseconds() {
        assert_eq!(
            GeneralizedTime::from(Utc.ymd(1985, 11, 6).and_hms(21, 6, 27)),
            GeneralizedTime::parse(&[
                0x18, 0xf, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36,
                0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x5a
            ])
            .unwrap()
            .1
        );
    }

    #[test]
    fn test_parse_with_excesive_bytes() {
        let rest: &[u8] = &[0x22, 0x22, 0x22];
        assert_eq!(
            (
                rest,
                GeneralizedTime::from(
                    Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)
                ),
            ),
            GeneralizedTime::parse(&[
                0x18, 0x11, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36,
                0x32, 0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a, 0x22,
                0x22, 0x22
            ])
            .unwrap()
        );
    }

    #[should_panic(expected = "NoDataForType")]
    #[test]
    fn test_parse_without_enough_value_octets() {
        GeneralizedTime::parse(&[
            0x18, 0x0e, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 0x32,
            0x31, 0x30, 0x36, 0x32, 0x37,
        ])
        .unwrap();
    }

    #[should_panic(expected = "UnmatchedTag")]
    #[test]
    fn test_parse_with_invalid_tag() {
        GeneralizedTime::parse(&[0x7, 0x1, 0x0]).unwrap();
    }

    #[should_panic(expected = "ParseIntError")]
    #[test]
    fn test_parse_with_no_number_characters() {
        GeneralizedTime::parse(&[
            0x18, 0x11, 0x41, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 0x32,
            0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33, 0x5a,
        ])
        .unwrap();
    }

    #[should_panic(expected = "ImplementationError")]
    #[test]
    fn test_parse_local_time() {
        GeneralizedTime::parse(&[
            0x18, 0x10, 0x31, 0x39, 0x38, 0x35, 0x31, 0x31, 0x30, 0x36, 0x32,
            0x31, 0x30, 0x36, 0x32, 0x37, 0x2e, 0x33,
        ])
        .unwrap();
    }
}
