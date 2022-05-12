#!/usr/bin/env bash

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/citro3d.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --blocklist-type "__builtin_va_list" \
    --blocklist-type "__va_list" \
    --opaque-type "GPU_.*" \
    --opaque-type "GFX_.*" \
    --opaque-type "gfx.*_t" \
    --opaque-type "DVL.*" \
    --opaque-type "shader.*" \
    --opaque-type "float24Uniform_s" \
    --allowlist-type "C3D_.*" \
    --allowlist-function "C3D_.*" \
    --allowlist-var "C3D_.*" \
    --allowlist-function 'AttrInfo_(Init|AddLoader|AddFixed)' \
    --allowlist-function 'BufInfo_(Init|Add)' \
    --allowlist-function 'Mtx_.*' \
    --raw-line "use ctru_sys::*;" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D_3DS \
    -D__3DS__ \
> src/bindings.rs
