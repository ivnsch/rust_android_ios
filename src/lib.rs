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
mod java_glue;
#[cfg(target_os = "android")]
pub use crate::java_glue::*;

pub struct MyRustStruct {
    a: i32,
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
    pub extern fn function_with_callback(&self, callback: Box<Callback>) {
        callback.call(123, false);
    }
}

pub trait Callback {
    fn call(&self, a_number: i32, a_boolean: bool);
}
