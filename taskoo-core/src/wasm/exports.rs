use std::convert::TryInto;
extern crate serde_json;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::command::{ContextCommand, SimpleCommand, StateCommand, TagCommand};
use crate::operation;
use crate::core::Operation;
use crate::wasm::helpers::read_data_from_js;

pub use crate::db::task_helper::Task;

static mut LENGTH: usize = 123;

#[derive(serde::Deserialize)]
struct AnnotationInput {
    task_id: i64,
    annotation: String,
}

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

unsafe fn read_raw_string_from_js(ptr: *mut u8, len: usize) -> String {
    let data = Vec::from_raw_parts(ptr, len, len);
    String::from_utf8(data).unwrap()
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

// Annotation Operation
#[no_mangle]
pub unsafe fn annotation(ptr: *mut u8, len: usize) -> *mut c_char {
    let input = read_raw_string_from_js(ptr, len);
    let payload: AnnotationInput = match serde_json::from_str(&input) {
        Ok(payload) => payload,
        Err(e) => {
            let message = serde_json::json!({"error": e.to_string()}).to_string();
            LENGTH = message.len();
            return CString::new(message).unwrap().into_raw();
        }
    };

    let mut operation = operation::AddAnnotation::new(payload.task_id, payload.annotation);
    let serded_string: String = match operation::execute(&mut operation) {
        Ok(_) => {
            let tasks = operation.get_result();
            if tasks.is_empty() {
                serde_json::json!({"error": "Unable to update annotation"}).to_string()
            } else {
                serde_json::to_string(&tasks[0]).unwrap()
            }
        }
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    };

    unsafe { LENGTH = serded_string.len() }
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
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
    let mut operation = match operation::ModifyOperation::new(&data) {
        Ok(operation) => operation,
        Err(e) => {
            let serded_string = serde_json::json!({"error": e.to_string()}).to_string();
            LENGTH = serded_string.len();
            let s = CString::new(serded_string).unwrap();
            return s.into_raw();
        }
    };

    let serded_string: String = match operation::execute(&mut operation) {
        Ok(_) => serde_json::to_string(&operation.get_result()).unwrap(),
        Err(e) => e.to_string(),
    };

    unsafe { LENGTH = serded_string.len() }
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
}

// Full Modify Operation
#[no_mangle]
pub unsafe fn modify(ptr: *mut u8, len: usize) -> *mut c_char {
    let data = read_data_from_js(ptr, len);
    let mut operation = match operation::ModifyOperation::new(&data) {
        Ok(operation) => operation,
        Err(e) => {
            let message = serde_json::json!({"error": e.to_string()}).to_string();
            LENGTH = message.len();
            return CString::new(message).unwrap().into_raw();
        }
    };

    let serded_string: String = match operation::execute(&mut operation) {
        Ok(_) => serde_json::to_string(&operation.get_result()).unwrap(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    };

    LENGTH = serded_string.len();
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
}

// Metadata for UI filters and editors
#[no_mangle]
pub unsafe fn metadata() -> *mut c_char {
    let contexts = match ContextCommand::new().and_then(|mut command| command.get_all()) {
        Ok(contexts) => contexts,
        Err(_) => vec![],
    };
    let tags = match TagCommand::new().and_then(|mut command| command.get_all()) {
        Ok(tags) => tags,
        Err(_) => vec![],
    };
    let custom_states = match StateCommand::new().and_then(|mut command| command.get_all()) {
        Ok(states) => states,
        Err(_) => vec![],
    };
    let serded_string = serde_json::json!({
        "contexts": contexts,
        "tags": tags,
        "states": ["ready", "started", "blocked", "completed"],
        "custom_states": custom_states,
        "priorities": ["H", "M", "L"]
    })
    .to_string();

    LENGTH = serded_string.len();
    let s = CString::new(serded_string).unwrap();
    return s.into_raw();
}

// Task info operation
#[no_mangle]
pub unsafe fn info(ptr: *mut u8, len: usize) -> *mut c_char {
    let data = read_data_from_js(ptr, len);
    let input = data.join(" ");
    let task_id = match input.trim().parse::<i64>() {
        Ok(task_id) => task_id,
        Err(_) => {
            let message = serde_json::json!({"error": "Task id must be an integer"}).to_string();
            LENGTH = message.len();
            return CString::new(message).unwrap().into_raw();
        }
    };

    let mut operation = operation::Get::new();
    operation.task_id = Some(task_id);

    let serded_string: String = match operation::execute(&mut operation) {
        Ok(_) => {
            let tasks = operation.get_result();
            if tasks.is_empty() {
                serde_json::json!({"error": format!("Unable to find task with id: {}", task_id)})
                    .to_string()
            } else {
                serde_json::to_string(&tasks[0]).unwrap()
            }
        }
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    };

    LENGTH = serded_string.len();
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
