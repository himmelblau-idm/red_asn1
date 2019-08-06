use failure_derive::Fail;

/// Error related to type value encoding/decoding, subtype of [`ErrorKind::InvalidValue`]
/// 
/// [`ErrorKind::InvalidValue`]: ./enum.ErrorKind.html#variant.InvalidValue
#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ValueErrorKind {
    /// There are no enough data provided for the length specified
    #[fail (display = "Invalid value: Not enough data for length")]
    NoDataForLength,

    /// There are not enough data octets for the type to be build
    #[fail (display = "Invalid value: Not enough data for type")]
    NoDataForType,

    /// There are octets which were not consumed in decoding
    #[fail (display = "Invalid value: Not all octects were consumed")]
    NoAllDataConsumed,

    /// Error formating non-utf8 characters
    #[fail (display = "Invalid value: Error formating non-utf8 characters")]
    Utf8Error,

    /// Error formating non-utf8 characters
    #[fail (display = "Invalid value: Error formating non-ascii characters")]
    AsciiError,

    /// Error parsing to int
    #[fail (display = "Invalid value: Error parsing to int")]
    ParseIntError,

    /// Error in value due to limitation of the implementation
    #[fail (display = "Invalid value: {}", _0)]
    ImplementationError(String),

}
