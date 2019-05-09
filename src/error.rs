use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Asn1Error {
    message: String
}

impl Asn1Error {
    pub fn new(message: String) -> Asn1Error {
        return Asn1Error{
            message
        };
    }
}

impl fmt::Display for Asn1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Asn1Error")
    }
}

impl error::Error for Asn1Error {
    fn description(&self) -> &str {
        return &self.message;
    }

    fn cause(&self) -> Option<&error::Error> {
        return None;
    }
}

impl std::convert::From<ascii::ToAsciiCharError> for Asn1Error {
    fn from(_inner: ascii::ToAsciiCharError) -> Asn1Error {
        return Asn1Error::new("Invalid value: Error formating non-ascii characters".to_string());
    }
}

impl std::convert::From<std::str::Utf8Error> for Asn1Error {
    fn from(_inner: std::str::Utf8Error) -> Asn1Error {
        return Asn1Error::new("Invalid value: Error formating non-utf8 characters".to_string());
    }
}

impl std::convert::From<std::num::ParseIntError> for Asn1Error {
    fn from(_inner: std::num::ParseIntError) -> Asn1Error {
        return Asn1Error::new("Invalid value: Error parsing to int".to_string());
    }
}
