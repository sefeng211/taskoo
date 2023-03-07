// Data that are passed from JS are allocated in shared memory
// This function reads the data from the shared memory and
// convert them into an array in Rust.
pub unsafe fn read_data_from_js(ptr: *mut u8, len: usize) -> Vec<String> {
    let data = Vec::from_raw_parts(ptr, len, len);
    let input_str = String::from_utf8(data).unwrap();

    let input_in_array = input_str.split_whitespace();

    let ret = input_in_array.map(String::from).collect();
    println!("read {:?}", ret);
    ret
}
