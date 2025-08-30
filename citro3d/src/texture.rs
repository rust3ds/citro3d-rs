use ctru_sys;

/// Texture filters.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_FILTER_PARAM")]
pub enum TextureFilterParam {
    #[doc(alias = "GPU_NEAREST")]
    Nearest = ctru_sys::GPU_NEAREST,
    #[doc(alias = "GPU_LINEAR")]
    Linear = ctru_sys::GPU_LINEAR,
}

impl TryFrom<u8> for TextureFilterParam {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_NEAREST => Ok(TextureFilterParam::Nearest),
            ctru_sys::GPU_LINEAR => Ok(TextureFilterParam::Linear),
            _ => Err("Invalid value for TextureFilterParam".to_string()),
        }
    }
}

/// Texture wrap modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_WRAP_PARAM")]
pub enum TextureWrapParam {
    #[doc(alias = "GPU_CLAMP_TO_EDGE")]
    ClampToEdge = ctru_sys::GPU_CLAMP_TO_EDGE,
    #[doc(alias = "GPU_CLAMP_TO_BORDER")]
    ClampToBorder = ctru_sys::GPU_CLAMP_TO_BORDER,
    #[doc(alias = "GPU_REPEAT")]
    Repeat = ctru_sys::GPU_REPEAT,
    #[doc(alias = "GPU_MIRRORED_REPEAT")]
    MirroredRepeat = ctru_sys::GPU_MIRRORED_REPEAT,
}

impl TryFrom<u8> for TextureWrapParam {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_CLAMP_TO_EDGE => Ok(TextureWrapParam::ClampToEdge),
            ctru_sys::GPU_CLAMP_TO_BORDER => Ok(TextureWrapParam::ClampToBorder),
            ctru_sys::GPU_REPEAT => Ok(TextureWrapParam::Repeat),
            ctru_sys::GPU_MIRRORED_REPEAT => Ok(TextureWrapParam::MirroredRepeat),
            _ => Err("Invalid value for TextureWrapParam".to_string()),
        }
    }
}

/// Texture modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXTURE_MODE_PARAM")]
pub enum TextureModeParam {
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

impl TryFrom<u8> for TextureModeParam {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEX_2D => Ok(TextureModeParam::Tex2D),
            ctru_sys::GPU_TEX_CUBE_MAP => Ok(TextureModeParam::CubeMap),
            ctru_sys::GPU_TEX_SHADOW_2D => Ok(TextureModeParam::Shadow2D),
            ctru_sys::GPU_TEX_PROJECTION => Ok(TextureModeParam::Projection),
            ctru_sys::GPU_TEX_SHADOW_CUBE => Ok(TextureModeParam::ShadowCube),
            ctru_sys::GPU_TEX_DISABLED => Ok(TextureModeParam::Disabled),
            _ => Err("Invalid value for TextureModeParam".to_string()),
        }
    }
}

/// Supported texture units.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXUNIT")]
pub enum TexureUnit {
    #[doc(alias = "GPU_TEXUNIT0")]
    Unit0 = ctru_sys::GPU_TEXUNIT0,
    #[doc(alias = "GPU_TEXUNIT1")]
    Unit1 = ctru_sys::GPU_TEXUNIT1,
    #[doc(alias = "GPU_TEXUNIT2")]
    Unit2 = ctru_sys::GPU_TEXUNIT2,
}

impl TryFrom<u8> for TexureUnit {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEXUNIT0 => Ok(TexureUnit::Unit0),
            ctru_sys::GPU_TEXUNIT1 => Ok(TexureUnit::Unit1),
            ctru_sys::GPU_TEXUNIT2 => Ok(TexureUnit::Unit2),
            _ => Err("Invalid value for TexureUnit".to_string()),
        }
    }
}

/// Supported texture formats.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXCOLOR")]
pub enum TextureColorFormat {
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

impl TryFrom<u8> for TextureColorFormat {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_RGBA8 => Ok(TextureColorFormat::Rgba8),
            ctru_sys::GPU_RGB8 => Ok(TextureColorFormat::Rgb8),
            ctru_sys::GPU_RGBA5551 => Ok(TextureColorFormat::Rgba5551),
            ctru_sys::GPU_RGB565 => Ok(TextureColorFormat::Rgb565),
            ctru_sys::GPU_RGBA4 => Ok(TextureColorFormat::Rgba4),
            ctru_sys::GPU_LA8 => Ok(TextureColorFormat::La8),
            ctru_sys::GPU_HILO8 => Ok(TextureColorFormat::Hilo8),
            ctru_sys::GPU_L8 => Ok(TextureColorFormat::L8),
            ctru_sys::GPU_A8 => Ok(TextureColorFormat::A8),
            ctru_sys::GPU_LA4 => Ok(TextureColorFormat::La4),
            ctru_sys::GPU_L4 => Ok(TextureColorFormat::L4),
            ctru_sys::GPU_A4 => Ok(TextureColorFormat::A4),
            ctru_sys::GPU_ETC1 => Ok(TextureColorFormat::Etc1),
            ctru_sys::GPU_ETC1A4 => Ok(TextureColorFormat::Etc1A4),
            _ => Err("Invalid value for TextureColorFormat".to_string()),
        }
    }
}

/// Texture faces.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEXFACE")]
pub enum TextureFace {
    /// 2D face
    #[doc(alias = "GPU_TEXFACE_2D")]
    TwoD = ctru_sys::GPU_TEXFACE_2D,

    /// +X face
    /// FIXME: ctru_sys::GPU_TEXFACE_2D and ctru_sys::GPU_POSITIVE_X have the same value which causes problems with rust.
    // #[doc(alias = "GPU_POSITIVE_X")]
    // PositiveX = ctru_sys::GPU_POSITIVE_X,

    /// -X face
    #[doc(alias = "GPU_NEGATIVE_X")]
    NegativeX = ctru_sys::GPU_NEGATIVE_X,
    /// +Y face
    #[doc(alias = "GPU_POSITIVE_Y")]
    PositiveY = ctru_sys::GPU_POSITIVE_Y,
    /// -Y face
    #[doc(alias = "GPU_NEGATIVE_Y")]
    NegativeY = ctru_sys::GPU_NEGATIVE_Y,
    /// +Z face
    #[doc(alias = "GPU_POSITIVE_Z")]
    PositiveZ = ctru_sys::GPU_POSITIVE_Z,
    /// -Z face
    #[doc(alias = "GPU_NEGATIVE_Z")]
    NegativeZ = ctru_sys::GPU_NEGATIVE_Z,
}

impl TryFrom<u8> for TextureFace {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEXFACE_2D => Ok(TextureFace::TwoD),
            /// FIXME: ctru_sys::GPU_TEXFACE_2D and ctru_sys::GPU_POSITIVE_X have the same value which causes problems with rust.
            // ctru_sys::GPU_POSITIVE_X => Ok(TextureFace::PositiveX),
            ctru_sys::GPU_NEGATIVE_X => Ok(TextureFace::NegativeX),
            ctru_sys::GPU_POSITIVE_Y => Ok(TextureFace::PositiveY),
            ctru_sys::GPU_NEGATIVE_Y => Ok(TextureFace::NegativeY),
            ctru_sys::GPU_POSITIVE_Z => Ok(TextureFace::PositiveZ),
            ctru_sys::GPU_NEGATIVE_Z => Ok(TextureFace::NegativeZ),
            _ => Err("Invalid value for TextureFace".to_string()),
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
            ctru_sys::GPU_PT_CLAMP_TO_ZERO => Ok(ProceduralTextureClamp::ClampToZero),
            ctru_sys::GPU_PT_CLAMP_TO_EDGE => Ok(ProceduralTextureClamp::ClampToEdge),
            ctru_sys::GPU_PT_REPEAT => Ok(ProceduralTextureClamp::Repeat),
            ctru_sys::GPU_PT_MIRRORED_REPEAT => Ok(ProceduralTextureClamp::MirroredRepeat),
            ctru_sys::GPU_PT_PULSE => Ok(ProceduralTextureClamp::Pulse),
            _ => Err("Invalid value for ProceduralTextureClamp".to_string()),
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
            ctru_sys::GPU_PT_U => Ok(ProceduralTextureMappingFunction::U),
            ctru_sys::GPU_PT_U2 => Ok(ProceduralTextureMappingFunction::U2),
            ctru_sys::GPU_PT_V => Ok(ProceduralTextureMappingFunction::V),
            ctru_sys::GPU_PT_V2 => Ok(ProceduralTextureMappingFunction::V2),
            ctru_sys::GPU_PT_ADD => Ok(ProceduralTextureMappingFunction::Add),
            ctru_sys::GPU_PT_ADD2 => Ok(ProceduralTextureMappingFunction::Add2),
            ctru_sys::GPU_PT_SQRT2 => Ok(ProceduralTextureMappingFunction::Sqrt2),
            ctru_sys::GPU_PT_MIN => Ok(ProceduralTextureMappingFunction::Min),
            ctru_sys::GPU_PT_MAX => Ok(ProceduralTextureMappingFunction::Max),
            ctru_sys::GPU_PT_RMAX => Ok(ProceduralTextureMappingFunction::RMax),
            _ => Err("Invalid value for ProceduralTextureMappingFunction".to_string()),
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
            ctru_sys::GPU_PT_NONE => Ok(ProceduralTextureShift::None),
            ctru_sys::GPU_PT_ODD => Ok(ProceduralTextureShift::Odd),
            ctru_sys::GPU_PT_EVEN => Ok(ProceduralTextureShift::Even),
            _ => Err("Invalid value for ProceduralTextureShift".to_string()),
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
            ctru_sys::GPU_PT_NEAREST => Ok(ProceduralTextureFilter::Nearest),
            ctru_sys::GPU_PT_LINEAR => Ok(ProceduralTextureFilter::Linear),
            ctru_sys::GPU_PT_NEAREST_MIP_NEAREST => Ok(ProceduralTextureFilter::NearestMipNearest),
            ctru_sys::GPU_PT_LINEAR_MIP_NEAREST => Ok(ProceduralTextureFilter::LinearMipNearest),
            ctru_sys::GPU_PT_NEAREST_MIP_LINEAR => Ok(ProceduralTextureFilter::NearestMipLinear),
            ctru_sys::GPU_PT_LINEAR_MIP_LINEAR => Ok(ProceduralTextureFilter::LinearMipLinear),
            _ => Err("Invalid value for ProceduralTextureFilter".to_string()),
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
            ctru_sys::GPU_LUT_NOISE => Ok(ProceduralTextureLutId::Noise),
            ctru_sys::GPU_LUT_RGBMAP => Ok(ProceduralTextureLutId::RGBMap),
            ctru_sys::GPU_LUT_ALPHAMAP => Ok(ProceduralTextureLutId::AlphaMap),
            ctru_sys::GPU_LUT_COLOR => Ok(ProceduralTextureLutId::Color),
            ctru_sys::GPU_LUT_COLORDIF => Ok(ProceduralTextureLutId::ColorDif),
            _ => Err("Invalid value for ProceduralTextureLutId".to_string()),
        }
    }
}

/// Texture RGB combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_RGB")]
pub enum TextureRgbOperand {
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

impl TryFrom<u8> for TextureRgbOperand {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVOP_RGB_SRC_COLOR => Ok(TextureRgbOperand::SrcColor),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_COLOR => Ok(TextureRgbOperand::OneMinusSrcColor),
            ctru_sys::GPU_TEVOP_RGB_SRC_ALPHA => Ok(TextureRgbOperand::SrcAlpha),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_ALPHA => Ok(TextureRgbOperand::OneMinusSrcAlpha),
            ctru_sys::GPU_TEVOP_RGB_SRC_R => Ok(TextureRgbOperand::SrcR),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_R => Ok(TextureRgbOperand::OneMinusSrcR),
            ctru_sys::GPU_TEVOP_RGB_0x06 => Ok(TextureRgbOperand::_0x06),
            ctru_sys::GPU_TEVOP_RGB_0x07 => Ok(TextureRgbOperand::UnknownHex07),
            ctru_sys::GPU_TEVOP_RGB_SRC_G => Ok(TextureRgbOperand::SrcG),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_G => Ok(TextureRgbOperand::OneMinusSrcG),
            ctru_sys::GPU_TEVOP_RGB_0x0A => Ok(TextureRgbOperand::UnknownHex0A),
            ctru_sys::GPU_TEVOP_RGB_0x0B => Ok(TextureRgbOperand::UnknownHex0B),
            ctru_sys::GPU_TEVOP_RGB_SRC_B => Ok(TextureRgbOperand::SrcB),
            ctru_sys::GPU_TEVOP_RGB_ONE_MINUS_SRC_B => Ok(TextureRgbOperand::OneMinusSrcB),
            ctru_sys::GPU_TEVOP_RGB_0x0E => Ok(TextureRgbOperand::UnknownHex0E),
            ctru_sys::GPU_TEVOP_RGB_0x0F => Ok(TextureRgbOperand::UnknownHex0F),
            _ => Err("Invalid value for TextureRgbOperand".to_string()),
        }
    }
}

/// Texture Alpha combiner operands.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVOP_A")]
pub enum TextureAlphaOperand {
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

impl TryFrom<u8> for TextureAlphaOperand {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVOP_A_SRC_ALPHA => Ok(TextureAlphaOperand::SrcAlpha),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_ALPHA => Ok(TextureAlphaOperand::OneMinusSrcAlpha),
            ctru_sys::GPU_TEVOP_A_SRC_R => Ok(TextureAlphaOperand::SrcRed),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_R => Ok(TextureAlphaOperand::OneMinusSrcRed),
            ctru_sys::GPU_TEVOP_A_SRC_G => Ok(TextureAlphaOperand::SrcGreen),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_G => Ok(TextureAlphaOperand::OneMinusSrcGreen),
            ctru_sys::GPU_TEVOP_A_SRC_B => Ok(TextureAlphaOperand::SrcBlue),
            ctru_sys::GPU_TEVOP_A_ONE_MINUS_SRC_B => Ok(TextureAlphaOperand::OneMinusSrcBlue),
            _ => Err("Invalid value for TextureAlphaOperand".to_string()),
        }
    }
}

/// Texture scale factors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TEVSCALE")]
pub enum TextureScale {
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

impl TryFrom<u8> for TextureScale {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_TEVSCALE_1 => Ok(TextureScale::Original),
            ctru_sys::GPU_TEVSCALE_2 => Ok(TextureScale::Double),
            ctru_sys::GPU_TEVSCALE_4 => Ok(TextureScale::Quadruple),
            _ => Err("Invalid value for TextureScale".to_string()),
        }
    }
}
