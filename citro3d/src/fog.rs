//! Fog/Gas unit configuration.

/// Fog modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_FOGMODE")]
pub enum FogMode {
    /// Fog/Gas unit disabled.
    #[doc(alias = "GPU_NO_FOG")]
    NoFog = ctru_sys::GPU_NO_FOG,

    /// Fog/Gas unit configured in Fog mode.
    #[doc(alias = "GPU_FOG")]
    Fog = ctru_sys::GPU_FOG,

    /// Fog/Gas unit configured in Gas mode.
    #[doc(alias = "GPU_GAS")]
    Gas = ctru_sys::GPU_GAS,
}

impl TryFrom<u8> for FogMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_NO_FOG => Ok(FogMode::NoFog),
            ctru_sys::GPU_FOG => Ok(FogMode::Fog),
            ctru_sys::GPU_GAS => Ok(FogMode::Gas),
            _ => Err("invalid value for FogMode".to_string()),
        }
    }
}

/// Gas shading density source values.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASMODE")]
pub enum GasMode {
    /// Plain density.
    #[doc(alias = "GPU_PLAIN_DENSITY")]
    PlainDensity = ctru_sys::GPU_PLAIN_DENSITY,

    /// Depth density.
    #[doc(alias = "GPU_DEPTH_DENSITY")]
    DepthDensity = ctru_sys::GPU_DEPTH_DENSITY,
}

impl TryFrom<u8> for GasMode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_PLAIN_DENSITY => Ok(GasMode::PlainDensity),
            ctru_sys::GPU_DEPTH_DENSITY => Ok(GasMode::DepthDensity),
            _ => Err("invalid value for GasMode".to_string()),
        }
    }
}

/// Gas color LUT inputs.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASLUTINPUT")]
pub enum GasLutInput {
    /// Gas density used as input.
    #[doc(alias = "GPU_GAS_DENSITY")]
    Density = ctru_sys::GPU_GAS_DENSITY,

    /// Light factor used as input.
    #[doc(alias = "GPU_GAS_LIGHT_FACTOR")]
    LightFactor = ctru_sys::GPU_GAS_LIGHT_FACTOR,
}

impl TryFrom<u8> for GasLutInput {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_GAS_DENSITY => Ok(GasLutInput::Density),
            ctru_sys::GPU_GAS_LIGHT_FACTOR => Ok(GasLutInput::LightFactor),
            _ => Err("invalid value for GasLutInput".to_string()),
        }
    }
}

/// Gas depth functions.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GPU_GASDEPTHFUNC")]
pub enum GasDepthFunction {
    /// Never pass (0).
    #[doc(alias = "GPU_GAS_NEVER")]
    Never = ctru_sys::GPU_GAS_NEVER,

    /// Always pass (1).
    #[doc(alias = "GPU_GAS_ALWAYS")]
    Always = ctru_sys::GPU_GAS_ALWAYS,

    /// Pass if greater than (1-X).
    #[doc(alias = "GPU_GAS_GREATER")]
    Greater = ctru_sys::GPU_GAS_GREATER,

    /// Pass if less than (X).
    #[doc(alias = "GPU_GAS_LESS")]
    Less = ctru_sys::GPU_GAS_LESS,
}

impl TryFrom<u8> for GasDepthFunction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            ctru_sys::GPU_GAS_NEVER => Ok(GasDepthFunction::Never),
            ctru_sys::GPU_GAS_ALWAYS => Ok(GasDepthFunction::Always),
            ctru_sys::GPU_GAS_GREATER => Ok(GasDepthFunction::Greater),
            ctru_sys::GPU_GAS_LESS => Ok(GasDepthFunction::Less),
            _ => Err("invalid value for GasDepthFunction".to_string()),
        }
    }
}
