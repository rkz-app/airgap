// build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cbindgen to recognize the 'cbindgen' cfg attribute
    println!("cargo::rustc-check-cfg=cfg(cbindgen)");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("airgap.h");

    // Create include directory if it doesn't exist
    std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);

    println!("cargo:rerun-if-changed=src/");
}