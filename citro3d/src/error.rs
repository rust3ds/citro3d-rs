//! General-purpose error and result types returned by public APIs of this crate.

use std::num::TryFromIntError;
use std::sync::TryLockError;

/// The common result type returned by `citro3d` functions.
pub type Result<T> = std::result::Result<T, Error>;

// TODO probably want a similar type to ctru::Result to make it easier to convert
// nonzero result codes to errors.

/// The common error type that may be returned by `citro3d` functions.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// C3D error code.
    System(libc::c_int),
    /// A C3D object or context could not be initialized.
    FailedToInitialize,
    /// A size parameter was specified that cannot be converted to the proper type.
    InvalidSize,
    /// Failed to select the given render target for drawing to.
    InvalidRenderTarget,
    /// Indicates that a reference could not be obtained because a lock is already
    /// held on the requested object.
    LockHeld,
    /// Indicates that too many vertex attributes were registered (max 12 supported).
    TooManyAttributes,
    /// Indicates that too many vertex buffer objects were registered (max 12 supported).
    TooManyBuffers,
    /// The given memory could not be converted to a physical address for sharing
    /// with the GPU. Data should be allocated with [`ctru::linear`].
    InvalidMemoryLocation,
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidSize
    }
}

impl<T> From<TryLockError<T>> for Error {
    fn from(_: TryLockError<T>) -> Self {
        Self::LockHeld
    }
}
