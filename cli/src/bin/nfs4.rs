// Copyright 2023 Remi Bernotavicius

use clap::{Parser, Subcommand};
use nfs4::FileAttributeId;
use nfs4_client::Result;
use std::net::TcpStream;

#[derive(Subcommand)]
enum Command {
    GetAttr { path: String },
    Download { remote: String, local: String },
}

#[derive(Parser)]
struct Options {
    host: String,
    #[clap(default_value_t = nfs4_client::NFS_PORT)]
    port: u16,
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let mut transport = TcpStream::connect((opts.host, opts.port))?;
    let mut client = nfs4_client::Client::new(&mut transport)?;
    match opts.command {
        Command::GetAttr { path } => {
            let reply = client.get_attr(&mut transport, &path)?;
            println!("{reply:#?}");
        }
        Command::Download { remote, local } => {
            let mut reply = client.get_attr(&mut transport, &remote)?;
            let handle = reply
                .object_attributes
                .remove_as(FileAttributeId::FileHandle)
                .unwrap();
            let file = std::fs::File::create(local)?;
            client.read_all(&mut transport, handle, file)?;
            println!("downloaded");
        }
    }

    Ok(())
}
