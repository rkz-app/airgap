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

    // =========================================================
    // 2️⃣ iOS only: compile ObjC
    // =========================================================
    if target.contains("apple-ios") {

        cc::Build::new()
            .files([
                "objc/AGQRResult.m",
                "objc/AGEncoder.m",
                "objc/AGDecoder.m",
            ])
            .flag("-fobjc-arc")
            .include("include")   // <-- header is here now
            .include("objc")
            .compile("airgap_objc");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=static=airgap_objc");
    }
}