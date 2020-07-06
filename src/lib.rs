#[cfg(target_os = "ios")]
mod ffi_ios;
#[cfg(target_os = "android")]
mod ffi_android;

// Core functionality goes here (or any other Rust file).
// This demo is only about FFI, so empty.
// Possible structures:
// - For simple calculations or services: functions.
// - For more complex scenarios: e.g. function that bootstraps a dependency graph,
//   stored in a static variable. the FFI/JNI functions call the dependency graph's functions.
