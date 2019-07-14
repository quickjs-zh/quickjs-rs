extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        Command::new("make")
                .current_dir("QuickJS")
                .status()
                .expect("failed to make!");
    } else {
        unimplemented!()
    }

    println!("cargo:rustc-link-search=QuickJS");
    println!("cargo:rustc-link-lib=quickjs");
    
    let bindings = bindgen::Builder::default()
        .header("./src/wrapper.h")
        .clang_arg("-IQuickJS")
        .generate()
        .expect("Unable to generate bindings");

    // 文件写入到$OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}