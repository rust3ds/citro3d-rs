//! General-purpose error and result types returned by public APIs of this crate.

/// The common result type returned by `citro2d` functions.
pub type Result<T> = std::result::Result<T, Error>;

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A C2D object or context could not be initialized.
    FailedToInitialize,
}
