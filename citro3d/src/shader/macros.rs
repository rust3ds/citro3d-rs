//! Helper macros for working with shader data.

/// Helper macro for including a file as bytes that are correctly aligned for
/// use as a [`Library`](super::Library).
#[macro_export]
macro_rules! include_aligned_bytes {
    ($path:expr) => {{
        // const block expression to encapsulate the static
        use $crate::shader::macros::AlignedAs;

        // this assignment is made possible by CoerceUnsized
        const ALIGNED: &AlignedAs<u32, [u8]> = &AlignedAs {
            _align: [],
            bytes: *include_bytes!($path),
        };

        &ALIGNED.bytes
    }};
}

/// Helper struct to [`include_bytes`] aligned as a specific type.
#[repr(C)] // guarantee 'bytes' comes after '_align'
#[doc(hidden)]
pub struct AlignedAs<Align, Bytes: ?Sized> {
    pub _align: [Align; 0],
    pub bytes: Bytes,
}
