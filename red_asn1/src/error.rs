use std::fmt;
use failure::*;
use failure_derive::Fail;
use std::result;
use crate::tag::*;

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