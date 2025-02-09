use std::convert::TryInto;
extern crate serde_json;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::operation;
use crate::core::Operation;
use crate::wasm::helpers::read_data_from_js;

pub use crate::db::task_helper::Task;

static mut LENGTH: usize = 123;

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_char {
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
pub extern "C" fn get_shared_buffer_size() -> i32 {
    unsafe { return LENGTH.try_into().unwrap() }
}

#[no_mangle]
pub extern "C" fn free_shared_buffer(ptr: *mut i8) {
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

// List Operation
#[no_mangle]
pub unsafe fn list(ptr: *mut u8, len: usize) -> *mut c_char {
    let data = read_data_from_js(ptr, len);
    let mut operations = operation::Get::new2(&data).expect("Failed to create the get operation");

    let mut ret: Vec<(String, &Vec<Task>)> = vec![];

    for operation_tuple in operations.iter_mut() {
        let serded_string: String = match operation::execute(&mut operation_tuple.1) {
            Ok(_) => {
                println!("got ok here");
                serde_json::to_string(&operation_tuple.1.get_result()).unwrap()
            }
            Err(e) => {
                println!("got err here");
                e.to_string()
            }
        };

        println!("serded_string {} ", serded_string);
        ret.push((operation_tuple.0.to_owned(), operation_tuple.1.get_result()));
    }
    let serded_ret: String = serde_json::to_string(&ret).unwrap();
    println!("serded_ret {} ", serded_ret);
    unsafe { LENGTH = serded_ret.len() }
    let s = CString::new(serded_ret).unwrap();
    return s.into_raw();
}

// Add Operation
#[no_mangle]
pub unsafe fn add(ptr: *mut u8, len: usize) {
    let data = read_data_from_js(ptr, len);
    let mut operation = operation::Add::new(&data).expect("Failed to create the add operation");
    operation::execute(&mut operation).expect("Failed to execute the operation");
    let added_tasks = &operation.get_result();
    println!("Added a task with id {}", added_tasks[0].id);
}

// Agenda Operation
#[no_mangle]
pub unsafe fn agenda(ptr: *mut u8, len: usize) -> *mut c_char {
    let data = read_data_from_js(ptr, len);
    let mut operation =
        operation::Agenda::new2(&data).expect("Failed to create the agenda operation");

    let serded_string: String = match operation::execute_agenda(&mut operation) {
        Ok(_) => serde_json::to_string(&operation.get_result()).unwrap(),
        Err(e) => e.to_string(),
    };

    println!("serded_ret for agenda {} ", serded_string);
    unsafe { LENGTH = serded_string.len() }
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
}

// State Changing Operation
#[no_mangle]
pub unsafe fn state_change(ptr: *mut u8, len: usize) -> *mut c_char {
    let data = read_data_from_js(ptr, len);
    let mut operation = operation::ModifyOperation::new2(&data)
        .expect("Failed to create the ModifyOperation for state changing");

    let serded_string: String = match operation::execute(&mut operation) {
        Ok(_) => serde_json::to_string(&operation.get_result()).unwrap(),
        Err(e) => e.to_string(),
    };

    unsafe { LENGTH = serded_string.len() }
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
}

// Delete Operation
#[no_mangle]
pub unsafe fn delete(ptr: *mut u8, len: usize) {
    println!("delete in wasm");
    let data = read_data_from_js(ptr, len);
    let mut operation =
        operation::DeleteOperation::new(&data).expect("Failed to create the DeleteOperation");
    operation::execute(&mut operation).expect("Failed to execute the operation");

    // The following doesn't work because taskoo-core doesn't store the delete task ids

    // let deleted_tasks = &operation.get_result();
    // let serded_ret: String = serde_json::to_string(&deleted_tasks).unwrap();
    // println!("serded_ret {} for delete", serded_ret);
    // unsafe { LENGTH = serded_ret.len() }
    // let s = CString::new(serded_ret).unwrap();
    // Return the delete tasks back to clients
    // return s.into_raw();
}
