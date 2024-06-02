//! Collection of the crate's error handling.

/// Enumeration of error variants which may occur on operations of this
/// crate.
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    /// TODO: This error variant is for development only and must be replaced
    /// by more appropriate error variants!
    #[error("generic error: {0}")]
    GenericError(String),
}

/// Custom error type for this crate.
pub type Result<T> = std::result::Result<T, Error>;
