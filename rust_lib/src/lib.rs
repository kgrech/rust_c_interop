extern crate core;

use std::os::raw::c_char;
use std::ffi::{c_void, CStr, CString};

const STRING: &str = "Hello from rust";

// ==== Passing rust string to C ====

// Option 1: Provide create and delete methods
#[no_mangle]
pub extern fn create_string() -> *const c_char {
    let c_string = CString::new(STRING).expect("CString::new failed");
    c_string.into_raw() // Move ownership to C
}

/// # Safety
/// The ptr should be a valid pointer to the string allocated by rust
#[no_mangle]
pub unsafe extern fn free_string(ptr: *const c_char) {
    // Take the ownership back to rust and drop the owner
    let _ = CString::from_raw(ptr as *mut _);
}

// Option 2: Pass the buffer
#[no_mangle]
pub extern fn get_string_len() -> usize {
    STRING.as_bytes().len() + 1
}

/// # Safety
/// The ptr should be a valid pointer to the buffer of required size
#[no_mangle]
pub unsafe extern fn copy_string(ptr: *mut c_char) {
    let bytes = STRING.as_bytes();
    let len = bytes.len();
    std::ptr::copy(STRING.as_bytes().as_ptr().cast(), ptr, len);
    std::ptr::write(ptr.offset(len as isize) as *mut u8, 0u8);
}

// Option 3: Pass memory allocator to rust
type Allocator = unsafe extern fn(usize) -> *mut c_void;

/// # Safety
/// The allocator function should return a pointer to a valid buffer
#[no_mangle]
pub unsafe extern fn get_string_with_allocator(allocator: Allocator) -> *mut c_char {
    let ptr: *mut c_char = allocator(get_string_len()).cast();
    copy_string(ptr);
    ptr
}

// Option 4: Call libc in rust
#[no_mangle]
pub unsafe extern fn get_string_with_malloc() -> *mut c_char {
    let ptr: *mut c_char = libc::malloc(get_string_len()).cast();
    copy_string(ptr);
    ptr
}

// Option 5: Borrow rust string
type Callback = unsafe extern fn(*const c_char);

#[no_mangle]
pub unsafe extern fn get_string_in_callback(callback: Callback) {
    let c_string = CString::new(STRING).expect("CString::new failed");
    // as_ptr() keeps ownership in rust unlike into_raw()
    callback(c_string.as_ptr())
}

// ==== Passing C string to rust ====
#[no_mangle]
/// # Safety
/// The ptr should be a pointer to valid String
pub unsafe extern fn print_c_string(ptr: *const c_char) {
    let c_str = CStr::from_ptr(ptr);
    let rust_str = c_str.to_str().expect("Bad encoding");
    // calling libc::free(ptr as *mut _); causes use after free vulnerability
    println!("1. Printed from rust: {}", rust_str);
    let owned = rust_str.to_owned();
    // calling libc::free(ptr as *mut _); does not cause after free vulnerability
    println!("2. Printed from rust: {}", owned);
}