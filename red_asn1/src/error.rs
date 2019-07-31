use std::fmt;
use failure::*;
use failure_derive::Fail;
use std::result;

pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>
}

#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum ErrorKind {
    #[fail (display = "Invalid type tag: Empty")]
    InvalidTypeTagEmpty,
    #[fail (display = "Invalid type tag: High form number unfinished")]
    InvalidTypeTagHighFormNumberUnfinished,
    #[fail (display = "Invalid type tag: Not match with expected tag")]
    InvalidTypeTagUnmatched,
    #[fail (display = "Invalid context tag: Empty tag")]
    InvalidContextTagEmpty,
    #[fail (display = "Invalid context tag: High form number unfinished")]
    InvalidContextTagHighFormNumberUnfinished,
    #[fail (display = "Invalid context tag: Not match with expected tag")]
    InvalidContextTagUnmatched,
    #[fail (display = "Invalid application tag: Empty tag")]
    InvalidApplicationTagEmpty,
    #[fail (display = "Invalid application tag: High form number unfinished")]
    InvalidApplicationTagHighFormNumberUnfinished,
    #[fail (display = "Invalid application tag: Not match with expected tag")]
    InvalidApplicationTagUnmatched,
    #[fail (display = "Invalid length: Empty")]
    InvalidLengthEmpty,
    #[fail (display = "Invalid length: Invalid length of length")]
    InvalidLengthOfLength,
    #[fail (display = "Invalid value: Not enough data for length")]
    NoDataForLength,
    #[fail (display = "Invalid value: Not enough data for type")]
    NoDataForType,
    #[fail (display = "Invalid value: Not all octects were consumed")]
    NoAllDataConsumed,
    #[fail (display = "No value provided")]
    NoValue,
    #[fail (display = "Invalid value: {}", _0)]
    InvalidValue(String),
    #[fail (display = "No component with such identifier")]
    NoComponent,
    #[fail (display = "{}::{} => {}", _0,_1,_2)]
    SequenceFieldError(String, String, Box<ErrorKind>),
    #[fail (display = "{} => {}", _0,_1)]
    SequenceError(String, Box<ErrorKind>)
}

impl Error {

    pub fn kind(&self) -> &ErrorKind {
        return self.inner.get_context();
    }

}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl std::convert::From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        return Self {
            inner: Context::new(kind)
        };
    }
}

impl std::convert::From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        return Self { inner };
    }
}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(_inner: std::str::Utf8Error) -> Self {
        return Self {
            inner: Context::new(ErrorKind::InvalidValue("Error formating non-utf8 characters".to_string()))
        };
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(_inner: std::num::ParseIntError) -> Self {
        return Self {
            inner: Context::new(ErrorKind::InvalidValue("Error parsing to int".to_string()))
        };
    }
}