//! General-purpose error and result types returned by public APIs of this crate.

use std::num::TryFromIntError;

/// The common result type returned by `citro3d` functions.
pub type Result<T> = std::result::Result<T, Error>;

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A C3D object or context could not be initialized.
    FailedToInitialize,
    /// A size parameter was specified that cannot be converted to the proper type.
    InvalidSize,
    /// Failed to select the given render target for drawing to.
    InvalidRenderTarget,
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidSize
    }
}
