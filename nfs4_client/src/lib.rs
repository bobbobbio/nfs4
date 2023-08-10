// Copyright 2023 Remi Bernotavicius

use derive_more::From;
use nfs4::*;
use paste::paste;
use rand::Rng as _;
use std::collections::VecDeque;
use std::io;
use sun_rpc_client::{RpcClient, Transport};

pub type Result<T> = std::result::Result<T, Error>;

pub struct TempResult<T>(Result<T>);

impl<T> From<StatusResult<T>> for TempResult<T> {
    fn from(res: StatusResult<T>) -> Self {
        TempResult(match res {
            StatusResult::Ok(res) => Ok(res),
            StatusResult::Err(e) => Err(e.into()),
        })
    }
}

impl From<SetAttrStatusResult> for TempResult<SetAttrRes> {
    fn from(res: SetAttrStatusResult) -> Self {
        TempResult(match res.status {
            StatusResult::Ok(_) => Ok(res.res),
            StatusResult::Err(e) => Err(e.into()),
        })
    }
}

impl<T> From<LockStatusResult<T>> for TempResult<T> {
    fn from(res: LockStatusResult<T>) -> Self {
        TempResult(match res {
            LockStatusResult::Ok(res) => Ok(res),
            LockStatusResult::Err(e) => Err(e.into()),
        })
    }
}

#[derive(Debug, From)]
pub enum Error {
    SunRpc(sun_rpc_client::Error),
    Protocol(StatusError),
    Lock(LockStatusError),
    Io(std::io::Error),
    #[from(ignore)]
    CompoundResponseMismatch(String),
}

const NFS: u32 = 100003;
const NFS_CB: u32 = 0x40000000;
pub const NFS_PORT: u16 = 2049;
const COMPOUND_PROCEDURE: u32 = 1;

macro_rules! compound_op_impl_ {
    ($name:ident, $args:ident, $res:ty) => {
        impl CompoundRequest for $args {
            type Response = $res;
            type Geometry = ();

            fn into_arg_array(self) -> (Vec<ArgOp>, Self::Geometry) {
                (vec![self.into()], ())
            }

            fn process_reply(
                res_array: &mut VecDeque<ResOp>,
                _geometry: (),
            ) -> Result<Self::Response> {
                let op = res_array
                    .pop_front()
                    .ok_or(Error::CompoundResponseMismatch("too few replies".into()))?;

                if let ResOp::$name(res) = op {
                    TempResult::from(res).0
                } else {
                    Err(Error::CompoundResponseMismatch(format!(
                        "expected {}, got {op:?}",
                        stringify!($name)
                    )))
                }
            }
        }
    };
}

macro_rules! compound_op_impl {
    ($($name:ident)*) => ($(paste! {
        compound_op_impl_! { $name, [<$name Args>], [<$name Res>] }
    })*)
}

macro_rules! compound_op_impl_no_args {
    ($($name:ident)*) => ($(paste! {
        #[derive(Copy, Clone)]
        pub struct $name;

        impl From<$name> for ArgOp {
            fn from(_: $name) -> ArgOp {
                ArgOp::$name
            }
        }

        compound_op_impl_! { $name, $name, [<$name Res>] }
    })*)
}

macro_rules! compound_op_impl_no_args_no_ret {
    ($($name:ident)*) => ($(paste! {
        #[derive(Copy, Clone)]
        pub struct $name;

        impl From<$name> for ArgOp {
            fn from(_: $name) -> ArgOp {
                ArgOp::$name
            }
        }

        compound_op_impl_! { $name, $name, () }
    })*)
}

macro_rules! compound_op_impl_no_ret {
    ($($name:ident)*) => ($(paste! {
        compound_op_impl_! { $name, [<$name Args>], () }
    })*)
}

compound_op_impl! {
    Close
    Commit
    Create
    GetAttr
    Link
    Lock
    LockU
    Open
    OpenDowngrade
    Read
    ReadDir
    Remove
    Rename
    SecInfo
    SetAttr
    Write
    BindConnToSession
    ExchangeId
    CreateSession
    GetDirDelegation
    GetDeviceInfo
    GetDeviceList
    LayoutCommit
    LayoutGet
    LayoutReturn
    SecInfoNoName
    Sequence
    SetSsv
    TestStateId
    WantDelegation
}

compound_op_impl_no_ret! {
    DelegPurge
    DelegReturn
    LockT
    LookUp
    NVerify
    OpenAttr
    PutFh
    Verify
    BackchannelCtl
    DestroySession
    FreeStateid
    DestroyClientId
    ReclaimComplete
}

compound_op_impl_no_args! {
    GetFh
    ReadLink
}

compound_op_impl_no_args_no_ret! {
    PutPubFh
    LookUpP
    PutRootFh
    RestoreFh
    SaveFh
}

trait CompoundRequest {
    type Response;
    type Geometry;

    fn into_arg_array(self) -> (Vec<ArgOp>, Self::Geometry);

    fn process_reply(
        res_array: &mut VecDeque<ResOp>,
        geometry: Self::Geometry,
    ) -> Result<Self::Response>;
}

impl<T> CompoundRequest for Vec<T>
where
    T: CompoundRequest,
{
    type Response = Vec<<T as CompoundRequest>::Response>;
    type Geometry = Vec<<T as CompoundRequest>::Geometry>;

    fn into_arg_array(self) -> (Vec<ArgOp>, Self::Geometry) {
        let mut arg_ret = vec![];
        let mut geo_ret = vec![];
        for e in self {
            let (v, g) = e.into_arg_array();
            arg_ret.extend(v);
            geo_ret.push(g);
        }
        (arg_ret, geo_ret)
    }

    fn process_reply(
        res_array: &mut VecDeque<ResOp>,
        geometry: Self::Geometry,
    ) -> Result<Self::Response> {
        let mut ret = vec![];
        for geo in geometry {
            ret.push(T::process_reply(res_array, geo)?);
        }
        Ok(ret)
    }
}

struct ReturnSecond<A, B>(A, B);

impl<A, B> CompoundRequest for ReturnSecond<A, B>
where
    A: CompoundRequest,
    B: CompoundRequest,
{
    type Response = B::Response;
    type Geometry = (A::Geometry, B::Geometry);

    fn into_arg_array(self) -> (Vec<ArgOp>, Self::Geometry) {
        let (mut a1, g1) = self.0.into_arg_array();
        let (a2, g2) = self.1.into_arg_array();
        a1.extend(a2);
        (a1, (g1, g2))
    }

    fn process_reply(
        res_array: &mut VecDeque<ResOp>,
        geometry: Self::Geometry,
    ) -> Result<Self::Response> {
        A::process_reply(res_array, geometry.0)?;
        Ok(B::process_reply(res_array, geometry.1)?)
    }
}

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {$(paste! {
        impl<$($name),+> CompoundRequest for ($($name),+)
        where
            $($name: CompoundRequest,)+
        {
            type Response = ($(<$name as CompoundRequest>::Response),+);
            type Geometry = ($(<$name as CompoundRequest>::Geometry),+);

            fn into_arg_array(self) -> (Vec<ArgOp>, Self::Geometry) {
                $(let ([<v $n>], [<g $n>]) = self.$n.into_arg_array();)+

                let mut v = vec![];
                $(v.extend([<v $n>]);)+
                (
                    v,
                    ($([<g $n>]),+)
                )
            }

            fn process_reply(
                res_array: &mut VecDeque<ResOp>, geometry: Self::Geometry
            ) -> Result<Self::Response> {
                Ok((
                    $(
                        $name::process_reply(res_array, geometry.$n)?
                    ),+
                ))
            }
        }
    })+}
}

tuple_impls! {
    (0 A 1 B)
    (0 A 1 B 2 C)
    (0 A 1 B 2 C 3 D)
    (0 A 1 B 2 C 3 D 4 E)
    (0 A 1 B 2 C 3 D 4 E 5 F)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L 12 M)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L 12 M 13 N)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L 12 M 13 N 14 O)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L 12 M 13 N 14 O 15 P)
    (0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H 8 I 9 J 10 K 11 L 12 M 13 N 14 O 15 P 16 Q)
}

struct ClientWithoutSession {
    rpc_client: RpcClient,
}

impl ClientWithoutSession {
    fn new(rpc_client: RpcClient) -> Self {
        Self { rpc_client }
    }

    fn do_compound<'a, Args>(
        &mut self,
        mut transport: &mut impl Transport,
        args: Args,
    ) -> Result<Args::Response>
    where
        Args: CompoundRequest,
    {
        let (arg_array, geometry) = args.into_arg_array();
        let call_args = CompoundArgs {
            tag: "Test Client".into(),
            minor_version: 1,
            arg_array,
        };

        self.rpc_client
            .send_request(&mut transport, COMPOUND_PROCEDURE, call_args)?;

        let compound_reply: CompoundRes = self.rpc_client.receive_reply(&mut transport)?;

        if let StatusResult::Err(e) = compound_reply.status {
            return Err(e.into());
        }

        let mut res_array = compound_reply.res_array.into_iter().collect();
        let reply = Args::process_reply(&mut res_array, geometry)?;

        if !res_array.is_empty() {
            return Err(Error::CompoundResponseMismatch(format!(
                "trailing response: {res_array:?}"
            )));
        }

        Ok(reply)
    }
}

fn random_client_owner() -> ClientOwner {
    let mut rng = rand::thread_rng();
    ClientOwner {
        verifier: Verifier(0x0),
        owner_id: rng.gen::<u64>().to_be_bytes().into(),
    }
}

pub struct Client {
    raw_client: ClientWithoutSession,
    session: CreateSessionRes,
    sequence_id: SequenceId,
}

impl Client {
    pub fn new(transport: &mut impl Transport) -> Result<Self> {
        let mut raw_client = ClientWithoutSession::new(RpcClient::new(NFS));

        let eid_res = raw_client.do_compound(
            transport,
            ExchangeIdArgs {
                client_owner: random_client_owner(),
                flags: ExchangeIdFlags::empty(),
                state_protect: StateProtect::None,
                client_impl_id: None,
            },
        )?;

        let client_id = eid_res.client_id;
        let session = raw_client.do_compound(
            transport,
            CreateSessionArgs {
                client_id,
                sequence_id: SequenceId(1),
                flags: CreateSessionFlags::empty(),
                fore_channel_attrs: ChannelAttrs {
                    header_pad_size: 0,
                    max_request_size: 1049620,
                    max_response_size: 1049480,
                    max_response_size_cached: 7584,
                    max_operations: 16,
                    max_requests: 64,
                    rdma_ird: None,
                },
                back_channel_attrs: ChannelAttrs {
                    header_pad_size: 0,
                    max_request_size: 4096,
                    max_response_size: 4096,
                    max_response_size_cached: 0,
                    max_operations: 16,
                    max_requests: 16,
                    rdma_ird: None,
                },
                program: NFS_CB,
                security_parameters: vec![],
            },
        )?;

        Ok(Self {
            raw_client,
            session,
            sequence_id: SequenceId(1),
        })
    }

    fn do_compound<'a, Args>(
        &mut self,
        transport: &mut impl Transport,
        args: Args,
    ) -> Result<Args::Response>
    where
        Args: CompoundRequest,
    {
        let sequence = SequenceArgs {
            session_id: self.session.session_id,
            sequence_id: self.sequence_id,
            slot_id: SlotId(0),
            highest_slot_id: SlotId(0),
            cache_this: false,
        };

        self.sequence_id.incr();

        Ok(self
            .raw_client
            .do_compound(transport, ReturnSecond(sequence, args))?)
    }

    pub fn get_attr(&mut self, transport: &mut impl Transport, path: &str) -> Result<GetAttrRes> {
        let mut get_attr_res = self.do_compound(
            transport,
            ReturnSecond(
                PutRootFh,
                GetAttrArgs {
                    attr_request: [FileAttributeId::SupportedAttrs].into_iter().collect(),
                },
            ),
        )?;

        let mut supported_attrs: EnumSet<FileAttributeId> = get_attr_res
            .object_attributes
            .remove_as(FileAttributeId::SupportedAttrs)
            .unwrap();

        supported_attrs.remove(FileAttributeId::TimeAccessSet);
        supported_attrs.remove(FileAttributeId::TimeModifySet);

        Ok(self.do_compound(
            transport,
            ReturnSecond(
                (
                    PutRootFh,
                    Vec::from_iter(path.split("/").filter_map(|c| {
                        (!c.is_empty()).then_some(LookUpArgs {
                            object_name: c.into(),
                        })
                    })),
                ),
                GetAttrArgs {
                    attr_request: supported_attrs,
                },
            ),
        )?)
    }

    pub fn read(
        &mut self,
        transport: &mut impl Transport,
        handle: FileHandle,
        offset: u64,
        count: u32,
    ) -> Result<ReadRes> {
        Ok(self.do_compound(
            transport,
            ReturnSecond(
                PutFhArgs { object: handle },
                ReadArgs {
                    state_id: StateId::anonymous(),
                    offset,
                    count,
                },
            ),
        )?)
    }

    pub fn read_all(
        &mut self,
        transport: &mut impl Transport,
        handle: FileHandle,
        mut sink: impl io::Write,
    ) -> Result<()> {
        let mut offset = 0;
        loop {
            let read_res = self.read(transport, handle.clone(), offset, 1024 * 1024)?;
            offset += read_res.data.len() as u64;
            sink.write_all(&read_res.data)?;
            if read_res.eof {
                break;
            }
        }
        Ok(())
    }
}