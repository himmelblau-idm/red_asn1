use chrono::prelude::*;
use std::default::Default;

/// Enum to indicate the format of time used by GeneralizedTime
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TimeFormat {
    /// Format YYYYmmddHHMMSS.DZ (Eg. "19851106210627.3Z")
    YYYYmmddHHMMSS_DZ,

    /// Format YYYYmmddHHMMSSZ (Eg. "19851106210627Z")
    YYYYmmddHHMMSSZ,
}

impl Default for TimeFormat {
    fn default() -> Self {
        TimeFormat::YYYYmmddHHMMSS_DZ
    }
}

impl TimeFormat {
    pub(crate) fn format_to_string(&self, datetime: &DateTime<Utc>) -> String {
        match *self {
            TimeFormat::YYYYmmddHHMMSS_DZ => {
                TimeFormat::_format_YYYYmmddHHMMSS_DZ(datetime)
            }
            TimeFormat::YYYYmmddHHMMSSZ => {
                datetime.format("%Y%m%d%H%M%SZ").to_string()
            }
        }
    }

    #[allow(non_snake_case)]
    fn _format_YYYYmmddHHMMSS_DZ(datetime: &DateTime<Utc>) -> String {
        let decisecond: u8 = (datetime.nanosecond() / 100000000) as u8;
        let formatted_string = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}.{}Z",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second(),
            decisecond
        );
        return formatted_string;
    }
}
