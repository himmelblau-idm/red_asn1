use std::fmt;
use std::result;
use ascii;

use super::*;

/// Result that encapsulates the Error type of this library
pub type Result<T> = result::Result<T, Error>;

/// Error in ASN1-DER decode/encode operations
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Error decoding tag
    InvalidTag(Box<TagErrorKind>),
    
    /// Error decoding length
    InvalidLength(Box<LengthErrorKind>),

    /// Error decoding value
    InvalidValue(Box<ValueErrorKind>),

    /// No value was provided to encode
    NoValue,

    /// No found component with the identifier specified
    NoComponent,

    /// Error in a field of a sequence
    SequenceFieldError(String, String, Box<Error>),

    /// Error while processing a sequence
    SequenceError(String, Box<Error>)
}

impl From<ValueErrorKind> for Error {
    fn from(kind: ValueErrorKind) -> Self {
        return Error::InvalidValue(Box::new(kind));
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl From<TagErrorKind> for Error {
    fn from(kind: TagErrorKind) -> Self {
        return Self::InvalidTag(Box::new(kind));
    }
}



impl From<LengthErrorKind> for Error {
    fn from(kind: LengthErrorKind) -> Self {
        return Self::InvalidLength(Box::new(kind));
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
