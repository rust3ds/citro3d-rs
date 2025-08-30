//! Render effects and behaviour used by the GPU.

/// Test functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_TESTFUNC")]
pub enum TestFunction {
    /// Never pass.
    #[doc(alias = "GPU_NEVER")]
    Never = ctru_sys::GPU_NEVER,

    /// Always pass.
    #[doc(alias = "GPU_ALWAYS")]
    Always = ctru_sys::GPU_ALWAYS,

    /// Pass if equal.
    #[doc(alias = "GPU_EQUAL")]
    Equal = ctru_sys::GPU_EQUAL,

    /// Pass if not equal.
    #[doc(alias = "GPU_NOTEQUAL")]
    NotEqual = ctru_sys::GPU_NOTEQUAL,

    /// Pass if less than.
    #[doc(alias = "GPU_LESS")]
    Less = ctru_sys::GPU_LESS,

    /// Pass if less than or equal.
    #[doc(alias = "GPU_LEQUAL")]
    LessOrEqual = ctru_sys::GPU_LEQUAL,

    /// Pass if greater than.
    #[doc(alias = "GPU_GREATER")]
    Greater = ctru_sys::GPU_GREATER,

    /// Pass if greater than or equal.
    #[doc(alias = "GPU_GEQUAL")]
    GreaterOrEqual = ctru_sys::GPU_GEQUAL,
}

impl TryFrom<u8> for TestFunction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_NEVER => Ok(Self::Never),
            ctru_sys::GPU_ALWAYS => Ok(Self::Always),
            ctru_sys::GPU_EQUAL => Ok(Self::Equal),
            ctru_sys::GPU_NOTEQUAL => Ok(Self::NotEqual),
            ctru_sys::GPU_LESS => Ok(Self::Less),
            ctru_sys::GPU_LEQUAL => Ok(Self::LessOrEqual),
            ctru_sys::GPU_GREATER => Ok(Self::Greater),
            ctru_sys::GPU_GEQUAL => Ok(Self::GreaterOrEqual),
            _ => Err("invalid value for TestFunction".to_string()),
        }
    }
}
/// Early depth test functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_EARLYDEPTHFUNC")]
pub enum EarlyDepthFunction {
    /// Pass if greater than or equal.
    #[doc(alias = "GPU_EARLYDEPTH_GEQUAL")]
    GreaterOrEqual = ctru_sys::GPU_EARLYDEPTH_GEQUAL,

    /// Pass if greater than.
    #[doc(alias = "GPU_EARLYDEPTH_GREATER")]
    Greater = ctru_sys::GPU_EARLYDEPTH_GREATER,

    /// Pass if less than or equal.
    #[doc(alias = "GPU_EARLYDEPTH_LEQUAL")]
    LessOrEqual = ctru_sys::GPU_EARLYDEPTH_LEQUAL,

    /// Pass if less than.
    #[doc(alias = "GPU_EARLYDEPTH_LESS")]
    Less = ctru_sys::GPU_EARLYDEPTH_LESS,
}

impl TryFrom<u8> for EarlyDepthFunction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_EARLYDEPTH_GEQUAL => Ok(Self::GreaterOrEqual),
            ctru_sys::GPU_EARLYDEPTH_GREATER => Ok(Self::Greater),
            ctru_sys::GPU_EARLYDEPTH_LEQUAL => Ok(Self::LessOrEqual),
            ctru_sys::GPU_EARLYDEPTH_LESS => Ok(Self::Less),
            _ => Err("invalid value for EarlyDepthFunction".to_string()),
        }
    }
}

/// Scissor test modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_SCISSORMODE")]
pub enum ScissorMode {
    /// Disable.
    #[doc(alias = "GPU_SCISSOR_DISABLE")]
    Disable = ctru_sys::GPU_SCISSOR_DISABLE,

    /// Exclude pixels inside the scissor box.
    #[doc(alias = "GPU_SCISSOR_INVERT")]
    Invert = ctru_sys::GPU_SCISSOR_INVERT,

    /// Exclude pixels outside of the scissor box.
    #[doc(alias = "GPU_SCISSOR_NORMAL")]
    Normal = ctru_sys::GPU_SCISSOR_NORMAL,
}

impl TryFrom<u8> for ScissorMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_SCISSOR_DISABLE => Ok(Self::Disable),
            ctru_sys::GPU_SCISSOR_INVERT => Ok(Self::Invert),
            ctru_sys::GPU_SCISSOR_NORMAL => Ok(Self::Normal),
            _ => Err("invalid value for ScissorMode".to_string()),
        }
    }
}

/// Stencil operations.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_STENCILOP")]
pub enum StencilOperation {
    /// Keep old value. (old_stencil)
    #[doc(alias = "GPU_STENCIL_KEEP")]
    Keep = ctru_sys::GPU_STENCIL_KEEP,

    /// Zero. (0)
    #[doc(alias = "GPU_STENCIL_ZERO")]
    Zero = ctru_sys::GPU_STENCIL_ZERO,

    /// Replace value. (ref)
    #[doc(alias = "GPU_STENCIL_REPLACE")]
    Replace = ctru_sys::GPU_STENCIL_REPLACE,

    /// Increment value. (old_stencil + 1 saturated to [0, 255])
    #[doc(alias = "GPU_STENCIL_INCR")]
    Increment = ctru_sys::GPU_STENCIL_INCR,

    /// Decrement value. (old_stencil - 1 saturated to [0, 255])
    #[doc(alias = "GPU_STENCIL_DECR")]
    Decrement = ctru_sys::GPU_STENCIL_DECR,

    /// Invert value. (~old_stencil)
    #[doc(alias = "GPU_STENCIL_INVERT")]
    Invert = ctru_sys::GPU_STENCIL_INVERT,

    /// Increment value. (old_stencil + 1)
    #[doc(alias = "GPU_STENCIL_INCR_WRAP")]
    IncrementWrap = ctru_sys::GPU_STENCIL_INCR_WRAP,

    /// Decrement value. (old_stencil - 1)
    #[doc(alias = "GPU_STENCIL_DECR_WRAP")]
    DecrementWrap = ctru_sys::GPU_STENCIL_DECR_WRAP,
}

impl TryFrom<u8> for StencilOperation {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_STENCIL_KEEP => Ok(Self::Keep),
            ctru_sys::GPU_STENCIL_ZERO => Ok(Self::Zero),
            ctru_sys::GPU_STENCIL_REPLACE => Ok(Self::Replace),
            ctru_sys::GPU_STENCIL_INCR => Ok(Self::Increment),
            ctru_sys::GPU_STENCIL_DECR => Ok(Self::Decrement),
            ctru_sys::GPU_STENCIL_INVERT => Ok(Self::Invert),
            ctru_sys::GPU_STENCIL_INCR_WRAP => Ok(Self::IncrementWrap),
            ctru_sys::GPU_STENCIL_DECR_WRAP => Ok(Self::DecrementWrap),
            _ => Err("invalid value for StencilOperation".to_string()),
        }
    }
}

/// Pixel write mask.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_WRITEMASK")]
pub enum WriteMask {
    /// Write red.
    #[doc(alias = "GPU_WRITE_RED")]
    Red = ctru_sys::GPU_WRITE_RED,

    /// Write green.
    #[doc(alias = "GPU_WRITE_GREEN")]
    Green = ctru_sys::GPU_WRITE_GREEN,

    /// Write blue.
    #[doc(alias = "GPU_WRITE_BLUE")]
    Blue = ctru_sys::GPU_WRITE_BLUE,

    /// Write alpha.
    #[doc(alias = "GPU_WRITE_ALPHA")]
    Alpha = ctru_sys::GPU_WRITE_ALPHA,

    /// Write depth.
    #[doc(alias = "GPU_WRITE_DEPTH")]
    Depth = ctru_sys::GPU_WRITE_DEPTH,

    /// Write all color components.
    #[doc(alias = "GPU_WRITE_COLOR")]
    Color = ctru_sys::GPU_WRITE_COLOR,

    /// Write all components.
    #[doc(alias = "GPU_WRITE_ALL")]
    All = ctru_sys::GPU_WRITE_ALL,
}

impl TryFrom<u8> for WriteMask {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_WRITE_RED => Ok(Self::Red),
            ctru_sys::GPU_WRITE_GREEN => Ok(Self::Green),
            ctru_sys::GPU_WRITE_BLUE => Ok(Self::Blue),
            ctru_sys::GPU_WRITE_ALPHA => Ok(Self::Alpha),
            ctru_sys::GPU_WRITE_DEPTH => Ok(Self::Depth),
            ctru_sys::GPU_WRITE_COLOR => Ok(Self::Color),
            ctru_sys::GPU_WRITE_ALL => Ok(Self::All),
            _ => Err("invalid value for WriteMask".to_string()),
        }
    }
}

/// Blend modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_BLENDEQUATION")]
pub enum BlendEquation {
    /// Add colors.
    #[doc(alias = "GPU_BLEND_ADD")]
    Add = ctru_sys::GPU_BLEND_ADD,

    /// Subtract colors.
    #[doc(alias = "GPU_BLEND_SUBTRACT")]
    Subtract = ctru_sys::GPU_BLEND_SUBTRACT,

    /// Reverse-subtract colors.
    #[doc(alias = "GPU_BLEND_REVERSE_SUBTRACT")]
    ReverseSubtract = ctru_sys::GPU_BLEND_REVERSE_SUBTRACT,

    /// Use the minimum color.
    #[doc(alias = "GPU_BLEND_MIN")]
    Min = ctru_sys::GPU_BLEND_MIN,

    /// Use the maximum color.
    #[doc(alias = "GPU_BLEND_MAX")]
    Max = ctru_sys::GPU_BLEND_MAX,
}

impl TryFrom<u8> for BlendEquation {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_BLEND_ADD => Ok(Self::Add),
            ctru_sys::GPU_BLEND_SUBTRACT => Ok(Self::Subtract),
            ctru_sys::GPU_BLEND_REVERSE_SUBTRACT => Ok(Self::ReverseSubtract),
            ctru_sys::GPU_BLEND_MIN => Ok(Self::Min),
            ctru_sys::GPU_BLEND_MAX => Ok(Self::Max),
            _ => Err("invalid value for BlendEquation".to_string()),
        }
    }
}

/// Blend factors.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_BLENDFACTOR")]
pub enum BlendFactor {
    /// Zero.
    #[doc(alias = "GPU_ZERO")]
    Zero = ctru_sys::GPU_ZERO,

    /// One.
    #[doc(alias = "GPU_ONE")]
    One = ctru_sys::GPU_ONE,

    /// Source color.
    #[doc(alias = "GPU_SRC_COLOR")]
    SrcColor = ctru_sys::GPU_SRC_COLOR,

    /// Source color - 1.
    #[doc(alias = "GPU_ONE_MINUS_SRC_COLOR")]
    OneMinusSrcColor = ctru_sys::GPU_ONE_MINUS_SRC_COLOR,

    /// Destination color.
    #[doc(alias = "GPU_DST_COLOR")]
    DstColor = ctru_sys::GPU_DST_COLOR,

    /// Destination color - 1.
    #[doc(alias = "GPU_ONE_MINUS_DST_COLOR")]
    OneMinusDstColor = ctru_sys::GPU_ONE_MINUS_DST_COLOR,

    /// Source alpha.
    #[doc(alias = "GPU_SRC_ALPHA")]
    SrcAlpha = ctru_sys::GPU_SRC_ALPHA,

    /// Source alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_SRC_ALPHA")]
    OneMinusSrcAlpha = ctru_sys::GPU_ONE_MINUS_SRC_ALPHA,

    /// Destination alpha.
    #[doc(alias = "GPU_DST_ALPHA")]
    DstAlpha = ctru_sys::GPU_DST_ALPHA,

    /// Destination alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_DST_ALPHA")]
    OneMinusDstAlpha = ctru_sys::GPU_ONE_MINUS_DST_ALPHA,

    /// Constant color.
    #[doc(alias = "GPU_CONSTANT_COLOR")]
    ConstantColor = ctru_sys::GPU_CONSTANT_COLOR,

    /// Constant color - 1.
    #[doc(alias = "GPU_ONE_MINUS_CONSTANT_COLOR")]
    OneMinusConstantColor = ctru_sys::GPU_ONE_MINUS_CONSTANT_COLOR,

    /// Constant alpha.
    #[doc(alias = "GPU_CONSTANT_ALPHA")]
    ConstantAlpha = ctru_sys::GPU_CONSTANT_ALPHA,

    /// Constant alpha - 1.
    #[doc(alias = "GPU_ONE_MINUS_CONSTANT_ALPHA")]
    OneMinusConstantAlpha = ctru_sys::GPU_ONE_MINUS_CONSTANT_ALPHA,

    /// Saturated alpha.
    #[doc(alias = "GPU_SRC_ALPHA_SATURATE")]
    SrcAlphaSaturate = ctru_sys::GPU_SRC_ALPHA_SATURATE,
}

impl TryFrom<u8> for BlendFactor {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_ZERO => Ok(Self::Zero),
            ctru_sys::GPU_ONE => Ok(Self::One),
            ctru_sys::GPU_SRC_COLOR => Ok(Self::SrcColor),
            ctru_sys::GPU_ONE_MINUS_SRC_COLOR => Ok(Self::OneMinusSrcColor),
            ctru_sys::GPU_DST_COLOR => Ok(Self::DstColor),
            ctru_sys::GPU_ONE_MINUS_DST_COLOR => Ok(Self::OneMinusDstColor),
            ctru_sys::GPU_SRC_ALPHA => Ok(Self::SrcAlpha),
            ctru_sys::GPU_ONE_MINUS_SRC_ALPHA => Ok(Self::OneMinusSrcAlpha),
            ctru_sys::GPU_DST_ALPHA => Ok(Self::DstAlpha),
            ctru_sys::GPU_ONE_MINUS_DST_ALPHA => Ok(Self::OneMinusDstAlpha),
            ctru_sys::GPU_CONSTANT_COLOR => Ok(Self::ConstantColor),
            ctru_sys::GPU_ONE_MINUS_CONSTANT_COLOR => Ok(Self::OneMinusConstantColor),
            ctru_sys::GPU_CONSTANT_ALPHA => Ok(Self::ConstantAlpha),
            ctru_sys::GPU_ONE_MINUS_CONSTANT_ALPHA => Ok(Self::OneMinusConstantAlpha),
            ctru_sys::GPU_SRC_ALPHA_SATURATE => Ok(Self::SrcAlphaSaturate),
            _ => Err("invalid value for BlendFactor".to_string()),
        }
    }
}

/// Logical operations.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_LOGICOP")]
pub enum LogicOperation {
    /// Clear.
    #[doc(alias = "GPU_LOGICOP_CLEAR")]
    Clear = ctru_sys::GPU_LOGICOP_CLEAR,

    /// Bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND")]
    And = ctru_sys::GPU_LOGICOP_AND,

    /// Reverse bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND_REVERSE")]
    AndReverse = ctru_sys::GPU_LOGICOP_AND_REVERSE,

    /// Copy.
    #[doc(alias = "GPU_LOGICOP_COPY")]
    Copy = ctru_sys::GPU_LOGICOP_COPY,

    /// Set.
    #[doc(alias = "GPU_LOGICOP_SET")]
    Set = ctru_sys::GPU_LOGICOP_SET,

    /// Inverted copy.
    #[doc(alias = "GPU_LOGICOP_COPY_INVERTED")]
    CopyInverted = ctru_sys::GPU_LOGICOP_COPY_INVERTED,

    /// No operation.
    #[doc(alias = "GPU_LOGICOP_NOOP")]
    Noop = ctru_sys::GPU_LOGICOP_NOOP,

    /// Invert.
    #[doc(alias = "GPU_LOGICOP_INVERT")]
    Invert = ctru_sys::GPU_LOGICOP_INVERT,

    /// Bitwise NAND.
    #[doc(alias = "GPU_LOGICOP_NAND")]
    Nand = ctru_sys::GPU_LOGICOP_NAND,

    /// Bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR")]
    Or = ctru_sys::GPU_LOGICOP_OR,

    /// Bitwise NOR.
    #[doc(alias = "GPU_LOGICOP_NOR")]
    Nor = ctru_sys::GPU_LOGICOP_NOR,

    /// Bitwise XOR.
    #[doc(alias = "GPU_LOGICOP_XOR")]
    Xor = ctru_sys::GPU_LOGICOP_XOR,

    /// Equivalent.
    #[doc(alias = "GPU_LOGICOP_EQUIV")]
    Equiv = ctru_sys::GPU_LOGICOP_EQUIV,

    /// Inverted bitwise AND.
    #[doc(alias = "GPU_LOGICOP_AND_INVERTED")]
    AndInverted = ctru_sys::GPU_LOGICOP_AND_INVERTED,

    /// Reverse bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR_REVERSE")]
    OrReverse = ctru_sys::GPU_LOGICOP_OR_REVERSE,

    /// Inverted bitwise OR.
    #[doc(alias = "GPU_LOGICOP_OR_INVERTED")]
    OrInverted = ctru_sys::GPU_LOGICOP_OR_INVERTED,
}

impl TryFrom<u8> for LogicOperation {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_LOGICOP_CLEAR => Ok(Self::Clear),
            ctru_sys::GPU_LOGICOP_AND => Ok(Self::And),
            ctru_sys::GPU_LOGICOP_AND_REVERSE => Ok(Self::AndReverse),
            ctru_sys::GPU_LOGICOP_COPY => Ok(Self::Copy),
            ctru_sys::GPU_LOGICOP_SET => Ok(Self::Set),
            ctru_sys::GPU_LOGICOP_COPY_INVERTED => Ok(Self::CopyInverted),
            ctru_sys::GPU_LOGICOP_NOOP => Ok(Self::Noop),
            ctru_sys::GPU_LOGICOP_INVERT => Ok(Self::Invert),
            ctru_sys::GPU_LOGICOP_NAND => Ok(Self::Nand),
            ctru_sys::GPU_LOGICOP_OR => Ok(Self::Or),
            ctru_sys::GPU_LOGICOP_NOR => Ok(Self::Nor),
            ctru_sys::GPU_LOGICOP_XOR => Ok(Self::Xor),
            ctru_sys::GPU_LOGICOP_EQUIV => Ok(Self::Equiv),
            ctru_sys::GPU_LOGICOP_AND_INVERTED => Ok(Self::AndInverted),
            ctru_sys::GPU_LOGICOP_OR_REVERSE => Ok(Self::OrReverse),
            ctru_sys::GPU_LOGICOP_OR_INVERTED => Ok(Self::OrInverted),
            _ => Err("invalid value for LogicOperation".to_string()),
        }
    }
}

/// Cull modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_CULLMODE")]
pub enum CullMode {
    /// Disabled.
    #[doc(alias = "GPU_CULL_NONE")]
    None = ctru_sys::GPU_CULL_NONE,

    /// Front, counter-clockwise.
    #[doc(alias = "GPU_CULL_FRONT_CCW")]
    FrontCounterClockwise = ctru_sys::GPU_CULL_FRONT_CCW,

    /// Back, counter-clockwise.
    #[doc(alias = "GPU_CULL_BACK_CCW")]
    BackCounterClockwise = ctru_sys::GPU_CULL_BACK_CCW,
}

impl TryFrom<u8> for CullMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_CULL_NONE => Ok(Self::None),
            ctru_sys::GPU_CULL_FRONT_CCW => Ok(Self::FrontCounterClockwise),
            ctru_sys::GPU_CULL_BACK_CCW => Ok(Self::BackCounterClockwise),
            _ => Err("invalid value for CullMode".to_string()),
        }
    }
}

/// Fragment operation modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_FRAGOPMODE")]
pub enum FragmentOperationMode {
    /// OpenGL mode.
    #[doc(alias = "GPU_FRAGOPMODE_GL")]
    Gl = ctru_sys::GPU_FRAGOPMODE_GL,

    /// Gas mode (?).
    #[doc(alias = "GPU_FRAGOPMODE_GAS_ACC")]
    GasAcc = ctru_sys::GPU_FRAGOPMODE_GAS_ACC,

    /// Shadow mode (?).
    #[doc(alias = "GPU_FRAGOPMODE_SHADOW")]
    Shadow = ctru_sys::GPU_FRAGOPMODE_SHADOW,
}

impl TryFrom<u8> for FragmentOperationMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_FRAGOPMODE_GL => Ok(Self::Gl),
            ctru_sys::GPU_FRAGOPMODE_GAS_ACC => Ok(Self::GasAcc),
            ctru_sys::GPU_FRAGOPMODE_SHADOW => Ok(Self::Shadow),
            _ => Err("invalid value for FragmentOperationMode".to_string()),
        }
    }
}
