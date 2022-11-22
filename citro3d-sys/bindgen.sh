#!/usr/bin/env bash

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/tex3ds.h" \
    --rust-target nightly \
    --use-core \
    --distrust-clang-mangling \
    --no-layout-tests \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --fit-macro-constant-types \
    --raw-line "use ctru_sys::*;" \
    --must-use-type "Result" \
    --generate "functions,types,vars" \
    --blocklist-type "u(8|16|32|64)" \
    --opaque-type "GPU_.*" \
    --opaque-type "GFX_.*" \
    --opaque-type "float24Uniform_s" \
    --allowlist-file ".*/c3d/.*[.]h" \
    --allowlist-file ".*/tex3ds[.]h" \
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
