use std::convert::TryInto;
extern crate serde_json;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::operation;

static HELLO: &'static str = "Hello, WASM!";
static mut LENGTH: usize = 123;
#[no_mangle]
pub extern "C" fn print_today_js() -> *mut c_char {
    println!("Start!!!");
    let mut operation1 = operation::Get::new();
    operation1.task_id = Some(1);

    let mut operation = operation::Agenda::new(String::from("today"), Some(String::from("today")));

    match operation::execute_agenda(&mut operation) {
        Ok(()) => {
            let results = operation.get_result();
            let serded_string: String = serde_json::to_string(&results).unwrap();
            unsafe { LENGTH = serded_string.len() }
            let s = CString::new(serded_string).unwrap();
            return s.into_raw();
        }
        Err(error) => println!("{:?}", error),
    }
    match operation::execute2(&mut operation1) {
        Ok(result) => {
            println!("returned 1!!");

            let serded_string: String = serde_json::to_string(&result).unwrap();
            unsafe { LENGTH = serded_string.len() }
            let s = CString::new(serded_string).unwrap();
            return s.into_raw();
        }
        Err(error) => println!("{:?}", error),
    };
    println!("returned 2!!");
    let s = CString::new(HELLO).unwrap();
    return s.into_raw();
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_char {
    println!("allocate!!!");
    // create a new mutable buffer with capacity `len`
    let mut buf = Vec::with_capacity(size);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure that its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    std::mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    println!("returned {:?}", ptr);
    return ptr;
}
/// Given a pointer to the start of a byte array and
/// its length, return the sum of its elements.
#[no_mangle]
pub unsafe fn read_data(ptr: *mut u8, len: usize) {
    // create a Vec<u8> from the pointer to the
    // linear memory and the length
    println!("read {:?}", ptr);
    let data = Vec::from_raw_parts(ptr, len, len);
    println!("{:?}", data);
}

#[no_mangle]
pub extern "C" fn print_hello_size() -> i32 {
    unsafe { return LENGTH.try_into().unwrap() }
}

#[no_mangle]
pub extern "C" fn print_hello_free(ptr: *mut i8) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

/// Given a pointer to the start of a byte array and
/// its length, read a string, create its uppercase
/// representation, then return the pointer to it
#[no_mangle]
pub unsafe fn upper(ptr: *mut u8, len: usize) {
    // create a `Vec<u8>` from the pointer and length
    // here we could also use Rust's excellent FFI
    // libraries to read a string, but for simplicity,
    // we are using the same method as for plain byte arrays
    let data = Vec::from_raw_parts(ptr, len, len);
    // read a Rust `String` from the byte array,
    let input_str = String::from_utf8(data).unwrap();

    println!("read {:?}", input_str);
}
