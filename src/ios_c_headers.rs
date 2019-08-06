use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use libc::c_char;
use std::os::raw::c_void;
use crate::MyRustStruct;

// A container for Rust structs
#[repr(C)]
pub struct Session {
    my_rust_struct: *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn session_new() -> *mut Session {
    // Create a worker
    let mut my_rust_struct = MyRustStruct::new();
    // Convert into a void pointer
    let my_rust_struct_ptr: *mut c_void = &mut my_rust_struct as *mut _ as *mut c_void;
    // Create a container
    let session = Session { my_rust_struct: my_rust_struct_ptr };
    // Return the raw pointer of the container on the heap
    Box::into_raw(Box::new(session))
}

#[no_mangle]
pub unsafe extern "C" fn session_greet(session: *mut Session, to: *const c_char) -> CFStringRef {
    // Get input string as Rust String
    let string = cstring_to_str(&to).unwrap();
    // Get worker pointer as Rust pointer
    let my_rust_struct_ptr = (*session).my_rust_struct;
    let my_rust_struct: &mut MyRustStruct = &mut *(my_rust_struct_ptr as *mut MyRustStruct);
    // Call the `action` method
    let string = my_rust_struct.greet(string);
    // Create a Objective-C String
    let cf_string = CFString::new(&string);
    let cf_string_ref = cf_string.as_concrete_TypeRef();
    // Tell Rust to not manage this memory
    ::std::mem::forget(cf_string);
    return cf_string_ref;
}

#[no_mangle]
pub unsafe extern "C" fn session_add(session: *mut Session, number: i32) -> i32 {
    let my_rust_struct_ptr = (*session).my_rust_struct;
    let my_rust_struct: &mut MyRustStruct = &mut *(my_rust_struct_ptr as *mut MyRustStruct);
    let result = my_rust_struct.add(number);
    ::std::mem::forget(result);
    return result;
}

// Useful conversion
unsafe fn cstring_to_str<'a>(cstring: &'a *const c_char) -> Option<&str> {
    if cstring.is_null() {
        return None;
    }
    let raw = ::std::ffi::CStr::from_ptr(*cstring);
    match raw.to_str() {
        Ok(s) => Some(s),
        Err(_) => None
    }
}
