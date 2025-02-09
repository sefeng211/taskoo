# Taskoo
Yet another task management App, written in Rust.

## Introduction
This is the monorepo that contains three projects.

`taskoo-core`: Contains the code that interacts with the local database. Both `taskoo-cli` and `taskoo-web` requires this project to be compiled
to work. This enpowers taskoo to have multiple interfaces, while only need to main a single copy
of code for its backend.

`taskoo-cli`: It's the repo for the command line interface for taskoo. You don't need to build
`taskoo-web` if you only need the CLI version.

`tasko-web`: It's the repo for the web interface.


## Development
### Build the command-line version
#### taskoo-cli
```
make build-cli
```
This command compiles `taskoo-cli` along with `taskoo-core`, no need to compile `tasko-core` again.

#### To only build `taskoo-core`
```
make build-core
```
This command builds `taskoo-core` as a library, however this library can only be used for
`taskoo-cli`. If you need to build `taskoo-core` for the web interface, please see below.

### Build the web version
First, make sure you have [wask-sdk-16](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-16) installed in your system.

`wasm32-wasip1` is also a required build target, so execute
```
rustup target add wasm32-wasipi
```

There are a couple of steps required.

1. Set `WASI_SDK_PATH` to be the path for `wask-sdk-16`.
2. Run `make build-web`
