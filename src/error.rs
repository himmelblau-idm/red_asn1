use std::fmt;
use failure::*;
use failure_derive::Fail;

pub type Asn1Result<T> = Result<T, Asn1Error>;


#[derive(Debug)]
pub struct Asn1Error {
    inner: Context<Asn1ErrorKind>
}

#[derive(Clone, Debug, Fail)]
pub enum Asn1ErrorKind {
    #[fail (display = "Invalid tag: Empty")]
    InvalidTagEmpty,
    #[fail (display = "Invalid tag: Invalid tag number")]
    InvalidTagNumber,
    #[fail (display = "Invalid tag: Not valid tag for type")]
    InvalidTypeTag,
    #[fail (display = "Invalid tag: Not valid tag for context")]
    InvalidContextTag,
    #[fail (display = "Invalid length: Empty")]
    InvalidLengthEmpty,
    #[fail (display = "Invalid length: Invalid length of length")]
    InvalidLengthOfLength,
    #[fail (display = "Invalid value: Not enough data for length")]
    NoDataForLength,
    #[fail (display = "Invalid value: Not enough data for type")]
    NoDataForType,
    #[fail (display = "No value provided")]
    NoValue,
    #[fail (display = "Invalid value: {}", _0)]
    InvalidValue(String),
    #[fail (display = "No component with such identifier")]
    NoComponent
}

impl Asn1Error {

    pub fn kind(&self) -> &Asn1ErrorKind {
        return self.inner.get_context();
    }

}

impl Fail for Asn1Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Asn1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl std::convert::From<Asn1ErrorKind> for Asn1Error {
    fn from(kind: Asn1ErrorKind) -> Asn1Error {
        return Asn1Error {
            inner: Context::new(kind)
        };
    }
}

impl std::convert::From<Context<Asn1ErrorKind>> for Asn1Error {
    fn from(inner: Context<Asn1ErrorKind>) -> Asn1Error {
        return Asn1Error { inner };
    }
}

impl std::convert::From<std::str::Utf8Error> for Asn1Error {
    fn from(_inner: std::str::Utf8Error) -> Asn1Error {
        return Asn1Error {
            inner: Context::new(Asn1ErrorKind::InvalidValue("Error formating non-utf8 characters".to_string()))
        };
    }
}

impl std::convert::From<std::num::ParseIntError> for Asn1Error {
    fn from(_inner: std::num::ParseIntError) -> Asn1Error {
        return Asn1Error {
            inner: Context::new(Asn1ErrorKind::InvalidValue("Error parsing to int".to_string()))
        };
    }
}