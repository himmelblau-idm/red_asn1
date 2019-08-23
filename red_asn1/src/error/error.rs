use std::fmt;
use failure::*;
use failure_derive::Fail;
use std::result;
use ascii;

use super::*;

/// Result that encapsulates the Error type of this library
pub type Result<T> = result::Result<T, Error>;

/// Error in ASN1-DER decode/encode operations
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>
}

/// Type of error
#[derive(Clone, Debug, PartialEq, Fail)]
pub enum ErrorKind {
    /// Error decoding tag
    #[fail (display = "{}", _0)]
    InvalidTag(Box<TagErrorKind>),
    
    /// Error decoding length
    #[fail (display = "{}", _0)]
    InvalidLength(Box<LengthErrorKind>),

    /// Error decoding value
    #[fail (display = "{}", _0)]
    InvalidValue(Box<ValueErrorKind>),

    /// No value was provided to encode
    #[fail (display = "No value provided")]
    NoValue,

    /// No found component with the identifier specified
    #[fail (display = "No component with such identifier")]
    NoComponent,

    /// Error in a field of a sequence
    #[fail (display = "{}::{} => {}", _0,_1,_2)]
    SequenceFieldError(String, String, Box<ErrorKind>),

    /// Error while processing a sequence
    #[fail (display = "{} => {}", _0,_1)]
    SequenceError(String, Box<ErrorKind>)
}

impl From<ValueErrorKind> for ErrorKind {
    fn from(kind: ValueErrorKind) -> Self {
        return ErrorKind::InvalidValue(Box::new(kind));
    }
}


impl Error {

    pub fn kind(&self) -> &ErrorKind {
        return self.inner.get_context();
    }

}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
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


impl From<TagErrorKind> for Error {
    fn from(kind: TagErrorKind) -> Self {
        return Self {
            inner: Context::new(ErrorKind::InvalidTag(Box::new(kind)))
        };
    }
}

impl From<ValueErrorKind> for Error {
    fn from(kind: ValueErrorKind) -> Self {
        return Self {
            inner: Context::new(ErrorKind::InvalidValue(Box::new(kind)))
        };
    }
}

impl From<LengthErrorKind> for Error {
    fn from(kind: LengthErrorKind) -> Self {
        return Self {
            inner: Context::new(ErrorKind::InvalidLength(Box::new(kind)))
        };
    }
}

impl std::convert::From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        return Self { inner };
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_inner: std::str::Utf8Error) -> Self {
        return Self::from(ValueErrorKind::Utf8Error);
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_inner: std::string::FromUtf8Error) -> Self {
        return Self::from(ValueErrorKind::Utf8Error);
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_inner: std::num::ParseIntError) -> Self {
        return Self::from(ValueErrorKind::ParseIntError);
    }
}

impl From<ascii::ToAsciiCharError> for Error {
    fn from(_inner: ascii::ToAsciiCharError) -> Self {
        return Self::from(ValueErrorKind::AsciiError);
    }
}
