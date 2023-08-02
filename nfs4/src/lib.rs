// Copyright 2023 Remi Bernotavicius

use bitflags::bitflags;
use bitflags_serde_shim::impl_serde_for_bitflags;
use num_enum::TryFromPrimitive;
use serde::{
    de::Deserializer,
    ser::{SerializeStruct as _, Serializer},
    Deserialize, Serialize,
};
use xdr_extras::{DeserializeWithDiscriminant, SerializeWithDiscriminant};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct CompoundArgs {
    tag: String,
    minor_version: u32,
    arg_array: Vec<ArgOp>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    struct Access: u32 {
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
struct AccessArgs {
    access: Access,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct StateId {
    sequence_id: u32,
    other: [u8; 12],
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct SequenceId(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct CloseArgs {
    sequence_id: SequenceId,
    open_stateid: StateId,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct CommitArgs {
    offset: u64,
    count: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct DeviceData {
    major: u32,
    minor: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct RetentionGet {
    duration: u64,
    begin_time: Option<Time>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct RetentionSet {
    enable: bool,
    duration: Option<u64>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum SetTime {
    SetToClientTime(Time) = 0,
    SetToServerTime = 1,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum FileType {
    Link(String) = 0,
    Block = 1,
    Character(DeviceData) = 2,
    Socket = 3,
    Fifo = 4,
    Directory = 5,
}

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
enum AttributeId {
    SupportedAttrs = 0,
    Type = 1,
    FhExpireType = 2,
    Change = 3,
    Size = 4,
    LinkSupport = 5,
    SymlinkSupport = 6,
    NamedAttr = 7,
    Fsid = 8,
    UniqueHandles = 9,
    LeaseTime = 10,
    RdattrError = 11,
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
    Numlinks = 35,
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
    SuppattrExclcreat = 75,
    FsCharsetCap = 76,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct FsId {
    major: u64,
    minor: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FileHandle(#[serde(with = "serde_bytes")] Vec<u8>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct Identity(String);

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
enum AceType {
    AccessAllowed = 0,
    AccessDenied = 1,
    SystemAudit = 2,
    SystemAlarm = 3,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    struct AceFlags: u32 {
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
    struct AceMask: u32 {
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
struct Ace {
    type_: AceType,
    flags: AceFlags,
    access_mask: AceMask,
    who: Identity,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct Acl {
    aces: Vec<Ace>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    struct AclFlags: u32 {
        const AUTO_INHERIT         = 0x00000001;
        const PROTECTED            = 0x00000002;
        const DEFAULTED            = 0x00000004;
    }
}

impl_serde_for_bitflags!(AclFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct AclWithFlags {
    flags: AclFlags,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ChangePolicy(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct Time {
    seconds: i64,
    nseconds: u32,
}

#[derive(
    SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Copy, Clone, Debug,
)]
#[repr(u32)]
enum LayoutType {
    NfsV41Files = 1,
    Osd2Objects = 2,
    BlockVolume = 3,
}

#[derive(
    SerializeWithDiscriminant,
    DeserializeWithDiscriminant,
    TryFromPrimitive,
    PartialEq,
    Eq,
    Clone,
    Debug,
)]
#[repr(u32)]
enum StatusError {
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
    NotSupp = 10004,
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
    AttrNotSupp = 10032,
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
    LockNotSupp = 10043,
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
    HashAlgUnsupp = 10072,
    ClientIdBusy = 10074,
    PnfsIoHole = 10075,
    SeqFalseRetry = 10076,
    BadHighSlot = 10077,
    DeadSession = 10078,
    EncrAlgUnsupp = 10079,
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
struct Component(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct PathName(Vec<Component>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsLocation {
    server: Vec<String>,
    root_path: PathName,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsLocations {
    fs_root: PathName,
    locations: Vec<FsLocation>,
}

bitflags! {
    #[derive(PartialEq, Eq, Copy, Clone, Debug)]
    struct FsLocationsInfoFlags: u32 {
        const VAR_SUB = 0x00000001;
    }
}

impl_serde_for_bitflags!(FsLocationsInfoFlags);

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsLocationsServer {
    currency: i32,
    #[serde(with = "serde_bytes")]
    info: Vec<u8>,
    server: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsLocationsItem {
    entries: Vec<FsLocationsServer>,
    root_path: PathName,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsLocationsInfo {
    flags: FsLocationsInfoFlags,
    valid_for: i32,
    fs_root: PathName,
    items: Vec<FsLocationsItem>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum FsStatusType {
    Fixed = 1,
    Updated = 2,
    Versioned = 3,
    Writable = 4,
    Referral = 5,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FsStatus {
    absent: bool,
    type_: FsStatusType,
    source: String,
    current: String,
    age: i32,
    version: Time,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LayoutHint {
    type_: LayoutType,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum ThresholdAttributeId {
    ReadSize = 0,
    WriteSize = 1,
    ReadIoSize = 2,
    WriteIoSize = 3,
}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum ThresholdAttribute {
    ReadSize(u32) = ThresholdAttributeId::ReadSize as u32,
    WriteSize(u32) = ThresholdAttributeId::WriteSize as u32,
    ReadIoSize(u32) = ThresholdAttributeId::ReadIoSize as u32,
    WriteIoSize(u32) = ThresholdAttributeId::WriteIoSize as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ThresholdItem {
    layout_type: LayoutType,
    hintset: BitMap<ThresholdAttributeId, ThresholdAttribute>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct MdsThreshold {
    hints: Vec<ThresholdItem>,
}

// TODO
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct BitMap<K, V> {
    phantom_data: std::marker::PhantomData<(K, V)>,
    map: Vec<u32>,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct BitSet<K> {
    phantom_data: std::marker::PhantomData<K>,
    map: Vec<u32>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct Mode(u32);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct ModeMasked(u32);

#[derive(PartialEq, Eq, Clone, Debug)]
enum StatusResult<T> {
    Ok(T),
    Err(StatusError),
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
                    Ok(StatusResult::Err(disc.try_into().map_err(|_| {
                        serde::de::Error::custom(format!(
                            "unexpected value {disc:?} for StatusError"
                        ))
                    })?))
                }
            }
        }

        deserializer.deserialize_struct("StatusResult", &[], Visitor(std::marker::PhantomData))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
struct Lease(u32);

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum Attribute {
    SupportedAttrs(BitSet<AttributeId>) = AttributeId::SupportedAttrs as u32,
    Type(FileType) = AttributeId::Type as u32,
    FhExpireType(u32) = AttributeId::FhExpireType as u32,
    Change(u64) = AttributeId::Change as u32,
    Size(u64) = AttributeId::Size as u32,
    LinkSupport(bool) = AttributeId::LinkSupport as u32,
    SymlinkSupport(bool) = AttributeId::SymlinkSupport as u32,
    NamedAttr(bool) = AttributeId::NamedAttr as u32,
    Fsid(FsId) = AttributeId::Fsid as u32,
    UniqueHandles(bool) = AttributeId::UniqueHandles as u32,
    LeaseTime(Lease) = AttributeId::LeaseTime as u32,
    RdattrError(StatusError) = AttributeId::RdattrError as u32,
    Acl(Acl) = AttributeId::Acl as u32,
    AclSupport(u32) = AttributeId::AclSupport as u32,
    Archive(bool) = AttributeId::Archive as u32,
    CanSetTime(bool) = AttributeId::CanSetTime as u32,
    CaseInsensitive(bool) = AttributeId::CaseInsensitive as u32,
    CasePreserving(bool) = AttributeId::CasePreserving as u32,
    ChownRestricted(bool) = AttributeId::ChownRestricted as u32,
    FileHandle(FileHandle) = AttributeId::FileHandle as u32,
    FileId(u64) = AttributeId::FileId as u32,
    FilesAvail(u64) = AttributeId::FilesAvail as u32,
    FilesFree(u64) = AttributeId::FilesFree as u32,
    FilesTotal(u64) = AttributeId::FilesTotal as u32,
    FsLocations(FsLocations) = AttributeId::FsLocations as u32,
    Homogeneous(bool) = AttributeId::Homogeneous as u32,
    MaxFileSize(u64) = AttributeId::MaxFileSize as u32,
    MaxLink(u32) = AttributeId::MaxLink as u32,
    MaxName(u32) = AttributeId::MaxName as u32,
    MaxRead(u64) = AttributeId::MaxRead as u32,
    MaxWrite(u64) = AttributeId::MaxWrite as u32,
    MimeType(String) = AttributeId::MimeType as u32,
    Mode(Mode) = AttributeId::Mode as u32,
    NoTrunc(bool) = AttributeId::NoTrunc as u32,
    Numlinks(u32) = AttributeId::Numlinks as u32,
    Owner(String) = AttributeId::Owner as u32,
    OwnerGroup(String) = AttributeId::OwnerGroup as u32,
    QuotaAvailHard(u64) = AttributeId::QuotaAvailHard as u32,
    QuotaAvailSoft(u64) = AttributeId::QuotaAvailSoft as u32,
    QuotaUsed(u64) = AttributeId::QuotaUsed as u32,
    RawDev(DeviceData) = AttributeId::RawDev as u32,
    SpaceAvail(u64) = AttributeId::SpaceAvail as u32,
    SpaceFree(u64) = AttributeId::SpaceFree as u32,
    SpaceTotal(u64) = AttributeId::SpaceTotal as u32,
    SpaceUsed(u64) = AttributeId::SpaceUsed as u32,
    System(bool) = AttributeId::System as u32,
    TimeAccess(Time) = AttributeId::TimeAccess as u32,
    TimeAccessSet(SetTime) = AttributeId::TimeAccessSet as u32,
    TimeBackup(Time) = AttributeId::TimeBackup as u32,
    TimeCreate(Time) = AttributeId::TimeCreate as u32,
    TimeDelta(Time) = AttributeId::TimeDelta as u32,
    TimeMetadata(Time) = AttributeId::TimeMetadata as u32,
    TimeModify(Time) = AttributeId::TimeModify as u32,
    TimeModifySet(SetTime) = AttributeId::TimeModifySet as u32,
    MountedOnFileid(u64) = AttributeId::MountedOnFileid as u32,
    DirNotifDelay(Time) = AttributeId::DirNotifDelay as u32,
    DirentNotifDelay(Time) = AttributeId::DirentNotifDelay as u32,
    Dacl(AclWithFlags) = AttributeId::Dacl as u32,
    Sacl(AclWithFlags) = AttributeId::Sacl as u32,
    ChangePolicy(ChangePolicy) = AttributeId::ChangePolicy as u32,
    FsStatus(FsStatus) = AttributeId::FsStatus as u32,
    FsLayoutType(Vec<LayoutType>) = AttributeId::FsLayoutType as u32,
    LayoutHint(LayoutHint) = AttributeId::LayoutHint as u32,
    LayoutType(Vec<LayoutType>) = AttributeId::LayoutType as u32,
    LayoutBlksize(u32) = AttributeId::LayoutBlksize as u32,
    LayoutAlignment(u32) = AttributeId::LayoutAlignment as u32,
    FsLocationsInfo(FsLocationsInfo) = AttributeId::FsLocationsInfo as u32,
    MdsThreshold(MdsThreshold) = AttributeId::MdsThreshold as u32,
    RetentionGet(RetentionGet) = AttributeId::RetentionGet as u32,
    RetentionSet(RetentionSet) = AttributeId::RetentionSet as u32,
    RetentevtGet(RetentionGet) = AttributeId::RetentevtGet as u32,
    RetentevtSet(RetentionSet) = AttributeId::RetentevtSet as u32,
    RetentionHold(u64) = AttributeId::RetentionHold as u32,
    ModeSetMasked(ModeMasked) = AttributeId::ModeSetMasked as u32,
    SuppattrExclcreat(BitSet<AttributeId>) = AttributeId::SuppattrExclcreat as u32,
    FsCharsetCap(u32) = AttributeId::FsCharsetCap as u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct CreateArgs {
    object_type: FileType,
    object_name: String,
    create_attrs: BitMap<AttributeId, Attribute>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct DelegpurgeArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct DelegreturnArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct GetattrArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct GetfhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LinkArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LockArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LocktArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LockuArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LookupArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LookuppArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct NverifyArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct OpenArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct OpenattrArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct OpenDowngradeArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct PutfhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct PutpubfhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct PutrootfhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ReadArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ReaddirArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ReadlinkArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct RemoveArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct RenameArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct RestorefhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SavefhArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SecinfoArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SetattrArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct VerifyArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct WriteArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct BackchannelCtlArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct BindConnToSessionArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ExchangeIdArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct CreateSessionArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct DestroySessionArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct FreeStateidArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct GetDirDelegationArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct GetdeviceinfoArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct GetdevicelistArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LayoutcommitArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LayoutgetArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct LayoutreturnArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SecinfoNoNameArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SequenceArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct SetSsvArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct TestStateidArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct WantDelegationArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct DestroyClientidArgs {/* todo */}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
struct ReclaimCompleteArgs {/* todo */}

#[derive(SerializeWithDiscriminant, DeserializeWithDiscriminant, PartialEq, Eq, Clone, Debug)]
#[repr(u32)]
enum ArgOp {
    Access(AccessArgs) = 3,
    Close(CloseArgs) = 4,
    Commit(CommitArgs) = 5,
    Create(CreateArgs) = 6,
    Delegpurge(DelegpurgeArgs) = 7,
    Delegreturn(DelegreturnArgs) = 8,
    Getattr(GetattrArgs) = 9,
    Getfh(GetfhArgs) = 10,
    Link(LinkArgs) = 11,
    Lock(LockArgs) = 12,
    Lockt(LocktArgs) = 13,
    Locku(LockuArgs) = 14,
    Lookup(LookupArgs) = 15,
    Lookupp(LookuppArgs) = 16,
    Nverify(NverifyArgs) = 17,
    Open(OpenArgs) = 18,
    Openattr(OpenattrArgs) = 19,
    OpenDowngrade(OpenDowngradeArgs) = 21,
    Putfh(PutfhArgs) = 22,
    Putpubfh(PutpubfhArgs) = 23,
    Putrootfh(PutrootfhArgs) = 24,
    Read(ReadArgs) = 25,
    Readdir(ReaddirArgs) = 26,
    Readlink(ReadlinkArgs) = 27,
    Remove(RemoveArgs) = 28,
    Rename(RenameArgs) = 29,
    Restorefh(RestorefhArgs) = 31,
    Savefh(SavefhArgs) = 32,
    Secinfo(SecinfoArgs) = 33,
    Setattr(SetattrArgs) = 34,
    Verify(VerifyArgs) = 37,
    Write(WriteArgs) = 38,
    BackchannelCtl(BackchannelCtlArgs) = 40,
    BindConnToSession(BindConnToSessionArgs) = 41,
    ExchangeId(ExchangeIdArgs) = 42,
    CreateSession(CreateSessionArgs) = 43,
    DestroySession(DestroySessionArgs) = 44,
    FreeStateid(FreeStateidArgs) = 45,
    GetDirDelegation(GetDirDelegationArgs) = 46,
    Getdeviceinfo(GetdeviceinfoArgs) = 47,
    Getdevicelist(GetdevicelistArgs) = 48,
    Layoutcommit(LayoutcommitArgs) = 49,
    Layoutget(LayoutgetArgs) = 50,
    Layoutreturn(LayoutreturnArgs) = 51,
    SecinfoNoName(SecinfoNoNameArgs) = 52,
    Sequence(SequenceArgs) = 53,
    SetSsv(SetSsvArgs) = 54,
    TestStateid(TestStateidArgs) = 55,
    WantDelegation(WantDelegationArgs) = 56,
    DestroyClientid(DestroyClientidArgs) = 57,
    ReclaimComplete(ReclaimCompleteArgs) = 58,
}
