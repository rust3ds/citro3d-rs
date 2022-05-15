use citro3d_sys::{GX_TRANSFER_FORMAT, GX_TRANSFER_IN_FORMAT, GX_TRANSFER_OUT_FORMAT};

use super::ColorFormat;

/// Control flags for a GX data transfer.
#[derive(Default, Clone, Copy)]
pub struct Flags(u32);

impl Flags {
    /// Set the input format of the data transfer.
    #[must_use]
    pub fn in_format(self, fmt: Format) -> Self {
        Self(self.0 | GX_TRANSFER_IN_FORMAT(fmt as GX_TRANSFER_FORMAT))
    }

    /// Set the output format of the data transfer.
    #[must_use]
    pub fn out_format(self, fmt: Format) -> Self {
        Self(self.0 | GX_TRANSFER_OUT_FORMAT(fmt as GX_TRANSFER_FORMAT))
    }

    #[must_use]
    pub fn bits(self) -> u32 {
        self.0
    }
}

/// The color format to use when transferring data to/from the GPU.
///
/// NOTE: this a distinct type from [`ColorFormat`] because they are not implicitly
/// convertible to one another. Use [`From::from`] to get the [`Format`] corresponding
/// to a given [`ColorFormat`].
#[repr(u32)]
pub enum Format {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha.
    RGBA8 = citro3d_sys::GX_TRANSFER_FMT_RGBA8,
    /// 8-bit Red + 8-bit Green + 8-bit Blue.
    RGB8 = citro3d_sys::GX_TRANSFER_FMT_RGB8,
    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha.
    RGB565 = citro3d_sys::GX_TRANSFER_FMT_RGB565,
    /// 5-bit Red + 6-bit Green + 5-bit Blue.
    RGB5A1 = citro3d_sys::GX_TRANSFER_FMT_RGB5A1,
    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha.
    RGBA4 = citro3d_sys::GX_TRANSFER_FMT_RGBA4,
}

impl From<ColorFormat> for Format {
    fn from(color_fmt: ColorFormat) -> Self {
        match color_fmt {
            ColorFormat::RGBA8 => Self::RGBA8,
            ColorFormat::RGB8 => Self::RGB8,
            ColorFormat::RGBA5551 => Self::RGB5A1,
            ColorFormat::RGB565 => Self::RGB565,
            ColorFormat::RGBA4 => Self::RGBA4,
        }
    }
}
