mod error;
pub use error::*;
pub use error::Error;

mod tagerror;
pub use tagerror::*;

mod valueerror;
pub use valueerror::*;


#[cfg(test)]
mod test {
    use super::*;
    use crate::tag::TagClass;

    #[test]
    fn raise_empty_tag_error() {
        let error_kind = super::Error::from(TagErrorKind::Empty(TagClass::Context));

        match error_kind {
            Error::InvalidTag(tag_error_kind) => {
                match *tag_error_kind {
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
