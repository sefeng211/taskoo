#!/usr/bin/env bash

# Set WASI_SDK_PATH to the correct location in your system.
# It should look like "/home/sefeng/.local/share/wasi-sdk-16.0"
# Currently only wasi-sdk-16 is tested/supported.

if [ -z "${WASI_SDK_PATH}" ]; then
    echo "WASI_SDK_PATH is not set, exit"
    exit 1
fi

export WASI_SYSROOT="${WASI_SDK_PATH}/share/wasi-sysroot"
export CC="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SYSROOT}"
export AR="${WASI_SDK_PATH}/bin/llvm-ar"
export CC_wasm32_wasi="${CC}"
export CARGO_TARGET_WASM32_WASI_LINKER="${WASI_SDK_PATH}/bin/wasm-ld"

export LIBSQLITE3_FLAGS="\
    -DSQLITE_OS_OTHER \
    -USQLITE_TEMP_STORE \
    -DSQLITE_TEMP_STORE=3 \
    -USQLITE_THREADSAFE \
    -DSQLITE_THREADSAFE=0 \
    -DSQLITE_OMIT_LOCALTIME \
    -DSQLITE_OMIT_LOAD_EXTENSION \
    -DLONGDOUBLE_TYPE=double"

cargo build --release --target "wasm32-wasip1"
