// Copyright 2023 Remi Bernotavicius

use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{fmt, io};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Xid(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Message<Args> {
    xid: Xid,
    body: MessageBody<Args>,
}

trait Procedure {
    type CallArgs: Serialize
        + for<'a> Deserialize<'a>
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + fmt::Debug;
    type ReturnArgs: Serialize
        + for<'a> Deserialize<'a>
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + fmt::Debug;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CallBody<CallArgsT> {
    rpc_version: u32,
    program: u32,
    version: u32,
    procedure: u32,
    credential: OpaqueAuth,
    verifier: OpaqueAuth,
    call_args: CallArgsT,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum AcceptedReplyBody<ReturnArgsT> {
    Success(ReturnArgsT),
    ProgramMismatch { low: u32, high: u32 },
    Default,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct AcceptedReply<ReturnArgsT> {
    verifier: OpaqueAuth,
    body: AcceptedReplyBody<ReturnArgsT>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u32)]
enum AuthFlavor {
    None = 0,
    Sys = 1,
    Short = 2,
    Dh = 3,
    RpcSecGss = 6,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct OpaqueAuth {
    flavor: AuthFlavor,
    body: Vec<u8>, // limit of 400 bytes
}

impl OpaqueAuth {
    fn none() -> Self {
        Self {
            flavor: AuthFlavor::None,
            body: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum AuthStat {
    Ok = 0, /* success */
    /*
     * failed at remote end
     */
    BadCred = 1,      /* bad credential (seal broken)   */
    RejectedCred = 2, /* client must begin new session  */
    BadVerf = 3,      /* bad verifier (seal broken)     */
    RejectedVerf = 4, /* verifier expired or replayed   */
    TooWeak = 5,      /* rejected for security reasons  */
    /*
     * failed locally
     */
    InvalidResp = 6, /* bogus response verifier        */
    Failed = 7,      /* reason unknown                 */
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum RejectedReply {
    RpcMismatch { low: u32, high: u32 },
    AuthError(AuthStat),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum ReplyBody<ReturnArgsT> {
    Accepted(AcceptedReply<ReturnArgsT>),
    Denied(RejectedReply),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum MessageBody<Args> {
    Call(CallBody<Args>),
    Reply(ReplyBody<Args>),
}

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

pub fn do_ping(mut transport: &mut impl Transport) -> Result<()> {
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

    println!("{reply:#?}");

    Ok(())
}
