// Copyright 2023 Remi Bernotavicius

use fs2::FileExt as _;
use log::LevelFilter;
use std::fs::File;
use std::path::{Path, PathBuf};
use vm_runner::Machine;

fn build_boot_image(path: &Path) {
    log::info!("vm_test_fixture: building boot disk: {path:?}");
    vm_runner::create_image(path.to_str().unwrap());
}

fn maybe_build_boot_image(path: &Path) {
    if path.exists() {
        log::info!("vm_test_fixture: boot disk already exists: {path:?}");
        return;
    }

    let lock_path = path.parent().unwrap().join(".vm_test_fixture.lock");
    log::info!("vm_test_fixture: lock file path: {lock_path:?}");

    let f = File::create(lock_path).unwrap();
    f.lock_exclusive().unwrap();

    if path.exists() {
        log::info!("vm_test_fixture: boot disk already exists: {path:?}");
        return;
    }

    build_boot_image(path);
}

pub fn fixture(ports: &[u16], body: impl FnOnce(&mut Machine)) {
    simple_logging::log_to_stderr(LevelFilter::Info);

    let sha = vm_runner::VM_HASH;
    let boot_image = PathBuf::from(env!("OUT_DIR")).join(format!("boot-{sha}.qcow2"));
    maybe_build_boot_image(&boot_image);
    vm_runner::run_vm(boot_image, ports, body);
}
