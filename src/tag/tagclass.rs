use std::convert::From;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TagClass {
    Universal = 0b00,
    Application = 0b01,
    Context = 0b10,
    Private = 0b11
}


impl From<u8> for TagClass {
    fn from(u: u8) -> TagClass {
        match u & 0x03 {
            0b00 => TagClass::Universal,
            0b01 => TagClass::Application,
            0b10 => TagClass::Context,
            0b11 => TagClass::Private,
            _ => unreachable!()
        }
    }
}
