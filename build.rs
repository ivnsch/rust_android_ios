extern crate bindgen;
extern crate cbindgen;

use std::{env, path::Path};

use bindgen::RustTarget;
static IOS_TARGET_HEADER: &str = "target/mobileapp-ios.h";
// add additional iOS frameworks that you wish to link against
static INCLUDE_SYS_IOS_FRAMEWORKS: [&str; 1] = ["CoreFoundation"];

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("-apple") {
        // Example of how to include iOS specific frameworks
        for f in INCLUDE_SYS_IOS_FRAMEWORKS.iter() {
            println!("cargo:rustc-link-lib=framework={}", &f);
        }
        gen_for_ios(&target);
    }
}

fn gen_for_ios(target: &str) {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_header("#include <CoreFoundation/CoreFoundation.h>")
        .generate()
        .expect("Unable to iOS generate bindings")
        .write_to_file(IOS_TARGET_HEADER);
}
