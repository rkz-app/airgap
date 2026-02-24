use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    // =========================================================
    // 1️⃣ Generate header FIRST
    // =========================================================
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let header_path = PathBuf::from(&crate_dir)
        .join("include")
        .join("airgap.h");

    std::fs::create_dir_all(header_path.parent().unwrap()).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&header_path);

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=cbindgen.toml");


    if target.contains("apple-ios") {
        let mut build = cc::Build::new();

        build
            .files([
                "objc/AGQRResult.m",
                "objc/AGEncoder.m",
                "objc/AGDecoder.m",
            ])
            .flag("-fobjc-arc")
            .flag("-miphoneos-version-min=10.0")
            .include("include")
            .include("objc")
            .compile("airgap_objc");

        let out_dir = env::var("OUT_DIR").unwrap();

        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static=airgap_objc");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=objc");

        println!("cargo:rustc-link-arg=-Wl,-ObjC");
    }
}