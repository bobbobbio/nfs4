// Copyright 2023 Remi Bernotavicius

use log::LevelFilter;
use std::env;
use std::path::Path;

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Info);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("boot.qcow2");
    vm_runner::create_image(dest.to_str().unwrap());

    println!("cargo:rerun-if-changed=build.rs");
}
