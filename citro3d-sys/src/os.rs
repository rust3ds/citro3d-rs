// TODO: move this to ctru-sys, maybe?
// would probably be auto-generated via https://github.com/rust3ds/ctru-rs/issues/123

use ctru_sys::{osSharedConfig_s, OS_SHAREDCFG_VADDR};

fn OS_SharedConfig() -> *mut osSharedConfig_s {
    OS_SHAREDCFG_VADDR as _
}

/// Gets the state of the 3D slider as a value from 0.0 to 1.0
pub unsafe fn osGet3DSliderState() -> f32 {
    (*OS_SharedConfig()).slider_3d
}
