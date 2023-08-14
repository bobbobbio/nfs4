// Copyright 2023 Remi Bernotavicius

use clap::Parser;
use log::LevelFilter;
use std::io::BufRead as _;
use std::path::PathBuf;

#[derive(Parser)]
struct ImageBuilderOptions {
    output: PathBuf,
}

#[derive(Parser)]
struct VmRunnerOptions {
    boot_disk: PathBuf,
    ports: Vec<u16>,
}

#[derive(Parser)]
enum Options {
    BuildImage(ImageBuilderOptions),
    RunVm(VmRunnerOptions),
}

fn main() {
    simple_logging::log_to_stderr(LevelFilter::Info);

    let opts = Options::parse();

    match opts {
        Options::BuildImage(opts) => vm_runner::create_image(opts.output),
        Options::RunVm(opts) => vm_runner::run_vm(opts.boot_disk, &opts.ports, |m| {
            println!("forwarded ports: {:#?}", m.forwarded_ports());

            println!("VM is running, press enter to exit");

            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line).unwrap();
        }),
    }
}
