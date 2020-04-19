mod error;
pub use error::Error;
pub use error::*;

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag::TagClass;

    #[test]
    fn raise_empty_tag_error() {
        let error_kind = super::Error::from(Error::EmptyTag(TagClass::Context));

        match error_kind {
            Error::EmptyTag(tag_class) => assert_eq!(TagClass::Context, tag_class),
            _ => unreachable!(),
        }
    }
}
