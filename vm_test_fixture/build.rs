// Copyright 2023 Remi Bernotavicius

use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=OUT_DIR={out_dir}");
}
