use std::convert::From;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TagType {
    Primitive = 0b0,
    Constructed = 0b1
}

impl From<u8> for TagType {
    fn from(u: u8) -> TagType {
        match u & 0x01 {
            0 => TagType::Primitive,
            1 => TagType::Constructed,
            _ => unreachable!()
        }
    }
}