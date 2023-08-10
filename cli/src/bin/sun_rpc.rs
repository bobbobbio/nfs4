// Copyright 2023 Remi Bernotavicius

use clap::Parser;
use std::net::TcpStream;
use sun_rpc_client::Result;

#[derive(Parser)]
struct Options {
    host: String,
    #[clap(default_value_t = sun_rpc_client::PORT_MAPPER_PORT)]
    port: u16,
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let mut transport = TcpStream::connect((opts.host, opts.port))?;
    let mut client = sun_rpc_client::RpcClient::new(sun_rpc_client::PORT_MAPPER);
    client.send_request(&mut transport, sun_rpc_client::NULL_PROCEDURE, ())?;
    client.receive_reply::<()>(&mut transport)?;

    Ok(())
}
