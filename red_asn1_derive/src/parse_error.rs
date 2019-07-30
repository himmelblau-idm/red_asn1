use std::fmt;
use failure::*;
use failure_derive::Fail;

pub type ParseComponentResult<T> = Result<T, ParseComponentError>;


#[derive(Debug)]
pub struct ParseComponentError {
    inner: Context<ParseComponentErrorKind>
}

#[derive(Clone, Debug, Fail)]
pub enum ParseComponentErrorKind {
    #[fail (display = "Not found attribute tag")]
    NotFoundAttributeTag,
    #[fail (display = "Invalid tag number value: must a number between 0 and 255")]
    InvalidTagNumberValue,
    #[fail (display = "Unknown attribute")]
    UnknownAttribute,
    #[fail (display = "Invalid field type")]
    InvalidFieldType,
    #[fail (display = "Sequence is not an struct")]
    NotStruct
}


impl ParseComponentError {

    pub fn kind(&self) -> &ParseComponentErrorKind {
        return self.inner.get_context();
    }

}

impl Fail for ParseComponentError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for ParseComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl std::convert::From<ParseComponentErrorKind> for ParseComponentError {
    fn from(kind: ParseComponentErrorKind) -> ParseComponentError {
        return ParseComponentError {
            inner: Context::new(kind)
        };
    }
}

impl std::convert::From<Context<ParseComponentErrorKind>> for ParseComponentError {
    fn from(inner: Context<ParseComponentErrorKind>) -> ParseComponentError {
        return ParseComponentError { inner };
    }
}