'use strict';

import process from 'process';
import fs from 'fs';
import path from 'path';
import os from 'os';
import { fileURLToPath } from 'url';
import { WASI } from 'wasi';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const TASKOO_HOME = process.env.TASKOO_HOME || process.env.HOME || os.homedir();
const CONFIG_DIR = path.join(TASKOO_HOME, ".config");
const TASKOO_CONFIG_DIR = path.join(CONFIG_DIR, "taskoo");
const CONFIG_PATH = path.join(TASKOO_CONFIG_DIR, "config");
const WASM_PATH = "./taskoo_core.wasm";

function expandHome(value) {
  if (value === "~") {
    return TASKOO_HOME;
  }
  if (value.startsWith("~/")) {
    return path.join(TASKOO_HOME, value.slice(2));
  }
  return value;
}

function readConfiguredDbDir() {
  if (!fs.existsSync(CONFIG_PATH)) {
    return TASKOO_CONFIG_DIR;
  }

  const config = fs.readFileSync(CONFIG_PATH, "utf8");
  const dbPathLine = config
    .split(/\r?\n/)
    .find((line) => line.trim().startsWith("db_path="));
  if (!dbPathLine) {
    return TASKOO_CONFIG_DIR;
  }

  const dbPath = expandHome(dbPathLine.split("=").slice(1).join("=").trim());
  return path.dirname(dbPath);
}

fs.mkdirSync(TASKOO_CONFIG_DIR, { recursive: true });
const TASKOO_DB_DIR = process.env.TASKOO_DB_DIR || readConfiguredDbDir();
fs.mkdirSync(TASKOO_DB_DIR, { recursive: true });

const wasi = new WASI({
  // Same as --mapdir of wasmtime, map virtual filesystem to host filesystem
  preopens: {
    [CONFIG_DIR]: CONFIG_DIR,
    [TASKOO_CONFIG_DIR]: TASKOO_CONFIG_DIR,
    [TASKOO_DB_DIR]: TASKOO_DB_DIR,
    [path.join(TASKOO_HOME, ".config")]: CONFIG_DIR,
    [path.join(TASKOO_HOME, ".config", "taskoo")]: TASKOO_CONFIG_DIR,
    '~/.config': CONFIG_DIR,
    '~/.config/taskoo': TASKOO_CONFIG_DIR,
  },
  version : "preview1",
  env: {
    RUST_LOG: "debug", // Enable the debug logging for Taskoo
    HOME: TASKOO_HOME,
    RUST_BACKTRACE: "full"
  }
});

// pass import Object to WASM to use host APIs
const importObject = {
  wasi_snapshot_preview1: wasi.wasiImport ,
  env: {
    memory: new WebAssembly.Memory({ initial: 256 }), // ✅ Provide memory if required
    // sqlite3_os_init: () => 0,  // ✅ Stub function (if required)
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

function readSharedString(offset) {
  const len = instance.exports.get_shared_buffer_size();
  const buffer = new Uint8Array(instance.exports.memory.buffer, offset, len);
  const data = new TextDecoder().decode(buffer);
  instance.exports.free_shared_buffer(offset);
  return data;
}

export class Endpoints {
  static List(input) {
    const allocated = allocateInput(input);
    const offset = instance.exports.list(allocated.ptr, allocated.bytes.length);
    return readSharedString(offset);
  }

  static Agenda(input) {
    const allocated = allocateInput(input);
    const offset = instance.exports.agenda(allocated.ptr, allocated.bytes.length);
    return readSharedString(offset);
  }

  static Add(input) {
    const allocated = allocateInput(input);
    instance.exports.add(allocated.ptr, allocated.bytes.length);
    return JSON.stringify({ok: true});
  }

  static Body(input) {
    const allocated = allocateInput(input);
    const result = instance.exports.body(allocated.ptr, allocated.bytes.length);
    return readSharedString(result);
  }

  static Annotation(input) {
    const allocated = allocateInput(input);
    const result = instance.exports.annotation(allocated.ptr, allocated.bytes.length);
    return readSharedString(result);
  }

  static Delete(input) {
    const allocated = allocateInput(input);
    instance.exports.delete(allocated.ptr, allocated.bytes.length);
    return JSON.stringify({ok: true});
  }

  static StateChange(input) {
    const allocated = allocateInput(input);

    const result = instance.exports.state_change(allocated.ptr, allocated.bytes.length);
    return readSharedString(result);
  }

  static Modify(input) {
    const allocated = allocateInput(input);
    const result = instance.exports.modify(allocated.ptr, allocated.bytes.length);
    return readSharedString(result);
  }

  static Info(input) {
    const allocated = allocateInput(input);
    const result = instance.exports.info(allocated.ptr, allocated.bytes.length);
    return readSharedString(result);
  }

  static Metadata() {
    const result = instance.exports.metadata();
    return readSharedString(result);
  }
};
