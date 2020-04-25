use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;


#[derive(Clone, Debug)]
pub enum ParseError {
    NotFoundAttributeTag,
    InvalidTagNumberValue,
    UnknownAttribute,
    InvalidFieldType,
    NotStruct
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
