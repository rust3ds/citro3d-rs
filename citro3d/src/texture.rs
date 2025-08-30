use ctru_sys;

/// Texture filters.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_FILTER_PARAM")]
pub enum Filter {
    #[doc(alias = "GPU_NEAREST")]
    Nearest = ctru_sys::GPU_NEAREST,

    #[doc(alias = "GPU_LINEAR")]
    Linear = ctru_sys::GPU_LINEAR,
}

impl TryFrom<u8> for Filter {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_NEAREST => Ok(Self::Nearest),
            ctru_sys::GPU_LINEAR => Ok(Self::Linear),
            _ => Err("invalid value for FilterParam".to_string()),
        }
    }
}

/// Texture wrap modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_WRAP_PARAM")]
pub enum Wrap {
    #[doc(alias = "GPU_CLAMP_TO_EDGE")]
    ClampToEdge = ctru_sys::GPU_CLAMP_TO_EDGE,

    #[doc(alias = "GPU_CLAMP_TO_BORDER")]
    ClampToBorder = ctru_sys::GPU_CLAMP_TO_BORDER,

    #[doc(alias = "GPU_REPEAT")]
    Repeat = ctru_sys::GPU_REPEAT,

    #[doc(alias = "GPU_MIRRORED_REPEAT")]
    MirroredRepeat = ctru_sys::GPU_MIRRORED_REPEAT,
}

impl TryFrom<u8> for Wrap {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_CLAMP_TO_EDGE => Ok(Self::ClampToEdge),
            ctru_sys::GPU_CLAMP_TO_BORDER => Ok(Self::ClampToBorder),
            ctru_sys::GPU_REPEAT => Ok(Self::Repeat),
            ctru_sys::GPU_MIRRORED_REPEAT => Ok(Self::MirroredRepeat),
            _ => Err("invalid value for Wrap".to_string()),
        }
    }
}

/// Texture modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_MODE_PARAM")]
pub enum Mode {
    #[doc(alias = "GPU_TEX_2D")]
    Tex2D = ctru_sys::GPU_TEX_2D,

    #[doc(alias = "GPU_TEX_CUBE_MAP")]
    CubeMap = ctru_sys::GPU_TEX_CUBE_MAP,

    #[doc(alias = "GPU_TEX_SHADOW_2D")]
    Shadow2D = ctru_sys::GPU_TEX_SHADOW_2D,

    #[doc(alias = "GPU_TEX_PROJECTION")]
    Projection = ctru_sys::GPU_TEX_PROJECTION,

    #[doc(alias = "GPU_TEX_SHADOW_CUBE")]
    ShadowCube = ctru_sys::GPU_TEX_SHADOW_CUBE,

    #[doc(alias = "GPU_TEX_DISABLED")]
    Disabled = ctru_sys::GPU_TEX_DISABLED,
}

impl TryFrom<u8> for Mode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEX_2D => Ok(Self::Tex2D),
            ctru_sys::GPU_TEX_CUBE_MAP => Ok(Self::CubeMap),
            ctru_sys::GPU_TEX_SHADOW_2D => Ok(Self::Shadow2D),
            ctru_sys::GPU_TEX_PROJECTION => Ok(Self::Projection),
            ctru_sys::GPU_TEX_SHADOW_CUBE => Ok(Self::ShadowCube),
            ctru_sys::GPU_TEX_DISABLED => Ok(Self::Disabled),
            _ => Err("invalid value for Mode".to_string()),
        }
    }
}

/// Supported texture units.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXUNIT")]
pub enum Unit {
    #[doc(alias = "GPU_TEXUNIT0")]
    Unit0 = ctru_sys::GPU_TEXUNIT0,

    #[doc(alias = "GPU_TEXUNIT1")]
    Unit1 = ctru_sys::GPU_TEXUNIT1,

    #[doc(alias = "GPU_TEXUNIT2")]
    Unit2 = ctru_sys::GPU_TEXUNIT2,
}

impl TryFrom<u8> for Unit {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEXUNIT0 => Ok(Self::Unit0),
            ctru_sys::GPU_TEXUNIT1 => Ok(Self::Unit1),
            ctru_sys::GPU_TEXUNIT2 => Ok(Self::Unit2),
            _ => Err("invalid value for Unit".to_string()),
        }
    }
}

/// Supported texture formats.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXCOLOR")]
pub enum ColorFormat {
    /// 8-bit Red + 8-bit Green + 8-bit Blue + 8-bit Alpha
    #[doc(alias = "GPU_RGBA8")]
    Rgba8 = ctru_sys::GPU_RGBA8,

    /// 8-bit Red + 8-bit Green + 8-bit Blue    #[doc(alias = "GPU_RGB8")]
    Rgb8 = ctru_sys::GPU_RGB8,

    /// 5-bit Red + 5-bit Green + 5-bit Blue + 1-bit Alpha
    #[doc(alias = "GPU_RGBA5551")]
    Rgba5551 = ctru_sys::GPU_RGBA5551,

    /// 5-bit Red + 6-bit Green + 5-bit Blue
    #[doc(alias = "GPU_RGB565")]
    Rgb565 = ctru_sys::GPU_RGB565,

    /// 4-bit Red + 4-bit Green + 4-bit Blue + 4-bit Alpha
    #[doc(alias = "GPU_RGBA4")]
    Rgba4 = ctru_sys::GPU_RGBA4,

    /// 8-bit Luminance + 8-bit Alpha
    #[doc(alias = "GPU_LA8")]
    La8 = ctru_sys::GPU_LA8,

    /// 8-bit Hi + 8-bit Lo
    #[doc(alias = "GPU_HILO8")]
    Hilo8 = ctru_sys::GPU_HILO8,

    /// 8-bit Luminance
    #[doc(alias = "GPU_L8")]
    L8 = ctru_sys::GPU_L8,

    /// 8-bit Alpha
    #[doc(alias = "GPU_A8")]
    A8 = ctru_sys::GPU_A8,

    /// 4-bit Luminance + 4-bit Alpha
    #[doc(alias = "GPU_LA4")]
    La4 = ctru_sys::GPU_LA4,

    /// 4-bit Luminance
    #[doc(alias = "GPU_L4")]
    L4 = ctru_sys::GPU_L4,

    /// 4-bit Alpha
    #[doc(alias = "GPU_A4")]
    A4 = ctru_sys::GPU_A4,

    /// ETC1 texture compression
    #[doc(alias = "GPU_ETC1")]
    Etc1 = ctru_sys::GPU_ETC1,

    /// ETC1 texture compression + 4-bit Alpha
    #[doc(alias = "GPU_ETC1A4")]
    Etc1A4 = ctru_sys::GPU_ETC1A4,
}

impl TryFrom<u8> for ColorFormat {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_RGBA8 => Ok(Self::Rgba8),
            ctru_sys::GPU_RGB8 => Ok(Self::Rgb8),
            ctru_sys::GPU_RGBA5551 => Ok(Self::Rgba5551),
            ctru_sys::GPU_RGB565 => Ok(Self::Rgb565),
            ctru_sys::GPU_RGBA4 => Ok(Self::Rgba4),
            ctru_sys::GPU_LA8 => Ok(Self::La8),
            ctru_sys::GPU_HILO8 => Ok(Self::Hilo8),
            ctru_sys::GPU_L8 => Ok(Self::L8),
            ctru_sys::GPU_A8 => Ok(Self::A8),
            ctru_sys::GPU_LA4 => Ok(Self::La4),
            ctru_sys::GPU_L4 => Ok(Self::L4),
            ctru_sys::GPU_A4 => Ok(Self::A4),
            ctru_sys::GPU_ETC1 => Ok(Self::Etc1),
            ctru_sys::GPU_ETC1A4 => Ok(Self::Etc1A4),
            _ => Err("invalid value for ColorFormat".to_string()),
        }
    }
}

/// Texture faces.
///
/// Faces are used for CubeMaps.
/// Standard 2D textures use only [`Face::PositiveX`], also accessible as [`Face::Bidimensional`].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXFACE")]
pub enum Face {
    /// +X face.
    ///
    /// This corresponds to the only face of 2D textures (see [`Face::Bidimensional`]).
    #[doc(alias = "GPU_POSITIVE_X")]
    PositiveX = ctru_sys::GPU_POSITIVE_X,

    /// -X face.
    #[doc(alias = "GPU_NEGATIVE_X")]
    NegativeX = ctru_sys::GPU_NEGATIVE_X,

    /// +Y face.
    #[doc(alias = "GPU_POSITIVE_Y")]
    PositiveY = ctru_sys::GPU_POSITIVE_Y,

    /// -Y face.
    #[doc(alias = "GPU_NEGATIVE_Y")]
    NegativeY = ctru_sys::GPU_NEGATIVE_Y,

    /// +Z face.
    #[doc(alias = "GPU_POSITIVE_Z")]
    PositiveZ = ctru_sys::GPU_POSITIVE_Z,

    /// -Z face.
    #[doc(alias = "GPU_NEGATIVE_Z")]
    NegativeZ = ctru_sys::GPU_NEGATIVE_Z,
}

impl Face {
    /// 2D face.
    ///
    /// Equal in value to [`Face::PositiveX`].
    #[allow(non_upper_case_globals)]
    #[doc(alias = "GPU_TEXFACE_2D")]
    pub const Bidimensional: Self = Self::PositiveX;
}

impl TryFrom<u8> for Face {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_POSITIVE_X => Ok(Self::PositiveX),
            // ctru_sys::GPU_TEXFACE_2D and ctru_sys::GPU_POSITIVE_X have the same value which causes problems with rust.
            // ctru_sys::GPU_TEXFACE_2D => Ok(Self::Bidimensional),
            ctru_sys::GPU_NEGATIVE_X => Ok(Self::NegativeX),
            ctru_sys::GPU_POSITIVE_Y => Ok(Self::PositiveY),
            ctru_sys::GPU_NEGATIVE_Y => Ok(Self::NegativeY),
            ctru_sys::GPU_POSITIVE_Z => Ok(Self::PositiveZ),
            ctru_sys::GPU_NEGATIVE_Z => Ok(Self::NegativeZ),
            _ => Err("invalid value for Face".to_string()),
        }
    }
}

/// Procedural texture clamp modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_CLAMP")]
pub enum ProceduralTextureClamp {
    /// Clamp to zero.
    #[doc(alias = "GPU_PT_CLAMP_TO_ZERO")]
    ClampToZero = ctru_sys::GPU_PT_CLAMP_TO_ZERO,

    /// Clamp to edge.
    #[doc(alias = "GPU_PT_CLAMP_TO_EDGE")]
    ClampToEdge = ctru_sys::GPU_PT_CLAMP_TO_EDGE,

    /// Symmetrical repeat.
    #[doc(alias = "GPU_PT_REPEAT")]
    Repeat = ctru_sys::GPU_PT_REPEAT,

    /// Mirrored repeat.
    #[doc(alias = "GPU_PT_MIRRORED_REPEAT")]
    MirroredRepeat = ctru_sys::GPU_PT_MIRRORED_REPEAT,

    /// Pulse.
    #[doc(alias = "GPU_PT_PULSE")]
    Pulse = ctru_sys::GPU_PT_PULSE,
}

impl TryFrom<u8> for ProceduralTextureClamp {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_PT_CLAMP_TO_ZERO => Ok(Self::ClampToZero),
            ctru_sys::GPU_PT_CLAMP_TO_EDGE => Ok(Self::ClampToEdge),
            ctru_sys::GPU_PT_REPEAT => Ok(Self::Repeat),
            ctru_sys::GPU_PT_MIRRORED_REPEAT => Ok(Self::MirroredRepeat),
            ctru_sys::GPU_PT_PULSE => Ok(Self::Pulse),
            _ => Err("invalid value for ProceduralTextureClamp".to_string()),
        }
    }
}

/// Procedural texture mapping functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_MAPFUNC")]
pub enum ProceduralTextureMappingFunction {
    /// U
    #[doc(alias = "GPU_PT_U")]
    U = ctru_sys::GPU_PT_U,

    /// U2
    #[doc(alias = "GPU_PT_U2")]
    U2 = ctru_sys::GPU_PT_U2,

    /// V
    #[doc(alias = "GPU_PT_V")]
    V = ctru_sys::GPU_PT_V,

    /// V2
    #[doc(alias = "GPU_PT_V2")]
    V2 = ctru_sys::GPU_PT_V2,

    /// U+V
    #[doc(alias = "GPU_PT_ADD")]
    Add = ctru_sys::GPU_PT_ADD,

    /// U2+V2
    #[doc(alias = "GPU_PT_ADD2")]
    Add2 = ctru_sys::GPU_PT_ADD2,

    /// sqrt(U2+V2)
    #[doc(alias = "GPU_PT_SQRT2")]
    Sqrt2 = ctru_sys::GPU_PT_SQRT2,

    /// min
    #[doc(alias = "GPU_PT_MIN")]
    Min = ctru_sys::GPU_PT_MIN,

    /// max
    #[doc(alias = "GPU_PT_MAX")]
    Max = ctru_sys::GPU_PT_MAX,

    /// rmax
    #[doc(alias = "GPU_PT_RMAX")]
    RMax = ctru_sys::GPU_PT_RMAX,
}

impl TryFrom<u8> for ProceduralTextureMappingFunction {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_PT_U => Ok(Self::U),
            ctru_sys::GPU_PT_U2 => Ok(Self::U2),
            ctru_sys::GPU_PT_V => Ok(Self::V),
            ctru_sys::GPU_PT_V2 => Ok(Self::V2),
            ctru_sys::GPU_PT_ADD => Ok(Self::Add),
            ctru_sys::GPU_PT_ADD2 => Ok(Self::Add2),
            ctru_sys::GPU_PT_SQRT2 => Ok(Self::Sqrt2),
            ctru_sys::GPU_PT_MIN => Ok(Self::Min),
            ctru_sys::GPU_PT_MAX => Ok(Self::Max),
            ctru_sys::GPU_PT_RMAX => Ok(Self::RMax),
            _ => Err("invalid value for ProceduralTextureMappingFunction".to_string()),
        }
    }
}

/// Procedural texture shift values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_SHIFT")]
pub enum ProceduralTextureShift {
    /// No shift.
    #[doc(alias = "GPU_PT_NONE")]
    None = ctru_sys::GPU_PT_NONE,

    /// Odd shift.
    #[doc(alias = "GPU_PT_ODD")]
    Odd = ctru_sys::GPU_PT_ODD,

    /// Even shift.
    #[doc(alias = "GPU_PT_EVEN")]
    Even = ctru_sys::GPU_PT_EVEN,
}

impl TryFrom<u8> for ProceduralTextureShift {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_PT_NONE => Ok(Self::None),
            ctru_sys::GPU_PT_ODD => Ok(Self::Odd),
            ctru_sys::GPU_PT_EVEN => Ok(Self::Even),
            _ => Err("invalid value for ProceduralTextureShift".to_string()),
        }
    }
}

/// Procedural texture filter values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_FILTER")]
pub enum ProceduralTextureFilter {
    /// Nearest-neighbor
    #[doc(alias = "GPU_PT_NEAREST")]
    Nearest = ctru_sys::GPU_PT_NEAREST,

    /// Linear interpolation
    #[doc(alias = "GPU_PT_LINEAR")]
    Linear = ctru_sys::GPU_PT_LINEAR,

    /// Nearest-neighbor with mipmap using nearest-neighbor
    #[doc(alias = "GPU_PT_NEAREST_MIP_NEAREST")]
    NearestMipNearest = ctru_sys::GPU_PT_NEAREST_MIP_NEAREST,

    /// Linear interpolation with mipmap using nearest-neighbor
    #[doc(alias = "GPU_PT_LINEAR_MIP_NEAREST")]
    LinearMipNearest = ctru_sys::GPU_PT_LINEAR_MIP_NEAREST,

    /// Nearest-neighbor with mipmap using linear interpolation
    #[doc(alias = "GPU_PT_NEAREST_MIP_LINEAR")]
    NearestMipLinear = ctru_sys::GPU_PT_NEAREST_MIP_LINEAR,

    /// Linear interpolation with mipmap using linear interpolation
    #[doc(alias = "GPU_PT_LINEAR_MIP_LINEAR")]
    LinearMipLinear = ctru_sys::GPU_PT_LINEAR_MIP_LINEAR,
}

impl TryFrom<u8> for ProceduralTextureFilter {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_PT_NEAREST => Ok(Self::Nearest),
            ctru_sys::GPU_PT_LINEAR => Ok(Self::Linear),
            ctru_sys::GPU_PT_NEAREST_MIP_NEAREST => Ok(Self::NearestMipNearest),
            ctru_sys::GPU_PT_LINEAR_MIP_NEAREST => Ok(Self::LinearMipNearest),
            ctru_sys::GPU_PT_NEAREST_MIP_LINEAR => Ok(Self::NearestMipLinear),
            ctru_sys::GPU_PT_LINEAR_MIP_LINEAR => Ok(Self::LinearMipLinear),
            _ => Err("invalid value for ProceduralTextureFilter".to_string()),
        }
    }
}

/// Procedural texture LUT IDs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_PROCTEX_LUTID")]
pub enum ProceduralTextureLutId {
    /// Noise table
    #[doc(alias = "GPU_LUT_NOISE")]
    Noise = ctru_sys::GPU_LUT_NOISE,

    /// RGB mapping function table
    #[doc(alias = "GPU_LUT_RGBMAP")]
    RGBMap = ctru_sys::GPU_LUT_RGBMAP,

    /// Alpha mapping function table
    #[doc(alias = "GPU_LUT_ALPHAMAP")]
    AlphaMap = ctru_sys::GPU_LUT_ALPHAMAP,

    /// Color table
    #[doc(alias = "GPU_LUT_COLOR")]
    Color = ctru_sys::GPU_LUT_COLOR,

    /// Color difference table
    #[doc(alias = "GPU_LUT_COLORDIF")]
    ColorDif = ctru_sys::GPU_LUT_COLORDIF,
}

impl TryFrom<u8> for ProceduralTextureLutId {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_LUT_NOISE => Ok(Self::Noise),
            ctru_sys::GPU_LUT_RGBMAP => Ok(Self::RGBMap),
            ctru_sys::GPU_LUT_ALPHAMAP => Ok(Self::AlphaMap),
            ctru_sys::GPU_LUT_COLOR => Ok(Self::Color),
            ctru_sys::GPU_LUT_COLORDIF => Ok(Self::ColorDif),
            _ => Err("invalid value for ProceduralTextureLutId".to_string()),
        }
    }
}

/// Texture RGB combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_RGB")]
pub enum RgbOperand {
    /// Source color.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_COLOR")]
    SrcColor = ctru_sys::GPU_TEVOP_RGB_SRC_COLOR,

    /// Source color - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR")]
    OneMinusSrcColor = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR,

    /// Source alpha.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA,

    /// Source red.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_R")]
    SrcR = ctru_sys::GPU_TEVOP_RGB_SRC_R,

    /// Source red - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_R")]
    OneMinusSrcR = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x06")]
    _0x06 = ctru_sys::GPU_TEVOP_RGB_0x06,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x07")]
    UnknownHex07 = ctru_sys::GPU_TEVOP_RGB_0x07,

    /// Source green.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_G")]
    SrcG = ctru_sys::GPU_TEVOP_RGB_SRC_G,

    /// Source green - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_G")]
    OneMinusSrcG = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0A")]
    UnknownHex0A = ctru_sys::GPU_TEVOP_RGB_0x0A,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0B")]
    UnknownHex0B = ctru_sys::GPU_TEVOP_RGB_0x0B,

    /// Source blue.
    #[doc(alias = "GPU_TEVOP_RGB_SRC_B")]
    SrcB = ctru_sys::GPU_TEVOP_RGB_SRC_B,

    /// Source blue - 1.
    #[doc(alias = "GPU_TEVOP_RGB_ONE_MINUS_SRC_B")]
    OneMinusSrcB = ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_B,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0E")]
    UnknownHex0E = ctru_sys::GPU_TEVOP_RGB_0x0E,

    /// Unknown.
    #[doc(alias = "GPU_TEVOP_RGB_0x0F")]
    UnknownHex0F = ctru_sys::GPU_TEVOP_RGB_0x0F,
}

impl TryFrom<u8> for RgbOperand {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVOP_RGB_SRC_COLOR => Ok(Self::SrcColor),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR => Ok(Self::OneMinusSrcColor),
            ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA => Ok(Self::SrcAlpha),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA => Ok(Self::OneMinusSrcAlpha),
            ctru_sys::GPU_TEVOP_RGB_SRC_R => Ok(Self::SrcR),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R => Ok(Self::OneMinusSrcR),
            ctru_sys::GPU_TEVOP_RGB_0x06 => Ok(Self::_0x06),
            ctru_sys::GPU_TEVOP_RGB_0x07 => Ok(Self::UnknownHex07),
            ctru_sys::GPU_TEVOP_RGB_SRC_G => Ok(Self::SrcG),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G => Ok(Self::OneMinusSrcG),
            ctru_sys::GPU_TEVOP_RGB_0x0A => Ok(Self::UnknownHex0A),
            ctru_sys::GPU_TEVOP_RGB_0x0B => Ok(Self::UnknownHex0B),
            ctru_sys::GPU_TEVOP_RGB_SRC_B => Ok(Self::SrcB),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_B => Ok(Self::OneMinusSrcB),
            ctru_sys::GPU_TEVOP_RGB_0x0E => Ok(Self::UnknownHex0E),
            ctru_sys::GPU_TEVOP_RGB_0x0F => Ok(Self::UnknownHex0F),
            _ => Err("invalid value for RgbOperand".to_string()),
        }
    }
}

/// Texture Alpha combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_A")]
pub enum AlphaOperand {
    /// Source alpha.
    #[doc(alias = "GPU_TEVOP_A_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_TEVOP_A_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA,

    /// Source red.
    #[doc(alias = "GPU_TEVOP_A_SRC_R")]
    SrcRed = ctru_sys::GPU_TEVOP_A_SRC_R,

    /// Source red - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_R")]
    OneMinusSrcRed = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R,

    /// Source green.
    #[doc(alias = "GPU_TEVOP_A_SRC_G")]
    SrcGreen = ctru_sys::GPU_TEVOP_A_SRC_G,

    /// Source green - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_G")]
    OneMinusSrcGreen = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G,

    /// Source blue.
    #[doc(alias = "GPU_TEVOP_A_SRC_B")]
    SrcBlue = ctru_sys::GPU_TEVOP_A_SRC_B,

    /// Source blue - 1.
    #[doc(alias = "GPU_TEVOP_A_ONE_MINUS_SRC_B")]
    OneMinusSrcBlue = ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B,
}

impl TryFrom<u8> for AlphaOperand {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVOP_A_SRC_ALPHA => Ok(Self::SrcAlpha),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA => Ok(Self::OneMinusSrcAlpha),
            ctru_sys::GPU_TEVOP_A_SRC_R => Ok(Self::SrcRed),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R => Ok(Self::OneMinusSrcRed),
            ctru_sys::GPU_TEVOP_A_SRC_G => Ok(Self::SrcGreen),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G => Ok(Self::OneMinusSrcGreen),
            ctru_sys::GPU_TEVOP_A_SRC_B => Ok(Self::SrcBlue),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B => Ok(Self::OneMinusSrcBlue),
            _ => Err("invalid value for AlphaOperand".to_string()),
        }
    }
}

/// Texture scale factors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVSCALE")]
pub enum Scale {
    /// 1x scale
    #[doc(alias = "GPU_TEVSCALE_1")]
    Original = ctru_sys::GPU_TEVSCALE_1,

    /// 2x scale
    #[doc(alias = "GPU_TEVSCALE_2")]
    Double = ctru_sys::GPU_TEVSCALE_2,

    /// 4x scale
    #[doc(alias = "GPU_TEVSCALE_4")]
    Quadruple = ctru_sys::GPU_TEVSCALE_4,
}

impl TryFrom<u8> for Scale {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVSCALE_1 => Ok(Self::Original),
            ctru_sys::GPU_TEVSCALE_2 => Ok(Self::Double),
            ctru_sys::GPU_TEVSCALE_4 => Ok(Self::Quadruple),
            _ => Err("invalid value for Scale".to_string()),
        }
    }
}
