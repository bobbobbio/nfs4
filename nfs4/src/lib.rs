// Copyright 2023 Remi Bernotavicius

use bitflags::bitflags;
use bitflags_serde_shim::impl_serde_for_bitflags;
use derive_more::{From, TryInto};
use enum_as_inner::EnumAsInner;
pub use enum_map::{EnumMap, EnumSet, ToId};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{
    de::Deserializer,
    ser::{SerializeStruct as _, Serializer},
    Deserialize, Serialize,
};
use serde_xdr::opaque_data::fixed_length;
use std::fmt;
use sun_rpc::{AuthFlavor, AuthSysParameters};
use xdr_extras::{DeserializeWithDiscriminant, SerializeWithDiscriminant};

mod enum_map;

pub type FileAttributes = EnumMap<FileAttributeId, FileAttribute>;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CompoundArgs {
    pub tag: String,
    pub minor_version: u32,
    pub arg_array: Vec<ArgOp>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CompoundRes {
    pub status: StatusResult<()>,
    pub tag: String,
    pub res_array: Vec<ResOp>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct Access: u32 {
        const READ      = 0x00000001;
        const LOOKUP    = 0x00000002;
        const MODIFY    = 0x00000004;
        const EXTEND    = 0x00000008;
        const DELETE    = 0x00000010;
        const EXECUTE   = 0x00000020;
    }
}

impl_serde_for_bitflags!(Access);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AccessArgs {
    pub access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct StateId {
    pub sequence_id: u32,
    #[serde(with = "fixed_length")]
    pub other: [u8; 12],
}

impl StateId {
    pub fn anonymous() -> Self {
        Self {
            sequence_id: 0,
            other: [0; 12],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct SequenceId(pub u32);

impl SequenceId {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CloseArgs {
    pub sequence_id: SequenceId,
    pub open_stateid: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CommitArgs {
    pub offset: u64,
    pub count: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DeviceData {
    pub major: u32,
    pub minor: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RetentionGet {
    pub duration: u64,
    pub begin_time: Option<Time>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RetentionSet {
    pub enable: bool,
    pub duration: Option<u64>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum SetTime {
    SetToClientTime(Time) = 0,
    SetToServerTime = 1,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum FileType {
    Regular = 1,
    Directory = 2,
    Block = 3,
    Character = 4,
    Link = 5,
    Socket = 6,
    Fifo = 7,
    AttrDir = 8,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum CreateType {
    Directory = 2,
    Block = 3,
    Character(DeviceData) = 4,
    Link(String) = 5,
    Socket = 6,
    Fifo = 7,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Copy,
    Clone,
    PartialOrd,
    Ord,
    Debug,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
pub enum FileAttributeId {
    SupportedAttrs = 0,
    Type = 1,
    FhExpireType = 2,
    Change = 3,
    Size = 4,
    LinkSupport = 5,
    SymlinkSupport = 6,
    NamedAttr = 7,
    FsId = 8,
    UniqueHandles = 9,
    LeaseTime = 10,
    ReadDirAttrError = 11,
    Acl = 12,
    AclSupport = 13,
    Archive = 14,
    CanSetTime = 15,
    CaseInsensitive = 16,
    CasePreserving = 17,
    ChownRestricted = 18,
    FileHandle = 19,
    FileId = 20,
    FilesAvail = 21,
    FilesFree = 22,
    FilesTotal = 23,
    FsLocations = 24,
    Homogeneous = 26,
    MaxFileSize = 27,
    MaxLink = 28,
    MaxName = 29,
    MaxRead = 30,
    MaxWrite = 31,
    MimeType = 32,
    Mode = 33,
    NoTrunc = 34,
    NumLinks = 35,
    Owner = 36,
    OwnerGroup = 37,
    QuotaAvailHard = 38,
    QuotaAvailSoft = 39,
    QuotaUsed = 40,
    RawDev = 41,
    SpaceAvail = 42,
    SpaceFree = 43,
    SpaceTotal = 44,
    SpaceUsed = 45,
    System = 46,
    TimeAccess = 47,
    TimeAccessSet = 48,
    TimeBackup = 49,
    TimeCreate = 50,
    TimeDelta = 51,
    TimeMetadata = 52,
    TimeModify = 53,
    TimeModifySet = 54,
    MountedOnFileid = 55,
    DirNotifDelay = 56,
    DirentNotifDelay = 57,
    Dacl = 58,
    Sacl = 59,
    ChangePolicy = 60,
    FsStatus = 61,
    FsLayoutType = 62,
    LayoutHint = 63,
    LayoutType = 64,
    LayoutBlksize = 65,
    LayoutAlignment = 66,
    FsLocationsInfo = 67,
    MdsThreshold = 68,
    RetentionGet = 69,
    RetentionSet = 70,
    RetentevtGet = 71,
    RetentevtSet = 72,
    RetentionHold = 73,
    ModeSetMasked = 74,
    SupportedAttrsExclusiveCreate = 75,
    FsCharsetCap = 76,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct FsId {
    pub major: u64,
    pub minor: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FileHandle(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Identity(pub String);

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
pub enum AceType {
    AccessAllowed = 0,
    AccessDenied = 1,
    SystemAudit = 2,
    SystemAlarm = 3,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct AceFlags: u32 {
        const FILE_INHERIT_ACE             = 0x00000001;
        const DIRECTORY_INHERIT_ACE        = 0x00000002;
        const NO_PROPAGATE_INHERIT_ACE     = 0x00000004;
        const INHERIT_ONLY_ACE             = 0x00000008;
        const SUCCESSFUL_ACCESS_ACE_FLAG   = 0x00000010;
        const FAILED_ACCESS_ACE_FLAG       = 0x00000020;
        const IDENTIFIER_GROUP             = 0x00000040;
        const INHERITED_ACE                = 0x00000080;
    }
}

impl_serde_for_bitflags!(AceFlags);

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct AceMask: u32 {
        const READ_DATA            = 0x00000001;
        const LIST_DIRECTORY       = 0x00000001;
        const WRITE_DATA           = 0x00000002;
        const ADD_FILE             = 0x00000002;
        const APPEND_DATA          = 0x00000004;
        const ADD_SUBDIRECTORY     = 0x00000004;
        const READ_NAMED_ATTRS     = 0x00000008;
        const WRITE_NAMED_ATTRS    = 0x00000010;
        const EXECUTE              = 0x00000020;
        const DELETE_CHILD         = 0x00000040;
        const READ_ATTRIBUTES      = 0x00000080;
        const WRITE_ATTRIBUTES     = 0x00000100;
        const WRITE_RETENTION      = 0x00000200;
        const WRITE_RETENTION_HOLD = 0x00000400;

        const DELETE               = 0x00010000;
        const READ_ACL             = 0x00020000;
        const WRITE_ACL            = 0x00040000;
        const WRITE_OWNER          = 0x00080000;
        const SYNCHRONIZE          = 0x00100000;
    }
}

impl_serde_for_bitflags!(AceMask);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Ace {
    pub type_: AceType,
    pub flags: AceFlags,
    pub access_mask: AceMask,
    pub who: Identity,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Acl {
    pub aces: Vec<Ace>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct AclFlags: u32 {
        const AUTO_INHERIT         = 0x00000001;
        const PROTECTED            = 0x00000002;
        const DEFAULTED            = 0x00000004;
    }
}

impl_serde_for_bitflags!(AclFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AclWithFlags {
    pub flags: AclFlags,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ChangePolicy(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Time {
    pub seconds: i64,
    pub nseconds: u32,
}

#[cfg(feature = "chrono")]
impl Time {
    pub fn to_date_time(&self) -> Option<chrono::NaiveDateTime> {
        chrono::NaiveDateTime::from_timestamp_opt(self.seconds, self.nseconds)
    }
}

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
pub enum LayoutType {
    NfsV41Files = 1,
    Osd2Objects = 2,
    BlockVolume = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockDenied {
    pub offset: u64,
    pub length: u64,
    pub lock_type: LockType,
    pub owner: StateOwner,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Clone,
    Debug,
    TryFromPrimitive,
)]
#[repr(u32)]
pub enum StatusError {
    Perm = 1,
    NoEnt = 2,
    Io = 5,
    NxIo = 6,
    Access = 13,
    Exist = 17,
    XDev = 18,
    NotDir = 20,
    Isdir = 21,
    Inval = 22,
    FBig = 27,
    NoSpc = 28,
    RoFs = 30,
    MLink = 31,
    NameTooLong = 63,
    NotEmpty = 66,
    DQuot = 69,
    Stale = 70,
    BadHandle = 10001,
    BadCookie = 10003,
    NotSupported = 10004,
    TooSmall = 10005,
    ServerFault = 10006,
    BadType = 10007,
    Delay = 10008,
    Same = 10009,
    Denied = 10010,
    Expired = 10011,
    Locked = 10012,
    Grace = 10013,
    FhExpired = 10014,
    ShareDenied = 10015,
    WrongSec = 10016,
    ClidInUse = 10017,
    Moved = 10019,
    NoFileHandle = 10020,
    MinorVersMismatch = 10021,
    StaleClientId = 10022,
    StaleStateId = 10023,
    OldStateId = 10024,
    BadStateId = 10025,
    BadSeqId = 10026,
    NotSame = 10027,
    LockRange = 10028,
    Symlink = 10029,
    RestoreFh = 10030,
    LeaseMoved = 10031,
    AttrNotSupported = 10032,
    NoGrace = 10033,
    ReclaimBad = 10034,
    ReclaimConflict = 10035,
    BadXdr = 10036,
    LocksHeld = 10037,
    OpenMode = 10038,
    BadOwner = 10039,
    BadChar = 10040,
    BadName = 10041,
    BadRange = 10042,
    LockNotSupported = 10043,
    OpIllegal = 10044,
    Deadlock = 10045,
    FileOpen = 10046,
    AdminRevoked = 10047,
    CbPathDown = 10048,
    BadIoMode = 10049,
    BadLayout = 10050,
    BadSessionDigest = 10051,
    BadSession = 10052,
    BadSlot = 10053,
    CompleteAlready = 10054,
    ConnNotBoundToSession = 10055,
    DelegAlreadyWanted = 10056,
    BackChanBusy = 10057,
    LayoutTryLater = 10058,
    LayoutUnavailable = 10059,
    NoMatchingLayout = 10060,
    RecallConflict = 10061,
    UnknownLayoutType = 10062,
    SeqMisordered = 10063,
    SequencePos = 10064,
    ReqTooBig = 10065,
    RepTooBig = 10066,
    RepTooBigToCache = 10067,
    RetryUncachedRep = 10068,
    UnsafeCompound = 10069,
    TooManyOps = 10070,
    OpNotInSession = 10071,
    HashAlgUnsupported = 10072,
    ClientIdBusy = 10074,
    PnfsIoHole = 10075,
    SeqFalseRetry = 10076,
    BadHighSlot = 10077,
    DeadSession = 10078,
    EncrAlgUnsupported = 10079,
    PnfsNoLayout = 10080,
    NotOnlyOp = 10081,
    WrongCred = 10082,
    WrongType = 10083,
    DirDelegUnavail = 10084,
    RejectDeleg = 10085,
    ReturnConflict = 10086,
    DelegRevoked = 10087,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Component(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PathName(Vec<Component>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsLocation {
    pub server: Vec<String>,
    pub root_path: PathName,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsLocations {
    pub fs_root: PathName,
    pub locations: Vec<FsLocation>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct FsLocationsInfoFlags: u32 {
        const VAR_SUB = 0x00000001;
    }
}

impl_serde_for_bitflags!(FsLocationsInfoFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsLocationsServer {
    pub currency: i32,
    #[serde(with = "serde_bytes")]
    pub info: Vec<u8>,
    pub server: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsLocationsItem {
    pub entries: Vec<FsLocationsServer>,
    pub root_path: PathName,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsLocationsInfo {
    pub flags: FsLocationsInfoFlags,
    pub valid_for: i32,
    pub fs_root: PathName,
    pub items: Vec<FsLocationsItem>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum FsStatusType {
    Fixed = 1,
    Updated = 2,
    Versioned = 3,
    Writable = 4,
    Referral = 5,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FsStatus {
    pub absent: bool,
    pub type_: FsStatusType,
    pub source: String,
    pub current: String,
    pub age: i32,
    pub version: Time,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutHint {
    pub type_: LayoutType,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Copy,
    Clone,
    PartialOrd,
    Ord,
    Debug,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
pub enum ThresholdAttributeId {
    ReadSize = 0,
    WriteSize = 1,
    ReadIoSize = 2,
    WriteIoSize = 3,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum ThresholdAttribute {
    ReadSize(u32) = ThresholdAttributeId::ReadSize as u32,
    WriteSize(u32) = ThresholdAttributeId::WriteSize as u32,
    ReadIoSize(u32) = ThresholdAttributeId::ReadIoSize as u32,
    WriteIoSize(u32) = ThresholdAttributeId::WriteIoSize as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ThresholdItem {
    pub layout_type: LayoutType,
    pub hintset: EnumMap<ThresholdAttributeId, ThresholdAttribute>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct MdsThreshold {
    hints: Vec<ThresholdItem>,
}

struct OctalFmt<T>(T);

impl<T> fmt::Debug for OctalFmt<T>
where
    T: fmt::Octal,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub struct Mode(pub u32);

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Mode").field(&OctalFmt(self.0)).finish()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub struct ModeMasked(pub u32);

impl fmt::Debug for ModeMasked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Mode").field(&OctalFmt(self.0)).finish()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StatusResult<T> {
    Ok(T),
    Err(StatusError),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LockStatusError {
    pub error: StatusError,
    pub denied: Option<LockDenied>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum LockStatusResult<T> {
    Ok(T),
    Err(LockStatusError),
}

impl<T> Serialize for StatusResult<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok(v) => {
                let mut state = serializer.serialize_struct("StatusResult", 2)?;
                state.serialize_field("discriminant", &0u32)?;
                state.serialize_field("ok", v)?;
                state.end()
            }
            Self::Err(e) => e.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for StatusResult<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = StatusResult<T>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StatusResult")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let disc: u32 = seq
                    .next_element()?
                    .ok_or(serde::de::Error::custom("expected discriminant"))?;
                if disc == 0 {
                    Ok(StatusResult::Ok(
                        seq.next_element()?
                            .ok_or(serde::de::Error::custom("expected value"))?,
                    ))
                } else {
                    let err_id: StatusError = disc.try_into().map_err(|_| {
                        serde::de::Error::custom(format!(
                            "unexpected value {disc:?} for StatusError"
                        ))
                    })?;
                    Ok(StatusResult::Err(err_id))
                }
            }
        }

        deserializer.deserialize_struct(
            "StatusResult",
            &["disc", "value"],
            Visitor(std::marker::PhantomData),
        )
    }
}

impl<T> Serialize for LockStatusResult<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok(v) => {
                let mut state = serializer.serialize_struct("LockStatusResult", 2)?;
                state.serialize_field("discriminant", &0u32)?;
                state.serialize_field("ok", v)?;
                state.end()
            }
            Self::Err(LockStatusError { error, denied }) => {
                let mut state = serializer.serialize_struct("LockStatusResult", 2)?;
                state.serialize_field("error", error)?;
                if error == &StatusError::Denied {
                    state.serialize_field("denied", denied.as_ref().unwrap())?;
                }
                state.end()
            }
        }
    }
}

impl<'de, T> Deserialize<'de> for LockStatusResult<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = LockStatusResult<T>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("LockStatusResult")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let disc: u32 = seq
                    .next_element()?
                    .ok_or(serde::de::Error::custom("expected discriminant"))?;
                if disc == 0 {
                    Ok(LockStatusResult::Ok(
                        seq.next_element()?
                            .ok_or(serde::de::Error::custom("expected value"))?,
                    ))
                } else {
                    let error: StatusError = disc.try_into().map_err(|_| {
                        serde::de::Error::custom(format!(
                            "unexpected value {disc:?} for StatusError"
                        ))
                    })?;
                    let denied = (error == StatusError::Denied)
                        .then(|| {
                            seq.next_element()?
                                .ok_or(serde::de::Error::custom("expected denied"))
                        })
                        .transpose()?;

                    Ok(LockStatusResult::Err(LockStatusError { error, denied }))
                }
            }
        }

        deserializer.deserialize_struct(
            "LockStatusResult",
            &["disc", "value", "denied"],
            Visitor(std::marker::PhantomData),
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Lease(pub u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Change(pub u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct FileId(pub u64);

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    EnumAsInner,
    PartialEq,
    Eq,
    Clone,
    Debug,
    TryInto,
)]
#[try_into(owned, ref)]
#[repr(u32)]
pub enum FileAttribute {
    SupportedAttrs(EnumSet<FileAttributeId>) = FileAttributeId::SupportedAttrs as u32,
    Type(FileType) = FileAttributeId::Type as u32,
    FhExpireType(u32) = FileAttributeId::FhExpireType as u32,
    Change(Change) = FileAttributeId::Change as u32,
    Size(u64) = FileAttributeId::Size as u32,
    LinkSupport(bool) = FileAttributeId::LinkSupport as u32,
    SymlinkSupport(bool) = FileAttributeId::SymlinkSupport as u32,
    NamedAttr(bool) = FileAttributeId::NamedAttr as u32,
    FsId(FsId) = FileAttributeId::FsId as u32,
    UniqueHandles(bool) = FileAttributeId::UniqueHandles as u32,
    LeaseTime(Lease) = FileAttributeId::LeaseTime as u32,
    ReadDirAttrError(StatusResult<()>) = FileAttributeId::ReadDirAttrError as u32,
    Acl(Acl) = FileAttributeId::Acl as u32,
    AclSupport(u32) = FileAttributeId::AclSupport as u32,
    Archive(bool) = FileAttributeId::Archive as u32,
    CanSetTime(bool) = FileAttributeId::CanSetTime as u32,
    CaseInsensitive(bool) = FileAttributeId::CaseInsensitive as u32,
    CasePreserving(bool) = FileAttributeId::CasePreserving as u32,
    ChownRestricted(bool) = FileAttributeId::ChownRestricted as u32,
    FileHandle(FileHandle) = FileAttributeId::FileHandle as u32,
    FileId(FileId) = FileAttributeId::FileId as u32,
    FilesAvail(u64) = FileAttributeId::FilesAvail as u32,
    FilesFree(u64) = FileAttributeId::FilesFree as u32,
    FilesTotal(u64) = FileAttributeId::FilesTotal as u32,
    FsLocations(FsLocations) = FileAttributeId::FsLocations as u32,
    Homogeneous(bool) = FileAttributeId::Homogeneous as u32,
    MaxFileSize(u64) = FileAttributeId::MaxFileSize as u32,
    MaxLink(u32) = FileAttributeId::MaxLink as u32,
    MaxName(u32) = FileAttributeId::MaxName as u32,
    MaxRead(u64) = FileAttributeId::MaxRead as u32,
    MaxWrite(u64) = FileAttributeId::MaxWrite as u32,
    MimeType(String) = FileAttributeId::MimeType as u32,
    Mode(Mode) = FileAttributeId::Mode as u32,
    NoTrunc(bool) = FileAttributeId::NoTrunc as u32,
    NumLinks(u32) = FileAttributeId::NumLinks as u32,
    Owner(String) = FileAttributeId::Owner as u32,
    OwnerGroup(String) = FileAttributeId::OwnerGroup as u32,
    QuotaAvailHard(u64) = FileAttributeId::QuotaAvailHard as u32,
    QuotaAvailSoft(u64) = FileAttributeId::QuotaAvailSoft as u32,
    QuotaUsed(u64) = FileAttributeId::QuotaUsed as u32,
    RawDev(DeviceData) = FileAttributeId::RawDev as u32,
    SpaceAvail(u64) = FileAttributeId::SpaceAvail as u32,
    SpaceFree(u64) = FileAttributeId::SpaceFree as u32,
    SpaceTotal(u64) = FileAttributeId::SpaceTotal as u32,
    SpaceUsed(u64) = FileAttributeId::SpaceUsed as u32,
    System(bool) = FileAttributeId::System as u32,
    TimeAccess(Time) = FileAttributeId::TimeAccess as u32,
    TimeAccessSet(SetTime) = FileAttributeId::TimeAccessSet as u32,
    TimeBackup(Time) = FileAttributeId::TimeBackup as u32,
    TimeCreate(Time) = FileAttributeId::TimeCreate as u32,
    TimeDelta(Time) = FileAttributeId::TimeDelta as u32,
    TimeMetadata(Time) = FileAttributeId::TimeMetadata as u32,
    TimeModify(Time) = FileAttributeId::TimeModify as u32,
    TimeModifySet(SetTime) = FileAttributeId::TimeModifySet as u32,
    MountedOnFileid(FileId) = FileAttributeId::MountedOnFileid as u32,
    DirNotifDelay(Time) = FileAttributeId::DirNotifDelay as u32,
    DirentNotifDelay(Time) = FileAttributeId::DirentNotifDelay as u32,
    Dacl(AclWithFlags) = FileAttributeId::Dacl as u32,
    Sacl(AclWithFlags) = FileAttributeId::Sacl as u32,
    ChangePolicy(ChangePolicy) = FileAttributeId::ChangePolicy as u32,
    FsStatus(FsStatus) = FileAttributeId::FsStatus as u32,
    FsLayoutType(Vec<LayoutType>) = FileAttributeId::FsLayoutType as u32,
    LayoutHint(LayoutHint) = FileAttributeId::LayoutHint as u32,
    LayoutType(Vec<LayoutType>) = FileAttributeId::LayoutType as u32,
    LayoutBlksize(u32) = FileAttributeId::LayoutBlksize as u32,
    LayoutAlignment(u32) = FileAttributeId::LayoutAlignment as u32,
    FsLocationsInfo(FsLocationsInfo) = FileAttributeId::FsLocationsInfo as u32,
    MdsThreshold(MdsThreshold) = FileAttributeId::MdsThreshold as u32,
    RetentionGet(RetentionGet) = FileAttributeId::RetentionGet as u32,
    RetentionSet(RetentionSet) = FileAttributeId::RetentionSet as u32,
    RetentevtGet(RetentionGet) = FileAttributeId::RetentevtGet as u32,
    RetentevtSet(RetentionSet) = FileAttributeId::RetentevtSet as u32,
    RetentionHold(u64) = FileAttributeId::RetentionHold as u32,
    ModeSetMasked(ModeMasked) = FileAttributeId::ModeSetMasked as u32,
    SupportedAttrsExclusiveCreate(EnumSet<FileAttributeId>) =
        FileAttributeId::SupportedAttrsExclusiveCreate as u32,
    FsCharsetCap(u32) = FileAttributeId::FsCharsetCap as u32,
}

impl ToId<FileAttributeId> for FileAttribute {
    fn to_id(&self) -> FileAttributeId {
        match self {
            Self::SupportedAttrs(..) => FileAttributeId::SupportedAttrs,
            Self::Type(..) => FileAttributeId::Type,
            Self::FhExpireType(..) => FileAttributeId::FhExpireType,
            Self::Change(..) => FileAttributeId::Change,
            Self::Size(..) => FileAttributeId::Size,
            Self::LinkSupport(..) => FileAttributeId::LinkSupport,
            Self::SymlinkSupport(..) => FileAttributeId::SymlinkSupport,
            Self::NamedAttr(..) => FileAttributeId::NamedAttr,
            Self::FsId(..) => FileAttributeId::FsId,
            Self::UniqueHandles(..) => FileAttributeId::UniqueHandles,
            Self::LeaseTime(..) => FileAttributeId::LeaseTime,
            Self::ReadDirAttrError(..) => FileAttributeId::ReadDirAttrError,
            Self::Acl(..) => FileAttributeId::Acl,
            Self::AclSupport(..) => FileAttributeId::AclSupport,
            Self::Archive(..) => FileAttributeId::Archive,
            Self::CanSetTime(..) => FileAttributeId::CanSetTime,
            Self::CaseInsensitive(..) => FileAttributeId::CaseInsensitive,
            Self::CasePreserving(..) => FileAttributeId::CasePreserving,
            Self::ChownRestricted(..) => FileAttributeId::ChownRestricted,
            Self::FileHandle(..) => FileAttributeId::FileHandle,
            Self::FileId(..) => FileAttributeId::FileId,
            Self::FilesAvail(..) => FileAttributeId::FilesAvail,
            Self::FilesFree(..) => FileAttributeId::FilesFree,
            Self::FilesTotal(..) => FileAttributeId::FilesTotal,
            Self::FsLocations(..) => FileAttributeId::FsLocations,
            Self::Homogeneous(..) => FileAttributeId::Homogeneous,
            Self::MaxFileSize(..) => FileAttributeId::MaxFileSize,
            Self::MaxLink(..) => FileAttributeId::MaxLink,
            Self::MaxName(..) => FileAttributeId::MaxName,
            Self::MaxRead(..) => FileAttributeId::MaxRead,
            Self::MaxWrite(..) => FileAttributeId::MaxWrite,
            Self::MimeType(..) => FileAttributeId::MimeType,
            Self::Mode(..) => FileAttributeId::Mode,
            Self::NoTrunc(..) => FileAttributeId::NoTrunc,
            Self::NumLinks(..) => FileAttributeId::NumLinks,
            Self::Owner(..) => FileAttributeId::Owner,
            Self::OwnerGroup(..) => FileAttributeId::OwnerGroup,
            Self::QuotaAvailHard(..) => FileAttributeId::QuotaAvailHard,
            Self::QuotaAvailSoft(..) => FileAttributeId::QuotaAvailSoft,
            Self::QuotaUsed(..) => FileAttributeId::QuotaUsed,
            Self::RawDev(..) => FileAttributeId::RawDev,
            Self::SpaceAvail(..) => FileAttributeId::SpaceAvail,
            Self::SpaceFree(..) => FileAttributeId::SpaceFree,
            Self::SpaceTotal(..) => FileAttributeId::SpaceTotal,
            Self::SpaceUsed(..) => FileAttributeId::SpaceUsed,
            Self::System(..) => FileAttributeId::System,
            Self::TimeAccess(..) => FileAttributeId::TimeAccess,
            Self::TimeAccessSet(..) => FileAttributeId::TimeAccessSet,
            Self::TimeBackup(..) => FileAttributeId::TimeBackup,
            Self::TimeCreate(..) => FileAttributeId::TimeCreate,
            Self::TimeDelta(..) => FileAttributeId::TimeDelta,
            Self::TimeMetadata(..) => FileAttributeId::TimeMetadata,
            Self::TimeModify(..) => FileAttributeId::TimeModify,
            Self::TimeModifySet(..) => FileAttributeId::TimeModifySet,
            Self::MountedOnFileid(..) => FileAttributeId::MountedOnFileid,
            Self::DirNotifDelay(..) => FileAttributeId::DirNotifDelay,
            Self::DirentNotifDelay(..) => FileAttributeId::DirentNotifDelay,
            Self::Dacl(..) => FileAttributeId::Dacl,
            Self::Sacl(..) => FileAttributeId::Sacl,
            Self::ChangePolicy(..) => FileAttributeId::ChangePolicy,
            Self::FsStatus(..) => FileAttributeId::FsStatus,
            Self::FsLayoutType(..) => FileAttributeId::FsLayoutType,
            Self::LayoutHint(..) => FileAttributeId::LayoutHint,
            Self::LayoutType(..) => FileAttributeId::LayoutType,
            Self::LayoutBlksize(..) => FileAttributeId::LayoutBlksize,
            Self::LayoutAlignment(..) => FileAttributeId::LayoutAlignment,
            Self::FsLocationsInfo(..) => FileAttributeId::FsLocationsInfo,
            Self::MdsThreshold(..) => FileAttributeId::MdsThreshold,
            Self::RetentionGet(..) => FileAttributeId::RetentionGet,
            Self::RetentionSet(..) => FileAttributeId::RetentionSet,
            Self::RetentevtGet(..) => FileAttributeId::RetentevtGet,
            Self::RetentevtSet(..) => FileAttributeId::RetentevtSet,
            Self::RetentionHold(..) => FileAttributeId::RetentionHold,
            Self::ModeSetMasked(..) => FileAttributeId::ModeSetMasked,
            Self::SupportedAttrsExclusiveCreate(..) => {
                FileAttributeId::SupportedAttrsExclusiveCreate
            }
            Self::FsCharsetCap(..) => FileAttributeId::FsCharsetCap,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateArgs {
    pub object_type: CreateType,
    pub object_name: String,
    pub create_attrs: FileAttributes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct ClientId(pub u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DelegPurgeArgs {
    pub client_id: ClientId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DelegReturnArgs {
    pub state_id: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetAttrArgs {
    pub attr_request: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LinkArgs {
    pub new_name: String,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum LockType {
    Read = 1,
    Write = 2,
    BlockingRead = 3,
    BlockingWrite = 4,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenToLockOwner {
    open_sequence_id: SequenceId,
    open_state_id: StateId,
    lock_sequence_id: SequenceId,
    lock_owner: StateOwner,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ExistingLockOwner {
    lock_state_id: StateId,
    lock_sequence_id: SequenceId,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u8)]
pub enum Locker {
    NewLockOwner(OpenToLockOwner) = 1,
    ExistingLockOwner(ExistingLockOwner) = 0,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockArgs {
    pub lock_type: LockType,
    pub reclaim: bool,
    pub offset: u64,
    pub length: u64,
    pub locker: Locker,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct StateOwner {
    pub client_id: ClientId,
    #[serde(with = "serde_bytes")]
    pub opaque: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockTArgs {
    pub lock_type: LockType,
    pub offset: u64,
    pub length: u64,
    pub owner: StateOwner,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockUArgs {
    pub lock_type: LockType,
    pub sequence_id: SequenceId,
    pub lock_state_id: StateId,
    pub offset: u64,
    pub length: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LookUpArgs {
    pub object_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NVerifyArgs {
    pub object_attributes: FileAttributes,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct ShareAccess: u32 {
        const READ                               = 0x00000001;
        const WRITE                              = 0x00000002;
        const BOTH                               = 0x00000003;
        const WANT_DELEG_MASK                    = 0xFF00;
        const WANT_NO_PREFERENCE                 = 0x0000;
        const WANT_READ_DELEG                    = 0x0100;
        const WANT_WRITE_DELEG                   = 0x0200;
        const WANT_ANY_DELEG                     = 0x0300;
        const WANT_NO_DELEG                      = 0x0400;
        const WANT_CANCEL                        = 0x0500;
        const WANT_SIGNAL_DELEG_WHEN_RESRC_AVAIL = 0x10000;
        const WANT_PUSH_DELEG_WHEN_UNCONTENDED   = 0x20000;
    }
}

impl_serde_for_bitflags!(ShareAccess);

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct ShareDeny: u32 {
        const NONE     = 0x00000000;
        const READ     = 0x00000001;
        const WRITE    = 0x00000002;
        const BOTH     = 0x00000003;
    }
}

impl_serde_for_bitflags!(ShareDeny);

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum OpenFlag {
    OpenNoCreate = 0,
    OpenCreate(CreateHow) = 1,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum OpenDelegationType {
    None = 0,
    Read = 1,
    Write = 2,
    NoneExt = 3,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum CreateHow {
    Unchecked = 0,
    Guarded {
        create_attrs: FileAttributes,
    } = 1,
    Exclusive {
        create_verifier: Verifier,
    } = 2,
    ExclusiveBoth {
        create_verifier: Verifier,
        create_attrs: FileAttributes,
    } = 3,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum OpenClaim {
    Null {
        file: String,
    } = 0,
    Previous {
        delegate_type: OpenDelegationType,
    } = 1,
    DelegateCurrent {
        delegate_current_info: OpenClaimDelegateCurrent,
    } = 2,
    DelegatePrevious {
        file_delegate_previous: String,
    } = 3,
    Fh = 4,
    DelegateCurrentFh {
        oc_delegate_state_id: StateId,
    } = 5,
    PreviousFh = 6,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenClaimDelegateCurrent {
    pub delegate_stateid: StateId,
    pub file: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenArgs {
    pub sequence_id: SequenceId,
    pub share_access: ShareAccess,
    pub share_deny: ShareDeny,
    pub owner: StateOwner,
    pub open_how: OpenFlag,
    pub claim: OpenClaim,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenAttrArgs {
    pub create_dir: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenDowngradeArgs {
    pub open_state_id: StateId,
    pub sequence_id: SequenceId,
    pub share_access: ShareAccess,
    pub share_deny: ShareDeny,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PutFhArgs {
    pub object: FileHandle,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReadArgs {
    pub state_id: StateId,
    pub offset: u64,
    pub count: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Cookie(pub u64);

impl Cookie {
    pub const fn initial() -> Self {
        Self(0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Verifier(pub u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReadDirArgs {
    pub cookie: Cookie,
    pub cookie_verifier: Verifier,
    pub directory_count: u32,
    pub max_count: u32,
    pub attr_request: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RemoveArgs {
    pub target: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RenameArgs {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SecInfoArgs {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetAttrArgs {
    pub state_id: StateId,
    pub object_attributes: FileAttributes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct VerifyArgs {
    pub object_attributes: FileAttributes,
}

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
pub enum StableHow {
    Unstable = 0,
    DataSync = 1,
    FileSync = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct WriteArgs {
    pub state_id: StateId,
    pub offset: u64,
    pub stable: StableHow,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum RpcGssService {
    None = 1,
    Integrity = 2,
    Privacy = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GssHandle(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GssCallbackHandles {
    service: RpcGssService,
    handle_from_server: GssHandle,
    handle_from_client: GssHandle,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum CallbackSecurityParameters {
    None = AuthFlavor::None as u32,
    Sys(AuthSysParameters) = AuthFlavor::Sys as u32,
    RpcSecGss(GssCallbackHandles) = AuthFlavor::RpcSecGss as u32,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum ChannelDirectionFromServer {
    Fore = 1,
    Back = 2,
    Both = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BackchannelCtlArgs {
    pub cp_program: u32,
    pub security_parameters: Vec<CallbackSecurityParameters>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct SessionId(#[serde(with = "fixed_length")] pub [u8; 16]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BindConnToSessionArgs {
    pub session_id: SessionId,
    pub direction: ChannelDirectionFromServer,
    pub use_connection_in_rdma_mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ClientOwner {
    pub verifier: Verifier,
    #[serde(with = "serde_bytes")]
    pub owner_id: Vec<u8>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct ExchangeIdFlags: u32 {
        const SUPP_MOVED_REFER    = 0x00000001;
        const SUPP_MOVED_MIGR     = 0x00000002;
        const BIND_PRINC_STATEID  = 0x00000100;
        const USE_NON_PNFS        = 0x00010000;
        const USE_PNFS_MDS        = 0x00020000;
        const USE_PNFS_DS         = 0x00040000;
        const MASK_PNFS           = 0x00070000;
        const UPD_CONFIRMED_REC_A = 0x40000000;
        const CONFIRMED_R         = 0x80000000;
    }
}

impl_serde_for_bitflags!(ExchangeIdFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct StateProtectOps {
    pub must_enforce: EnumSet<OperationId>,
    pub must_allow: EnumSet<OperationId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SsvProtInfo {
    pub ops: StateProtectOps,
    pub hash_algorithm: u32,
    pub encryption_algorithm: u32,
    pub ssv_length: u32,
    pub window: u32,
    pub handles: Vec<GssHandle>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SecOid(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SsvStateProtectParams {
    pub ops: StateProtectOps,
    pub hash_algorithms: Vec<SecOid>,
    pub encryption_algorithms: Vec<SecOid>,
    pub window: u32,
    pub num_gss_handles: u32,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum StateProtect {
    None = 0,
    MachCred(StateProtectOps) = 1,
    Ssv(SsvStateProtectParams) = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ImplId {
    pub domain: String,
    pub name: String,
    pub date: Time,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ExchangeIdArgs {
    pub client_owner: ClientOwner,
    pub flags: ExchangeIdFlags,
    pub state_protect: StateProtect,
    pub client_impl_id: Option<ImplId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ChannelAttrs {
    pub header_pad_size: u32,
    pub max_request_size: u32,
    pub max_response_size: u32,
    pub max_response_size_cached: u32,
    pub max_operations: u32,
    pub max_requests: u32,
    pub rdma_ird: Option<u32>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct CreateSessionFlags: u32 {
        const PERSIST              = 0x00000001;
        const CONN_BACK_CHAN       = 0x00000002;
        const CONN_RDMA            = 0x00000004;
    }
}

impl_serde_for_bitflags!(CreateSessionFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateSessionArgs {
    pub client_id: ClientId,
    pub sequence_id: SequenceId,
    pub flags: CreateSessionFlags,
    pub fore_channel_attrs: ChannelAttrs,
    pub back_channel_attrs: ChannelAttrs,
    pub program: u32,
    pub security_parameters: Vec<CallbackSecurityParameters>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DestroySessionArgs {
    pub session_id: SessionId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct FreeStateidArgs {
    pub state_id: StateId,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Copy,
    Clone,
    PartialOrd,
    Ord,
    Debug,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
pub enum NotifyType {
    ChangeChildAttrs = 0,
    ChangeDirAttrs = 1,
    RemoveEntry = 2,
    AddEntry = 3,
    RenameEntry = 4,
    ChangeCookieVerifier = 5,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDirDelegationArgs {
    pub signal_delegation_available: bool,
    pub notification_types: EnumSet<NotifyType>,
    pub child_attr_delay: Time,
    pub dir_attr_delay: Time,
    pub child_attributes: EnumSet<FileAttributeId>,
    pub dir_attributes: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct DeviceId(#[serde(with = "fixed_length")] pub [u8; 16]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Util(pub u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDeviceInfoArgs {
    pub device_id: DeviceId,
    pub util: Util,
    pub first_stripe_index: u32,
    pub pattern_offset: u64,
    pub fh_list: Vec<FileHandle>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDeviceListArgs {
    pub layout_type: LayoutType,
    pub max_devices: u32,
    pub cookie: Cookie,
    pub cookie_verifier: Verifier,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutUpdate {
    pub type_: LayoutType,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutCommitArgs {
    pub offset: u64,
    pub length: u64,
    pub reclaim: bool,
    pub state_id: StateId,
    pub last_write_offset: Option<u64>,
    pub time_modify: Option<Time>,
    pub layout_update: LayoutUpdate,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum LayoutIoMode {
    Read = 1,
    ReadWrite = 2,
    Any = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutGetArgs {
    pub signal_layout_available: bool,
    pub layout_type: LayoutType,
    pub io_mode: LayoutIoMode,
    pub offset: u64,
    pub length: u64,
    pub min_length: u64,
    pub state_id: StateId,
    pub max_count: u32,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum LayoutReturnType {
    File = 1,
    FsId = 2,
    All = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutReturnFile {
    pub offset: u64,
    pub length: u64,
    pub state_id: StateId,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum LayoutReturn {
    File(LayoutReturnFile) = LayoutReturnType::File as u32,
    FsId = LayoutReturnType::FsId as u32,
    All = LayoutReturnType::All as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutReturnArgs {
    pub reclaim: bool,
    pub layout_type: LayoutType,
    pub io_mode: LayoutIoMode,
    pub layout_return: LayoutReturn,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum SecInfoStyle {
    CurrentFh = 0,
    Parent = 1,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SecInfoNoNameArgs {
    pub style: SecInfoStyle,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct SlotId(pub u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SequenceArgs {
    pub session_id: SessionId,
    pub sequence_id: SequenceId,
    pub slot_id: SlotId,
    pub highest_slot_id: SlotId,
    pub cache_this: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetSsvArgs {
    #[serde(with = "serde_bytes")]
    pub ssv: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub digest: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct TestStateIdArgs {
    pub state_ids: Vec<StateId>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum DelegationClaim {
    Null = 0,
    Previous { delegate_type: OpenDelegationType } = 1,
    DelegationCurrent = 2,
    DelegationPrevious = 3,
    Fh = 4,
    DelegationCurrentFh = 5,
    DelefationPreviousFh = 6,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct WantDelegationArgs {
    pub want: ShareAccess,
    pub claim: DelegationClaim,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DestroyClientIdArgs {
    pub client_id: ClientId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReclaimCompleteArgs {
    pub one_fs: bool,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Copy,
    Clone,
    Debug,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
pub enum OperationId {
    Access = 3,
    Close = 4,
    Commit = 5,
    Create = 6,
    DelegPurge = 7,
    DelegReturn = 8,
    GetAttr = 9,
    GetFh = 10,
    Link = 11,
    Lock = 12,
    LockT = 13,
    LockU = 14,
    LookUp = 15,
    LookUpP = 16,
    NVerify = 17,
    Open = 18,
    OpenAttr = 19,
    OpenDowngrade = 21,
    PutFh = 22,
    PutPubFh = 23,
    PutRootFh = 24,
    Read = 25,
    ReadDir = 26,
    ReadLink = 27,
    Remove = 28,
    Rename = 29,
    RestoreFh = 31,
    SaveFh = 32,
    SecInfo = 33,
    SetAttr = 34,
    Verify = 37,
    Write = 38,
    BackchannelCtl = 40,
    BindConnToSession = 41,
    ExchangeId = 42,
    CreateSession = 43,
    DestroySession = 44,
    FreeStateid = 45,
    GetDirDelegation = 46,
    GetDeviceInfo = 47,
    GetDeviceList = 48,
    LayoutCommit = 49,
    LayoutGet = 50,
    LayoutReturn = 51,
    SecInfoNoName = 52,
    Sequence = 53,
    SetSsv = 54,
    TestStateId = 55,
    WantDelegation = 56,
    DestroyClientId = 57,
    ReclaimComplete = 58,
}

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, From, PartialEq, Eq, Clone, Debug,
)]
#[repr(u32)]
pub enum ArgOp {
    Access(AccessArgs) = OperationId::Access as u32,
    Close(CloseArgs) = OperationId::Close as u32,
    Commit(CommitArgs) = OperationId::Commit as u32,
    Create(CreateArgs) = OperationId::Create as u32,
    DelegPurge(DelegPurgeArgs) = OperationId::DelegPurge as u32,
    DelegReturn(DelegReturnArgs) = OperationId::DelegReturn as u32,
    GetAttr(GetAttrArgs) = OperationId::GetAttr as u32,
    GetFh = OperationId::GetFh as u32,
    Link(LinkArgs) = OperationId::Link as u32,
    Lock(LockArgs) = OperationId::Lock as u32,
    LockT(LockTArgs) = OperationId::LockT as u32,
    LockU(LockUArgs) = OperationId::LockU as u32,
    LookUp(LookUpArgs) = OperationId::LookUp as u32,
    LookUpP = OperationId::LookUpP as u32,
    NVerify(NVerifyArgs) = OperationId::NVerify as u32,
    Open(OpenArgs) = OperationId::Open as u32,
    OpenAttr(OpenAttrArgs) = OperationId::OpenAttr as u32,
    OpenDowngrade(OpenDowngradeArgs) = OperationId::OpenDowngrade as u32,
    PutFh(PutFhArgs) = OperationId::PutFh as u32,
    PutPubFh = OperationId::PutPubFh as u32,
    PutRootFh = OperationId::PutRootFh as u32,
    Read(ReadArgs) = OperationId::Read as u32,
    ReadDir(ReadDirArgs) = OperationId::ReadDir as u32,
    ReadLink = OperationId::ReadLink as u32,
    Remove(RemoveArgs) = OperationId::Remove as u32,
    Rename(RenameArgs) = OperationId::Rename as u32,
    RestoreFh = OperationId::RestoreFh as u32,
    SaveFh = OperationId::SaveFh as u32,
    SecInfo(SecInfoArgs) = OperationId::SecInfo as u32,
    SetAttr(SetAttrArgs) = OperationId::SetAttr as u32,
    Verify(VerifyArgs) = OperationId::Verify as u32,
    Write(WriteArgs) = OperationId::Write as u32,
    BackchannelCtl(BackchannelCtlArgs) = OperationId::BackchannelCtl as u32,
    BindConnToSession(BindConnToSessionArgs) = OperationId::BindConnToSession as u32,
    ExchangeId(ExchangeIdArgs) = OperationId::ExchangeId as u32,
    CreateSession(CreateSessionArgs) = OperationId::CreateSession as u32,
    DestroySession(DestroySessionArgs) = OperationId::DestroySession as u32,
    FreeStateid(FreeStateidArgs) = OperationId::FreeStateid as u32,
    GetDirDelegation(GetDirDelegationArgs) = OperationId::GetDirDelegation as u32,
    GetDeviceInfo(GetDeviceInfoArgs) = OperationId::GetDeviceInfo as u32,
    GetDeviceList(GetDeviceListArgs) = OperationId::GetDeviceList as u32,
    LayoutCommit(LayoutCommitArgs) = OperationId::LayoutCommit as u32,
    LayoutGet(LayoutGetArgs) = OperationId::LayoutGet as u32,
    LayoutReturn(LayoutReturnArgs) = OperationId::LayoutReturn as u32,
    SecInfoNoName(SecInfoNoNameArgs) = OperationId::SecInfoNoName as u32,
    Sequence(SequenceArgs) = OperationId::Sequence as u32,
    SetSsv(SetSsvArgs) = OperationId::SetSsv as u32,
    TestStateId(TestStateIdArgs) = OperationId::TestStateId as u32,
    WantDelegation(WantDelegationArgs) = OperationId::WantDelegation as u32,
    DestroyClientId(DestroyClientIdArgs) = OperationId::DestroyClientId as u32,
    ReclaimComplete(ReclaimCompleteArgs) = OperationId::ReclaimComplete as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DirectoryEntry {
    pub cookie: Cookie,
    pub name: String,
    pub attrs: FileAttributes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DirectoryList {
    #[serde(with = "xdr_extras::list")]
    pub entries: Vec<DirectoryEntry>,
    pub eof: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct AccessRes {
    pub supported: Access,
    pub access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CloseRes {
    pub open_state_id: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CommitRes {
    pub write_verifier: Verifier,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct ChangeId(u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ChangeInfo {
    pub atomic: bool,
    pub before: ChangeId,
    pub after: ChangeId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateRes {
    pub change_info: ChangeInfo,
    pub attribute_set: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetAttrRes {
    pub object_attributes: FileAttributes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetFhRes {
    pub object: FileHandle,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LinkRes {
    pub change_info: ChangeInfo,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockRes {
    pub lock_state_id: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LockURes {
    pub lock_state_id: StateId,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct OpenResult: u32 {
        const CONFIRM      = 0x00000002;
        const LOCKTYPE_POSIX = 0x00000004;
        const PRESERVE_UNLINKED = 0x00000008;
        const MAY_NOTIFY_LOCK = 0x00000020;
    }
}

impl_serde_for_bitflags!(OpenResult);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenReadDelegation {
    pub state_id: StateId,
    pub recall: bool,
    pub permissions: Ace,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct ModifiedLimit {
    pub num_blocks: u32,
    pub bytes_per_block: u32,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum SpaceLimit {
    Size { file_size: u64 } = 1,
    Blocks { modified_blocks: ModifiedLimit } = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenWriteDelegation {
    pub state_id: StateId,
    pub recall: bool,
    pub space_limit: SpaceLimit,
    pub permissions: Ace,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum OpenNoneDelegation {
    NotWanted = 0,
    Contention { server_will_push_delegation: bool } = 1,
    Resource { server_will_signal_available: bool } = 2,
    NotSupportedFileType = 3,
    WriteDelegationNotSupportedFileType = 4,
    NotSupportedUpgrade = 5,
    NotSupportedDowngrade = 6,
    Cancelled = 7,
    IsDir = 8,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum OpenDelegation {
    None = OpenDelegationType::None as u32,
    Read { read: OpenReadDelegation } = OpenDelegationType::Read as u32,
    Write { write: OpenWriteDelegation } = OpenDelegationType::Write as u32,
    NoneExt { why_none: OpenNoneDelegation } = OpenDelegationType::NoneExt as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenRes {
    pub state_id: StateId,
    pub change_info: ChangeInfo,
    pub result_flags: OpenResult,
    pub attribute_set: EnumSet<FileAttributeId>,
    pub delegation: OpenDelegation,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct OpenDowngradeRes {
    pub open_state_id: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReadRes {
    pub eof: bool,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReadDirRes {
    pub cookie_verifier: Verifier,
    pub reply: DirectoryList,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ReadLinkRes {
    pub link: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RemoveRes {
    pub change_info: ChangeInfo,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RenameRes {
    pub source_change_info: ChangeInfo,
    pub target_change_info: ChangeInfo,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Qop(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct RpcSecGssInfo {
    pub oid: SecOid,
    pub qop: Qop,
    pub service: RpcGssService,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum SecurityInfo {
    None = AuthFlavor::None as u32,
    Sys = AuthFlavor::Sys as u32,
    RpcSecGss { flavor_info: RpcSecGssInfo } = AuthFlavor::RpcSecGss as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SecInfoRes {
    pub body: Vec<SecurityInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetAttrRes {
    pub attr_set: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetAttrStatusResult {
    pub status: StatusResult<()>,
    pub res: SetAttrRes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct WriteRes {
    pub count: u32,
    pub committed: StableHow,
    pub write_veritifer: Verifier,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct BindConnToSessionRes {
    pub session_id: SessionId,
    pub direction: ChannelDirectionFromServer,
    pub use_connection_in_rdma_mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ServerOwner {
    pub minor_id: u64,
    #[serde(with = "serde_bytes")]
    pub major_id: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ServerScope(#[serde(with = "serde_bytes")] pub Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ExchangeIdRes {
    pub client_id: ClientId,
    pub sequence_id: SequenceId,
    pub flags: ExchangeIdFlags,
    pub state_protect: StateProtect,
    pub server_owner: ServerOwner,
    pub server_scope: ServerScope,
    pub server_impl_id: Option<ImplId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct CreateSessionRes {
    pub session_id: SessionId,
    pub sequence_id: SequenceId,
    pub flags: CreateSessionFlags,
    pub fore_channel_attrs: ChannelAttrs,
    pub back_channel_attrs: ChannelAttrs,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDirDelegationRes {
    pub cookie_verifier: Verifier,
    pub state_id: StateId,
    pub notification: EnumSet<FileAttributeId>,
    pub child_attributes: EnumSet<FileAttributeId>,
    pub dir_attributes: EnumSet<FileAttributeId>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct DeviceAddr {
    pub layout_type: LayoutType,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    PartialEq,
    Eq,
    Copy,
    Clone,
    Debug,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u32)]
pub enum NotifyDeviceIdType {
    Change = 1,
    Delete = 2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDeviceInfoRes {
    pub device_addr: DeviceAddr,
    pub notification: EnumSet<NotifyDeviceIdType>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct GetDeviceListRes {
    pub cookie: Cookie,
    pub cookie_verifier: Verifier,
    pub device_id_list: Vec<DeviceId>,
    pub eof: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutCommitRes {
    pub new_size: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutContent {
    pub type_: LayoutType,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Layout {
    pub offset: u64,
    pub length: u64,
    pub io_mode: LayoutIoMode,
    pub content: LayoutContent,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutGetRes {
    pub return_on_close: bool,
    pub state_id: StateId,
    pub layout: Vec<Layout>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct LayoutReturnRes {
    pub state_id: Option<StateId>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    pub struct SequenceStatusFlags: u32 {
        const CB_PATH_DOWN                  = 0x00000001;
        const CB_GSS_CONTEXTS_EXPIRING      = 0x00000002;
        const CB_GSS_CONTEXTS_EXPIRED       = 0x00000004;
        const EXPIRED_ALL_STATE_REVOKED     = 0x00000008;
        const EXPIRED_SOME_STATE_REVOKED    = 0x00000010;
        const ADMIN_STATE_REVOKED           = 0x00000020;
        const RECALLABLE_STATE_REVOKED      = 0x00000040;
        const LEASE_MOVED                   = 0x00000080;
        const RESTART_RECLAIM_NEEDED        = 0x00000100;
        const CB_PATH_DOWN_SESSION          = 0x00000200;
        const BACKCHANNEL_FAULT             = 0x00000400;
        const DEVID_CHANGED                 = 0x00000800;
        const DEVID_DELETED                 = 0x00001000;
    }
}

impl_serde_for_bitflags!(SequenceStatusFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SequenceRes {
    pub session_id: SessionId,
    pub sequence_id: SequenceId,
    pub slot_id: SlotId,
    pub highest_slot_id: SlotId,
    pub target_highest_slot_id: SlotId,
    pub status_flags: SequenceStatusFlags,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct SetSsvRes {
    #[serde(with = "serde_bytes")]
    pub digest: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct TestStateIdRes {
    pub status_codes: Vec<StatusResult<()>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct WantDelegationRes {
    pub delegation: OpenDelegation,
}

pub type SecInfoNoNameRes = SecInfoRes;

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
pub enum ResOp {
    Access(StatusResult<AccessRes>) = OperationId::Access as u32,
    Close(StatusResult<CloseRes>) = OperationId::Close as u32,
    Commit(StatusResult<CommitRes>) = OperationId::Commit as u32,
    Create(StatusResult<CreateRes>) = OperationId::Create as u32,
    DelegPurge(StatusResult<()>) = OperationId::DelegPurge as u32,
    DelegReturn(StatusResult<()>) = OperationId::DelegReturn as u32,
    GetAttr(StatusResult<GetAttrRes>) = OperationId::GetAttr as u32,
    GetFh(StatusResult<GetFhRes>) = OperationId::GetFh as u32,
    Link(LockStatusResult<LinkRes>) = OperationId::Link as u32,
    Lock(LockStatusResult<LockRes>) = OperationId::Lock as u32,
    LockT(StatusResult<()>) = OperationId::LockT as u32,
    LockU(StatusResult<LockURes>) = OperationId::LockU as u32,
    LookUp(StatusResult<()>) = OperationId::LookUp as u32,
    LookUpP(StatusResult<()>) = OperationId::LookUpP as u32,
    NVerify(StatusResult<()>) = OperationId::NVerify as u32,
    Open(StatusResult<OpenRes>) = OperationId::Open as u32,
    OpenAttr(StatusResult<()>) = OperationId::OpenAttr as u32,
    OpenDowngrade(StatusResult<OpenDowngradeRes>) = OperationId::OpenDowngrade as u32,
    PutFh(StatusResult<()>) = OperationId::PutFh as u32,
    PutPubFh(StatusResult<()>) = OperationId::PutPubFh as u32,
    PutRootFh(StatusResult<()>) = OperationId::PutRootFh as u32,
    Read(StatusResult<ReadRes>) = OperationId::Read as u32,
    ReadDir(StatusResult<ReadDirRes>) = OperationId::ReadDir as u32,
    ReadLink(StatusResult<ReadLinkRes>) = OperationId::ReadLink as u32,
    Remove(StatusResult<RemoveRes>) = OperationId::Remove as u32,
    Rename(StatusResult<RenameRes>) = OperationId::Rename as u32,
    RestoreFh(StatusResult<()>) = OperationId::RestoreFh as u32,
    SaveFh(StatusResult<()>) = OperationId::SaveFh as u32,
    SecInfo(StatusResult<SecInfoRes>) = OperationId::SecInfo as u32,
    SetAttr(SetAttrStatusResult) = OperationId::SetAttr as u32,
    Verify(StatusResult<()>) = OperationId::Verify as u32,
    Write(StatusResult<WriteRes>) = OperationId::Write as u32,
    BackchannelCtl(StatusResult<()>) = OperationId::BackchannelCtl as u32,
    BindConnToSession(StatusResult<BindConnToSessionRes>) = OperationId::BindConnToSession as u32,
    ExchangeId(StatusResult<ExchangeIdRes>) = OperationId::ExchangeId as u32,
    CreateSession(StatusResult<CreateSessionRes>) = OperationId::CreateSession as u32,
    DestroySession(StatusResult<()>) = OperationId::DestroySession as u32,
    FreeStateid(StatusResult<()>) = OperationId::FreeStateid as u32,
    GetDirDelegation(StatusResult<GetDirDelegationRes>) = OperationId::GetDirDelegation as u32,
    GetDeviceInfo(StatusResult<GetDeviceInfoRes>) = OperationId::GetDeviceInfo as u32,
    GetDeviceList(StatusResult<GetDeviceListRes>) = OperationId::GetDeviceList as u32,
    LayoutCommit(StatusResult<LayoutCommitRes>) = OperationId::LayoutCommit as u32,
    LayoutGet(StatusResult<LayoutGetRes>) = OperationId::LayoutGet as u32,
    LayoutReturn(StatusResult<LayoutReturnRes>) = OperationId::LayoutReturn as u32,
    SecInfoNoName(StatusResult<SecInfoRes>) = OperationId::SecInfoNoName as u32,
    Sequence(StatusResult<SequenceRes>) = OperationId::Sequence as u32,
    SetSsv(StatusResult<SetSsvRes>) = OperationId::SetSsv as u32,
    TestStateId(StatusResult<TestStateIdRes>) = OperationId::TestStateId as u32,
    WantDelegation(StatusResult<WantDelegationRes>) = OperationId::WantDelegation as u32,
    DestroyClientId(StatusResult<()>) = OperationId::DestroyClientId as u32,
    ReclaimComplete(StatusResult<()>) = OperationId::ReclaimComplete as u32,
}
