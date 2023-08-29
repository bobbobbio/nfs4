// Copyright 2023 Remi Bernotavicius

use chrono::{offset::TimeZone as _, Local};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use nfs4::{FileAttribute, FileAttributeId, FileAttributes};
use nfs4_client::Result;
use std::net::TcpStream;
use std::path::PathBuf;

fn file_attrs(s: &str) -> std::result::Result<FileAttributes, String> {
    let mut attrs = FileAttributes::default();

    for e in s.split(',') {
        let i = e.find('=').ok_or(String::from("Missing `=`"))?;
        let key = &e[..i];
        let value = &e[(i + 1)..];
        attrs.insert(match key {
            "size" => FileAttribute::Size(value.parse::<u64>().map_err(|e| e.to_string())?),
            "owner" => FileAttribute::Owner(value.into()),
            "owner_group" => FileAttribute::OwnerGroup(value.into()),
            other => return Err(format!("unsupported attribute `{other}`")),
        });
    }

    Ok(attrs)
}

#[derive(Subcommand)]
enum Command {
    GetAttr {
        path: PathBuf,
    },
    SetAttr {
        path: PathBuf,
        #[arg(value_parser = file_attrs)]
        attrs: FileAttributes,
    },
    ReadDir {
        path: PathBuf,
    },
    Remove {
        path: PathBuf,
    },
    Download {
        remote: PathBuf,
        local: PathBuf,
    },
    Upload {
        local: PathBuf,
        remote: PathBuf,
    },
}

#[derive(Parser)]
struct Options {
    host: String,
    #[clap(default_value_t = nfs4_client::NFS_PORT)]
    port: u16,
    #[command(subcommand)]
    command: Command,
}

fn print_listing(entries: &[nfs4::DirectoryEntry]) {
    for e in entries {
        let name = &e.name;
        let mode: &nfs4::Mode = e.attrs.get_as(FileAttributeId::Mode).unwrap();
        let num_links: &u32 = e.attrs.get_as(FileAttributeId::NumLinks).unwrap();
        let owner: &String = e.attrs.get_as(FileAttributeId::Owner).unwrap();
        let size: &u64 = e.attrs.get_as(FileAttributeId::Size).unwrap();

        let modify_raw: &nfs4::Time = e.attrs.get_as(FileAttributeId::TimeModify).unwrap();
        let modify = modify_raw.to_date_time().unwrap();
        let modify_str = Local.from_local_datetime(&modify).unwrap().to_rfc2822();

        println!("{mode:?} {num_links:3} {owner:5} {size:10} {modify_str:31} {name}");
    }
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let transport = TcpStream::connect((opts.host, opts.port))?;
    let mut client = nfs4_client::Client::new(transport)?;
    match opts.command {
        Command::GetAttr { path } => {
            let handle = client.look_up(&path)?;
            let reply = client.get_attr(handle)?;
            println!("{reply:#?}");
        }
        Command::ReadDir { path } => {
            let handle = client.look_up(&path)?;
            let reply = client.read_dir(handle)?;
            print_listing(&reply);
        }
        Command::Remove { path } => {
            let (parent_dir, name) = (path.parent().unwrap(), path.file_name().unwrap());
            let parent = client.look_up(parent_dir)?;
            client.remove(parent, name.to_str().unwrap())?;
        }
        Command::Download { remote, local } => {
            let local_file = if local.to_string_lossy().ends_with('/') {
                local.join(remote.file_name().unwrap())
            } else {
                local
            };

            let handle = client.look_up(&remote)?;
            let mut remote_attrs = client.get_attr(handle.clone())?.object_attributes;
            let size = remote_attrs.remove_as(FileAttributeId::Size).unwrap();

            let progress = ProgressBar::new(size).with_style(
                ProgressStyle::with_template("{wide_bar} {percent}% {binary_bytes_per_sec}")
                    .unwrap(),
            );
            let file = std::fs::File::create(local_file)?;
            client.read_all(handle, progress.wrap_write(file))?;
        }
        Command::SetAttr { path, attrs } => {
            let handle = client.look_up(&path)?;
            client.set_attr(handle, attrs)?;
        }
        Command::Upload { local, remote } => {
            let (parent_dir, name) = if remote.to_string_lossy().ends_with('/') {
                (remote.as_ref(), local.file_name().unwrap())
            } else {
                (remote.parent().unwrap(), remote.file_name().unwrap())
            };

            let parent = client.look_up(parent_dir)?;
            let handle = client.create_file(parent, name.to_str().unwrap())?;

            let file = std::fs::File::open(local)?;
            let progress = ProgressBar::new(file.metadata()?.len()).with_style(
                ProgressStyle::with_template("{wide_bar} {percent}% {binary_bytes_per_sec}")
                    .unwrap(),
            );
            client.write_all(handle, progress.wrap_read(file))?;
        }
    }

    Ok(())
}
