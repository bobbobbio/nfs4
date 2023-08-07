// Copyright 2023 Remi Bernotavicius

use derive_more::From;
use std::io;

use sun_rpc::{CallBody, Message, MessageBody, OpaqueAuth, Xid};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Deseralization(serde_xdr::CompatDeserializationError),
    Serialization(serde_xdr::CompatSerializationError),
    Io(io::Error),
}

pub trait Transport: io::Read + io::Write {}

impl<T> Transport for T where T: io::Read + io::Write {}

const PORT_MAPPER: u32 = 100000;
pub const PORT_MAPPER_PORT: u16 = 111;

pub fn do_ping(mut transport: &mut impl Transport) -> Result<Message<()>> {
    let message = Message {
        xid: Xid(1),
        body: MessageBody::Call(CallBody {
            rpc_version: 2,
            program: PORT_MAPPER,
            version: 4,
            procedure: 0,
            credential: OpaqueAuth::none(),
            verifier: OpaqueAuth::none(),
            call_args: (),
        }),
    };
    let mut serialized = vec![0; 4];
    serde_xdr::to_writer(&mut serialized, &message)?;

    let fragment_header = (serialized.len() - 4) as u32 | 0x1 << 31;
    serde_xdr::to_writer(&mut &mut serialized[..4], &fragment_header)?;

    transport.write_all(&serialized[..])?;

    let fragment_header: u32 = serde_xdr::from_reader(transport)?;
    let length = fragment_header & !(0x1 << 31);
    let reply: Message<()> =
        serde_xdr::from_reader(&mut io::Read::take(&mut transport, length as u64))?;

    Ok(reply)
}

#[test]
fn ping() {
    vm_test_fixture::fixture(|m| {
        let port = m
            .forwarded_ports()
            .iter()
            .find(|p| p.guest == PORT_MAPPER_PORT)
            .unwrap();
        let mut transport = std::net::TcpStream::connect(("127.0.0.1", port.host)).unwrap();
        let reply = do_ping(&mut transport).unwrap();
        assert_eq!(
            reply,
            Message {
                xid: Xid(1),
                body: MessageBody::Reply(sun_rpc::ReplyBody::Accepted(sun_rpc::AcceptedReply {
                    verifier: OpaqueAuth {
                        flavor: sun_rpc::AuthFlavor::None,
                        body: vec![],
                    },
                    body: sun_rpc::AcceptedReplyBody::Success(()),
                })),
            }
        );
    });
}
