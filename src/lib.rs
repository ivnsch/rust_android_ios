use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[cfg(target_os = "ios")]
extern crate core_foundation;
#[cfg(target_os = "ios")]
mod ios_c_headers;
#[cfg(target_os = "ios")]
extern crate libc;
#[cfg(target_os = "ios")]
pub use ios_c_headers::*;

use log::info;
#[cfg(target_os = "android")]
mod ffi_android;

pub struct MyRustStruct {
    a: i32,
}

static mut sender: Option<Sender<i32>> = None;

#[derive(Serialize, Deserialize)]
struct MyRustStructForJsonExample {
    string_field: String,
    int_field: i32
}

impl MyRustStruct {
    #[no_mangle]
    pub extern fn new() -> MyRustStruct {
        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("Hello"),
        );
        log_panics::init(); // log panics rather than printing them
        info!("init log system - done");
        MyRustStruct { a: 2 }
    }

    #[no_mangle]
    pub extern fn add(&self, val: i32) -> i32 {
        self.a + val
    }

    #[no_mangle]
    pub extern fn greet(&self, to: &str) -> String {
        format!("Hello {} ✋\nIt's a pleasure to meet you!", to)
    }

    #[no_mangle]
    pub extern fn json(&self, parameter: &str) -> String {
        let decoded: MyRustStructForJsonExample = serde_json::from_str(parameter).unwrap();
        let my_struct = MyRustStructForJsonExample {
            string_field: format!("{} {}", decoded.string_field, "updated"),
            int_field: decoded.int_field + 1
        };
        return serde_json::to_string(&my_struct).unwrap();
    }

    #[no_mangle]
    pub extern fn function_with_callback(&self, callback: Box<Callback>) {
        callback.call(123, false);
    }

    #[no_mangle]
    pub extern fn observe(&self, callback: Box<Callback>) {
        let my_callback = unsafe { std::mem::transmute::<Box<dyn Callback>, Box<dyn Callback + Send>>(callback) };
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        unsafe { sender = Some(tx); }
        thread::spawn(move || {
            for i in rx.iter() {
                my_callback.call(i, true)
            }
        });
    }

    #[no_mangle]
    pub extern fn send_to_observers(&self, val: i32) {
        unsafe {
            match &sender {
                Some(s) => { s.send(val); },
                None => println!("No callback registered"),
            };
        }
    }
}

pub trait Callback {
    fn call(&self, a_number: i32, a_boolean: bool);
}
