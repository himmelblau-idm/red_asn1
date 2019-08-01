use std::default::Default;
use chrono::prelude::*;

/// Enum to indicate the format of time used by GeneralizedTime
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TimeFormat {
    /// Format YYYYmmddHHMMSS.DZ
    YYYYmmddHHMMSS_DZ,

    /// Format YYYYmmddHHMMSSZ
    YYYYmmddHHMMSSZ
}

impl Default for TimeFormat {
    fn default() -> Self { TimeFormat::YYYYmmddHHMMSS_DZ }
}

impl TimeFormat {

    /// Transforms a DateTime into a String with the specified format
    /// 
    /// # Example
    /// ```
    /// use chrono::prelude::*;
    /// use red_asn1::TimeFormat;
    /// 
    /// assert_eq!(
    ///     "19851106210627.3Z",
    ///     TimeFormat::YYYYmmddHHMMSS_DZ.format_to_string(
    ///         &Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)
    ///     )
    /// );
    /// 
    /// assert_eq!(
    ///     "19851106210627Z",
    ///     TimeFormat::YYYYmmddHHMMSSZ.format_to_string(
    ///         &Utc.ymd(1985, 11, 6).and_hms_nano(21, 6, 27, 300000000)
    ///     )
    /// );
    /// ```
    pub fn format_to_string(&self, datetime: &DateTime<Utc>) -> String {
        match *self {
            TimeFormat::YYYYmmddHHMMSS_DZ => TimeFormat::_format_YYYYmmddHHMMSS_DZ(datetime),
            TimeFormat::YYYYmmddHHMMSSZ => datetime.format("%Y%m%d%H%M%SZ").to_string()
        }
    }

    #[allow(non_snake_case)]
    fn _format_YYYYmmddHHMMSS_DZ(datetime: &DateTime<Utc>) -> String {
        let decisecond: u8 = (datetime.nanosecond() / 100000000) as u8;
        let formatted_string = format!("{:04}{:02}{:02}{:02}{:02}{:02}.{}Z", 
        datetime.year(), datetime.month(), datetime.day(), 
        datetime.hour(), datetime.minute(), datetime.second(), decisecond);
        return formatted_string;
    }
}