#!/usr/bin/env bash

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/citro3d.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --fit-macro-constant-types \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --opaque-type "GPU_.*" \
    --opaque-type "GFX_.*" \
    --opaque-type "float24Uniform_s" \
    --allowlist-type "C3D_.*" \
    --allowlist-type "DVLB_.*" \
    --allowlist-type "shader.*" \
    --allowlist-type "float24Uniform_s" \
    --allowlist-function "C3D_.*" \
    --allowlist-function "shader.*" \
    --allowlist-function "DVLB_.*" \
    --allowlist-function "linear.*" \
    --allowlist-var "C3D_.*" \
    --allowlist-type "GPU_.*" \
    --allowlist-type "GX_.*" \
    --allowlist-function 'AttrInfo_(Init|AddLoader|AddFixed)' \
    --allowlist-function 'BufInfo_(Init|Add)' \
    --allowlist-function 'Mtx_.*' \
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
