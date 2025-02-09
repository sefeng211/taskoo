'use strict';

import process from 'process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const CONFIG_DIR = "/home/sefeng/.config";
const TASKOO_CONFIG_DIR = "/home/sefeng/.config/taskoo";
const WASM_PATH = "./taskoo_core.wasm";

const wasi = new WASI({
  // Same as --mapdir of wasmtime, map virtual filesystem to host filesystem
  preopens: {
    '/home/sefeng/.config': CONFIG_DIR,
    '/home/sefeng/.config/taskoo': TASKOO_CONFIG_DIR,
    '~/.config': CONFIG_DIR,
    '~/.config/taskoo': TASKOO_CONFIG_DIR,
  },
  version : "preview1",
  env: {
    RUST_LOG: "debug", // Enable the debug logging for Taskoo
    HOME: "/home/sefeng" // Enable the debug logging for Taskoo
  }
});

// pass import Object to WASM to use host APIs
const importObject = {
  wasi_snapshot_preview1: wasi.wasiImport ,
  env: {
    memory: new WebAssembly.Memory({ initial: 256 }), // ✅ Provide memory if required
    sqlite3_os_init: () => 0,  // ✅ Stub function (if required)
  }
};

console.log(process.env.WASM_PATH);
const wasm =
  await WebAssembly.compile(fs.readFileSync(
    path.resolve(__dirname, WASM_PATH)));
const instance = await WebAssembly.instantiate(wasm, importObject);

// WASI try to initialize WASM instanced
wasi.initialize(instance);

export function run() {
  const offset = instance.exports.print_today_js();
  const len = instance.exports.get_shared_buffer_size();
  const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
  const hello = buffer.reduce((str, cur) => str + String.fromCharCode(cur), '');
  instance.exports.free_shared_buffer(offset);
  return hello
}

// const input = "hel";
// var bytes = new TextEncoder("utf-8").encode(input);
// Copy `data` into the `instance` exported memory buffer.
export function passStringToWASM(input) {
  var bytes = new TextEncoder("utf-8").encode(input);
  const ptr = instance.exports.allocate(bytes.length);
  // the `alloc` function returns an offset in
  // the module's memory to the start of the block
  // create a typed `ArrayBuffer` at `ptr` of proper size
  var mem = new Uint8Array(instance.exports.memory.buffer, ptr, bytes.length);
  // copy the content of `data` into the memory buffer
  mem.set(new Uint8Array(bytes));
  // return the pointer
  instance.exports.upper(ptr, bytes.length);
  return ptr;
}

function allocateInput(input) {
  var bytes = new TextEncoder("utf-8").encode(input);
  const ptr = instance.exports.allocate(bytes.length);
  // the `alloc` function returns an offset in
  // the module's memory to the start of the block
  // create a typed `ArrayBuffer` at `ptr` of proper size
  var mem = new Uint8Array(instance.exports.memory.buffer, ptr, bytes.length);
  // copy the content of `data` into the memory buffer
  mem.set(new Uint8Array(bytes));
  return {ptr: ptr, bytes: bytes};
}

export class Endpoints {
  static List(input) {
    const allocated = allocateInput(input);
    const offset = instance.exports.list(allocated.ptr, allocated.bytes.length);
    const len = instance.exports.get_shared_buffer_size();
    const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
    const data = new TextDecoder().decode(buffer);
    instance.exports.free_shared_buffer(offset);
    return data;
  }

  static Agenda(input) {
    const allocated = allocateInput(input);
    const offset = instance.exports.agenda(allocated.ptr, allocated.bytes.length);
    const len = instance.exports.get_shared_buffer_size();
    const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
    const data = new TextDecoder().decode(buffer);
    instance.exports.free_shared_buffer(offset);
    return data;
  }

  static Add(input) {
    const allocated = allocateInput(input);
    instance.exports.add(allocated.ptr, allocated.bytes.length);
  }

  static Delete(input) {
    const allocated = allocateInput(input);
    const offset = instance.exports.delete(allocated.ptr, allocated.bytes.length);
    const len = instance.exports.get_shared_buffer_size();
    const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
    const data = new TextDecoder().decode(buffer);
    instance.exports.free_shared_buffer(offset);
    return data;
  }

  static StateChange(input) {
    const allocated = allocateInput(input);

    const result = instance.exports.state_change(allocated.ptr, allocated.bytes.length);
    const len = instance.exports.get_shared_buffer_size();
    const buffer = new Uint8Array(instance.exports.memory.buffer, result, len);

    const data = new TextDecoder().decode(buffer);
    instance.exports.free_shared_buffer(result);
    return data;
  }
};

