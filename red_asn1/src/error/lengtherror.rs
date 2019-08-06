use failure_derive::Fail;

/// Error related to type length encoding/decoding, subtype of [`ErrorKind::InvalidLength`]
/// 
/// [`ErrorKind::InvalidLength`]: ./enum.ErrorKind.html#variant.InvalidLength
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum LengthErrorKind {

    /// No length was provided
    #[fail (display = "Invalid length: Empty")]
    InvalidLengthEmpty,

    /// The size of the length octets (in long form) is incorrect
    #[fail (display = "Invalid length: Invalid length of length")]
    InvalidLengthOfLength,
}
