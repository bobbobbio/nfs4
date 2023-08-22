// copyright 2023 Remi Bernotavicius

use std::env;
use std::path::PathBuf;

fn main() {
    let src_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/lib.rs");
    let hash = sha256::try_digest(src_path).unwrap();
    println!("cargo:rustc-env=VM_HASH={}", hash);
}
