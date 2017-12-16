#!/usr/bin/env bash

clang_version=$1

if [ -z "$clang_version" ]; then
    echo "  usage: ./bindgen.sh <clang_version>"
    echo "example: ./bindgen.sh 5.0.0"
    echo "Check your current version with \`clang -v\`."
    exit 1
fi

set -euxo pipefail

bindgen "$DEVKITPRO/libctru/include/citro3d.h" \
    --rust-target 1.21 \
    --use-core \
    --distrust-clang-mangling \
    --no-doc-comments \
    --no-derive-debug \
    --no-layout-tests \
    --no-rustfmt-bindings \
    --ctypes-prefix "::libc" \
    --no-prepend-enum-name \
    --generate "functions,types,vars" \
    --blacklist-type "u(8|16|32|64)" \
    --blacklist-type "__builtin_va_list" \
    --blacklist-type "__va_list" \
    --no-recursive-whitelist \
    --whitelist-type "C3D_.*" \
    --whitelist-function "C3D_.*" \
    --whitelist-var "C3D_.*" \
    --whitelist-function 'AttrInfo_(Init|AddLoader|AddFixed)' \
    --whitelist-function 'BufInfo_(Init|Add)' \
    --whitelist-function 'Mtx_.*' \
    --raw-line "use libctru::*;" \
    -- \
    --target=arm-none-eabi \
    --sysroot=$DEVKITARM/arm-none-eabi \
    -isystem$DEVKITARM/arm-none-eabi/include \
    -isystem/usr/lib/clang/$clang_version/include \
    -I$DEVKITPRO/libctru/include \
    -mfloat-abi=hard \
    -march=armv6k \
    -mtune=mpcore \
    -mfpu=vfp \
    -DARM11 \
    -D_3DS \
> src/bindgen.rs
