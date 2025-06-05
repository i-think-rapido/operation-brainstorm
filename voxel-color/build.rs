
use cbindgen::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    extern crate cbindgen;

    let output_file = target_dir().join("voxel-color.h").display().to_string();
    let config = Config::from_file("cbindgen.toml").unwrap();

    
    cbindgen::Builder::new()
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);
}

fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}
