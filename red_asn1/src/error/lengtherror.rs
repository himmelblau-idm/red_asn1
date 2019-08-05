use failure_derive::Fail;

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum LengthErrorKind {

    /// No length was provided
    #[fail (display = "Invalid length: Empty")]
    InvalidLengthEmpty,

    /// The size of the length octets (in long form) is incorrect
    #[fail (display = "Invalid length: Invalid length of length")]
    InvalidLengthOfLength,
}
