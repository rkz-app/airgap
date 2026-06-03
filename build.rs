use std::env;
use std::path::PathBuf;

fn main() {
    // Skip cbindgen for embedded/cross-compilation builds (std feature not enabled)
    if std::env::var("CARGO_FEATURE_STD").is_err() {
        return;
    }

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

    // Wrap embedded-unavailable functions in #ifndef AIRGAP_EMBEDDED guards
    add_embedded_guards(&header_path);

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=cbindgen.toml");
}

/// Adds `#ifndef AIRGAP_EMBEDDED` / `#endif` guards around functions
/// that require `std` or `qr` features and are not available in
/// embedded (no_std) builds.
///
/// Currently guards:
///   - `airgap_encoder_new()`       (requires std + qr)
///   - `airgap_encoder_generate_png()` (requires qr)
fn add_embedded_guards(path: &std::path::Path) {
    let source = std::fs::read_to_string(path).unwrap();
    let mut output = String::new();
    let mut in_encoder_new = false;

    for line in source.lines() {
        if line.contains("airgap_encoder_new(") && !line.contains("with_session_id") {
            in_encoder_new = true;
            output.push_str("#ifndef AIRGAP_EMBEDDED\n");
            output.push_str(line);
            output.push('\n');
        } else if in_encoder_new {
            output.push_str(line);
            if line.trim_end().ends_with(");") {
                output.push_str("\n#endif /* AIRGAP_EMBEDDED */\n");
                in_encoder_new = false;
            } else {
                output.push('\n');
            }
        } else if line.contains("airgap_encoder_generate_png") {
            output.push_str("#ifndef AIRGAP_EMBEDDED\n");
            output.push_str(line);
            output.push_str("\n#endif /* AIRGAP_EMBEDDED */\n");
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    std::fs::write(path, output).unwrap();
}
