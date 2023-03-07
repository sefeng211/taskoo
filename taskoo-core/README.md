# Taskoo Core
This directory contains the Taskoo source files that performs the actual
functionalities. All frontends rely on them.

## Compilation
`taskoo-core` can be compiled standalone (though you need to use either
`taskoo-web` or `taskoo-cli` to use it).

To compile it for `taskoo-cli`, run
```
cargo build
```

To compile it for `taskoo-web`, run
```
./build_wasm.sh
```

##
# Installation
`rustup target add wasm32-wasi`

## Core
### Task

