use ascii;
use std::fmt;
use std::result;

use super::*;

/// Result that encapsulates the Error type of this library
pub type Result<T> = result::Result<T, Error>;

/// Error in ASN1-DER decode/encode operations
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Error decoding tag
    InvalidTag(Box<TagErrorKind>),

    /// No length was provided
    LengthEmpty,

    /// The size of the length is higher than the available octets
    NotEnoughLengthOctects,

    /// No value was provided to encode
    NoValue,

    /// No found component with the identifier specified
    NoComponent,

    /// Error in a field of a sequence
    SequenceFieldError(String, String, Box<Error>),

    /// Error while processing a sequence
    SequenceError(String, Box<Error>),

    /// There are no enough data provided for the length specified
    NoDataForLength,

    /// There are not enough data octets for the type to be build
    NoDataForType,

    /// There are octets which were not consumed in decoding
    NoAllDataConsumed,

    /// Error formating non-utf8 characters
    Utf8Error,

    /// Error formating non-utf8 characters
    AsciiError,

    /// Error parsing to int
    ParseIntError,

    /// Error in value due to limitation of the implementation
    ImplementationError(String),

    /// Error in value due to a constraint in the type
    ConstraintError(String),
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

impl From<std::str::Utf8Error> for Error {
    fn from(_inner: std::str::Utf8Error) -> Self {
        return Self::Utf8Error;
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_inner: std::string::FromUtf8Error) -> Self {
        return Self::Utf8Error;
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_inner: std::num::ParseIntError) -> Self {
        return Self::ParseIntError;
    }
}

impl From<ascii::ToAsciiCharError> for Error {
    fn from(_inner: ascii::ToAsciiCharError) -> Self {
        return Self::AsciiError;
    }
}
