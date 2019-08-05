use failure_derive::Fail;
use crate::tag::*;

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum TagErrorKind {
    /// Tag cannot be decoded because there are no data
    #[fail (display = "Invalid {} tag: Empty", _0)]
    Empty(TagClass),

    /// All data was consumed but tag length octets did not finished (high tag number form)
    #[fail (display = "Invalid {} tag: High form number unfinished", _0)]
    HighFormNumberUnfinished(TagClass),

    /// Tag decoded is not the expected for the type
    #[fail (display = "Invalid {} tag: Not match with expected tag", _0)]
    Unmatched(TagClass),
}