use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_char;
use log::*;
use mpsc::Receiver;
use std::{
    sync::mpsc::{self, Sender},
    thread,
};

#[no_mangle]
pub unsafe extern "C" fn greet(who: *const c_char) -> CFStringRef {
    let str: String = cstring_to_str(&who).into();
    to_cf_str(format!("Hello 👋 {}!", str))
}

#[no_mangle]
pub unsafe extern "C" fn add(
    value1: i32,
    value2: i32,
) -> i32 {
    info!("Passed value1: {}, value2: {}", value1, value2);
    value1 + value2
}

#[no_mangle]
pub unsafe extern "C" fn pass_struct(
    object: *const ParamStruct,
) {
    info!("Received struct from iOS: {:?}", object);
}

#[no_mangle]
pub unsafe extern "C" fn return_struct(
) -> ReturnStruct {
    ReturnStruct { string: to_cf_str("my string parameter".to_owned()), int: 123 }
}

pub static mut CALLBACK_SENDER: Option<Sender<String>> = None;

#[no_mangle]
pub unsafe extern "C" fn register_callback(
    callback: unsafe extern "C" fn(CFStringRef),
) {
    register_callback_internal(Box::new(callback));

    // Let's send a message immediately, to test it
    send_to_callback("Hello callback!".to_owned());
}

// Convert C string to Rust string slice
unsafe fn cstring_to_str<'a>(cstring: &'a *const c_char) -> &str {
    if cstring.is_null() {
        // Of course in a real project you'd return Result instead
        panic!("cstring is null")
    }

    let raw = ::std::ffi::CStr::from_ptr(*cstring);
    raw.to_str().expect("Couldn't convert c string to slice")
}

fn to_cf_str(str: String) -> CFStringRef {
    let cf_string = CFString::new(&str);
    let cf_string_ref = cf_string.as_concrete_TypeRef();
    ::std::mem::forget(cf_string);
    cf_string_ref
}

unsafe fn send_to_callback(string: String) {
    match &CALLBACK_SENDER {
        Some(s) => {
            s.send(string).expect("Couldn't send message to callback!");
        }
        None => {
            info!("No callback registered");
        }
    }
}

fn register_callback_internal(callback: Box<dyn MyCallback>) {
    // Make callback implement Send (marker for thread safe, basically) https://doc.rust-lang.org/std/marker/trait.Send.html
    let my_callback =
        unsafe { std::mem::transmute::<Box<dyn MyCallback>, Box<dyn MyCallback + Send>>(callback) };

    // Create channel
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Save the sender in a static variable, which will be used to push elements to the callback
    unsafe {
        CALLBACK_SENDER = Some(tx);
    }

    // Thread waits for elements pushed to SENDER and calls the callback
    thread::spawn(move || {
        for string in rx.iter() {
            let cf_string = to_cf_str(string);
            my_callback.call(cf_string)
        }
    });
}

pub trait MyCallback {
    fn call(&self, par: CFStringRef);
}

impl MyCallback for unsafe extern "C" fn(CFStringRef) {
    fn call(&self, par: CFStringRef) {
        unsafe {
            self(par);
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ParamStruct {
    string: *const c_char,
    int: i32
}
#[repr(C)]
#[derive(Debug)]
pub struct ReturnStruct {
    string: CFStringRef,
    int: i32
}
