use std::fmt;
use failure::*;
use failure_derive::Fail;
use std::result;
use crate::tag::*;
use ascii;

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
    
    /// Error decoding value
    #[fail (display = "{}", _0)]
    InvalidValue(Box<ValueErrorKind>),

    /// No value was provided to encode
    #[fail (display = "No value provided")]
    NoValue,

    #[fail (display = "Invalid length: Empty")]
    InvalidLengthEmpty,
    #[fail (display = "Invalid length: Invalid length of length")]
    InvalidLengthOfLength,

    #[fail (display = "No component with such identifier")]
    NoComponent,
    #[fail (display = "{}::{} => {}", _0,_1,_2)]
    SequenceFieldError(String, String, Box<ErrorKind>),
    #[fail (display = "{} => {}", _0,_1)]
    SequenceError(String, Box<ErrorKind>)
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum TagErrorKind {
    /// Tag cannot be decoded because there are no data
    #[fail (display = "Invalid {} tag: Empty", _0)]
    Empty(TagClass),

    /// All data was consumed but tag length octets did not finished (high tag number form)
    #[fail (display = "Invalid {} tag: High form number unfinished", _0)]
    HighFormNumberUnfinished(TagClass),

    /// Tag decoded is not the expected for the type
    #[fail (display = "Invalid {} tag: Not match with expected tag", _0)]
    Unmatched(TagClass),
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ValueErrorKind {
    /// There are no enough data provided for the length specified
    #[fail (display = "Invalid value: Not enough data for length")]
    NoDataForLength,

    /// There are not enough data octets for the type to be build
    #[fail (display = "Invalid value: Not enough data for type")]
    NoDataForType,

    /// There are octets which were not consumed in decoding
    #[fail (display = "Invalid value: Not all octects were consumed")]
    NoAllDataConsumed,

    /// Error formating non-utf8 characters
    #[fail (display = "Invalid value: Error formating non-utf8 characters")]
    Utf8Error,

    /// Error formating non-utf8 characters
    #[fail (display = "Invalid value: Error formating non-ascii characters")]
    AsciiError,

    /// Error parsing to int
    #[fail (display = "Invalid value: Error parsing to int")]
    ParseIntError,

    /// Error in value due to limitation of the implementation
    #[fail (display = "Invalid value: {}", _0)]
    ImplementationError(String),

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

impl From<ValueErrorKind> for ErrorKind {
    fn from(kind: ValueErrorKind) -> Self {
        return ErrorKind::InvalidValue(Box::new(kind));
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn raise_empty_tag_error() {
        let error_kind = super::Error::from(TagErrorKind::Empty(TagClass::Context));

        match error_kind.kind() {
            ErrorKind::InvalidTag(tag_error_kind) => {
                match **tag_error_kind {
                    TagErrorKind::Empty(tag_class) => {
                        assert_eq!(TagClass::Context, tag_class)
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }

    }

}