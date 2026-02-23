use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(cbindgen)");

    let target = env::var("TARGET").unwrap_or_default();

    // =========================================================
    // iOS-SPECIFIC BUILD LOGIC
    // =========================================================
    if target.contains("apple-ios") {
        // Enforce iOS 12 deployment target
        println!("cargo:rustc-env=IPHONEOS_DEPLOYMENT_TARGET=12.0");

        if target.contains("sim") {
            println!("cargo:rustc-link-arg=-mios-simulator-version-min=12.0");
        } else {
            println!("cargo:rustc-link-arg=-miphoneos-version-min=12.0");
        }

        // Compile Objective-C wrappers
        cc::Build::new()
            .files([
                "objc/AGQRResult.m",
                "objc/AGEncoder.m",
                "objc/AGDecoder.m",
            ])
            .flag("-fobjc-arc")
            .include("include")
            .include("objc")
            .compile("airgap_objc");

        // Link Apple frameworks
        println!("cargo:rustc-link-lib=framework=Foundation");

        // Rebuild triggers
        println!("cargo:rerun-if-changed=objc/AGQRResult.m");
        println!("cargo:rerun-if-changed=objc/AGEncoder.m");
        println!("cargo:rerun-if-changed=objc/AGDecoder.m");
        println!("cargo:rerun-if-changed=objc/");
    }

    // =========================================================
    // HEADER GENERATION (ALL PLATFORMS)
    // =========================================================
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("airgap.h");

    std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(
            cbindgen::Config::from_file("cbindgen.toml")
                .expect("Unable to load cbindgen.toml"),
        )
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);

    println!("cargo:rerun-if-changed=src/");
}