# Requirement
* Sqlite >= 3.6.19 (Foreign_key support)

## WebAssembly Support
In theory, we can compile this library to WASM to make it usable in browser,
such that browsers can be clients as well. However, there's a blocking issue
(https://github.com/rusqlite/rusqlite/issues/873).

* Update
Running taskoo in the browser is now partially supported, it's not
perfect, still need more investigation to make it fully functional.

The idea is to compile `taskoo-core` into a wasm executable, and then
run it in the node runtime. (node has an experiment wasi feature).
Run `./build_wasm.sh` to compile the wasm executable.

Run `node --experimental-wasi-unstable-preview1 --no-warnings index.mjs`

## Core
### Task

