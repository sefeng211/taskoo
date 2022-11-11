'use strict';

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const wasi = new WASI({
  // Same as --mapdir of wasmtime, map virtual filesystem to host filesystem
  preopens: {
    '/home/sefeng/.config': '/home/sefeng/.config/',
    '/home/sefeng/.config/taskoo': '/home/sefeng/.config/taskoo',
  },
});

// pass import Object to WASM to use host APIs
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

// Load, and compile, and instantiate WASM
// const wasm =
//   await WebAssembly.compile(fs.readFileSync(
//     path.resolve(__dirname,
//       './target/wasm32-wasi/debug/node_wasm.wasm')));
const wasm =
  await WebAssembly.compile(fs.readFileSync(
    path.resolve(__dirname,
      './taskoo_core_check.wasm')));
const instance = await WebAssembly.instantiate(wasm, importObject);

// WASI try to initialize WASM instanced
wasi.initialize(instance);

// Run WASM function
instance.exports.print_hello();
instance.exports.print_today_js();
instance.exports.print_hello_size();
instance.exports.print_hello_free();

export function run() {
  const offset = instance.exports.print_today_js();
  const len = instance.exports.print_hello_size();
  const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
  const hello = buffer.reduce((str, cur) => str + String.fromCharCode(cur), '');

  console.log(hello);
  instance.exports.print_hello_free(offset);
  return hello
}

const data = ["a", "b", "c"];
const input = "hel";
var bytes = new TextEncoder("utf-8").encode(input);
const ptr = instance.exports.allocate(3);
// Copy `data` into the `instance` exported memory buffer.
function copyMemory() {
  // the `alloc` function returns an offset in
  // the module's memory to the start of the block
  // create a typed `ArrayBuffer` at `ptr` of proper size
  var mem = new Uint8Array(instance.exports.memory.buffer, ptr, data.length);
  // copy the content of `data` into the memory buffer
  mem.set(new Uint8Array(bytes));
  // return the pointer
  console.log(ptr);
  return ptr;
}
copyMemory();
instance.exports.upper(ptr, 3);

// export function pass_data() {
// }

