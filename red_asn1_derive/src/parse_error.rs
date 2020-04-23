use std::fmt;

pub type ParseComponentResult<T> = Result<T, ParseComponentError>;


#[derive(Clone, Debug)]
pub enum ParseComponentError {
    NotFoundAttributeTag,
    InvalidTagNumberValue,
    UnknownAttribute,
    InvalidFieldType,
    NotStruct
}

impl fmt::Display for ParseComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
