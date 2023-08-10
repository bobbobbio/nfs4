// Copyright 2023 Remi Bernotavicius

use serde::{Deserialize, Serialize};
use xdr_extras::{DeserializeWithDiscriminant, SerializeWithDiscriminant};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Xid(pub u32);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Message<Args> {
    pub xid: Xid,
    pub body: MessageBody<Args>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CallBody<CallArgsT> {
    pub rpc_version: u32,
    pub program: u32,
    pub version: u32,
    pub procedure: u32,
    pub credential: OpaqueAuth,
    pub verifier: OpaqueAuth,
    pub call_args: CallArgsT,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum AcceptedReplyBody<ReturnArgsT> {
    Success(ReturnArgsT) = 0,
    ProgramUnavailable = 1,
    ProgramMismatch { low: u32, high: u32 } = 2,
    ProcedureUnavailable = 3,
    GarbageArguments = 4,
    SystemError = 5,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AcceptedReply<ReturnArgsT> {
    pub verifier: OpaqueAuth,
    pub body: AcceptedReplyBody<ReturnArgsT>,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum AuthFlavor {
    None = 0,
    Sys = 1,
    Short = 2,
    Dh = 3,
    RpcSecGss = 6,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uid(pub u32);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Gid(pub u32);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AuthSysParameters {
    pub stamp: i32,
    pub machine_name: String,
    pub uid: Uid,
    pub gid: Gid,
    pub gids: Vec<Gid>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct OpaqueAuth {
    pub flavor: AuthFlavor,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>, // limit of 400 bytes
}

impl OpaqueAuth {
    pub fn none() -> Self {
        Self {
            flavor: AuthFlavor::None,
            body: vec![],
        }
    }

    pub fn auth_sys(params: AuthSysParameters) -> Self {
        Self {
            flavor: AuthFlavor::Sys,
            body: serde_xdr::to_bytes(&params).unwrap(),
        }
    }
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum AuthStat {
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

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum RejectedReply {
    RpcMismatch { low: u32, high: u32 } = 0,
    AuthError(AuthStat) = 1,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum ReplyBody<ReturnArgsT> {
    Accepted(AcceptedReply<ReturnArgsT>) = 0,
    Denied(RejectedReply) = 1,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
)]
#[repr(u32)]
pub enum MessageBody<Args> {
    Call(CallBody<Args>) = 0,
    Reply(ReplyBody<Args>) = 1,
}
