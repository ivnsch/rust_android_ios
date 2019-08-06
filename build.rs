extern crate bindgen;
extern crate cbindgen;

use rust_swig::{JavaConfig, LanguageConfig};
use std::{env, path::Path};

use bindgen::RustTarget;
static IOS_TARGET_HEADER: &str = "target/mobileapp-ios.h";
// add additional iOS frameworks that you wish to link against
static INCLUDE_SYS_IOS_FRAMEWORKS: [&str; 1] = ["CoreFoundation"];

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("-android") {
        gen_for_android();

    } else if target.contains("-apple") {
        // Example of how to include iOS specific frameworks
        for f in INCLUDE_SYS_IOS_FRAMEWORKS.iter() {
            println!("cargo:rustc-link-lib=framework={}", &f);
        }
        gen_for_ios(&target);
    }
}

fn gen_for_android() {
    env_logger::init();
    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = Path::new("src").join("java_glue.rs.in");
    let out_src = Path::new(&out_dir).join("java_glue.rs");

    let swig_gen = rust_swig::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(
            Path::new("app")
                .join("src")
                .join("main")
                .join("java")
                .join("com")
                .join("schuetz")
                .join("rust_android_ios"),
            "com.schuetz.rust_android_ios".into(),
        )
            .use_null_annotation_from_package("android.support.annotation".into()),
    )).rustfmt_bindings(true);

    swig_gen.expand("android bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed={}", in_src.display());
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
