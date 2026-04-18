use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Indicates the PC/SC error code.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__ErrorCodes.html
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/secauthn/authentication-return-values

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResultCode {
    #[default]
    Success,
    RemovedCard,
    ResetCard,
    UnpoweredCard,
    UnresponsiveCard,
    UnsupportedCard,
    ReaderUnavailable,
    SharingViolation,
    NotTransacted,
    NoSmartcard,
    ProtoMismatch,
    SystemCancelled,
    NotReady,
    Cancelled,
    InsufficientBuffer,
    InvalidHandle,
    InvalidParameter,
    InvalidValue,
    NoMemory,
    Timeout,
    UnknownReader,
    UnsupportedFeature,
    NoReadersAvailable,
    ServiceStopped,
    NoService,
    CommError,
    InternalError,
    ServerTooBusy,
    Unexpected,
    Shutdown,
    UnknownCard,
    Unknown,
}

/// Maps to the |SCARD_SHARE_*| values.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ShareMode {
    #[default]
    Shared,
    Exclusive,
    Direct,
}

/// Indicates what the reader should do with the card.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Disposition {
    #[default]
    LeaveCard,
    ResetCard,
    UnpowerCard,
    EjectCard,
}

/// Maps to |SCARD_*| connection state values.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionState {
    #[default]
    Absent,
    Present,
    Swallowed,
    Powered,
    Negotiable,
    Specific,
}

/// Maps to the |SCARD_STATE_*| flags.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateFlags {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unaware: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub present: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inuse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpowered: Option<bool>,
}

/// Maps to the |SCARD_PROTOCOL_*| flags.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSet {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub t0: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub t1: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
}

/// Maps to the |SCARD_PROTOCOL_*| values.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Protocol {
    #[default]
    T0,
    T1,
    Raw,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateIn {

    pub reader: String,

    pub currentState: ReaderStateFlags,

    pub currentInsertionCount: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateOut {

    pub reader: String,

    pub eventState: ReaderStateFlags,

    pub eventCount: u64,

    pub atr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "SmartCardEmulation.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "SmartCardEmulation.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "SmartCardEmulation.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "SmartCardEmulation.disable";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardEstablishContext| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportEstablishContextResultParams {

    pub requestId: String,

    pub contextId: u64,
}

impl ReportEstablishContextResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportEstablishContextResult"; }

impl crate::CdpCommand for ReportEstablishContextResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportEstablishContextResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardReleaseContext| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportReleaseContextResultParams {

    pub requestId: String,
}

impl ReportReleaseContextResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportReleaseContextResult"; }

impl crate::CdpCommand for ReportReleaseContextResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportReleaseContextResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardListReaders| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportListReadersResultParams {

    pub requestId: String,

    pub readers: Vec<String>,
}

impl ReportListReadersResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportListReadersResult"; }

impl crate::CdpCommand for ReportListReadersResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportListReadersResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardGetStatusChange| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportGetStatusChangeResultParams {

    pub requestId: String,

    pub readerStates: Vec<ReaderStateOut>,
}

impl ReportGetStatusChangeResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportGetStatusChangeResult"; }

impl crate::CdpCommand for ReportGetStatusChangeResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportGetStatusChangeResult";
    type Response = crate::EmptyReturns;
}

/// Reports the result of a |SCardBeginTransaction| call.
/// On success, this creates a new transaction object.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportBeginTransactionResultParams {

    pub requestId: String,

    pub handle: i64,
}

impl ReportBeginTransactionResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportBeginTransactionResult"; }

impl crate::CdpCommand for ReportBeginTransactionResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportBeginTransactionResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a call that returns only a result code.
/// Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.
/// 
/// This maps to:
/// 1. SCardCancel
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel
/// 
/// 2. SCardDisconnect
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect
/// 
/// 3. SCardSetAttrib
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib
/// 
/// 4. SCardEndTransaction
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportPlainResultParams {

    pub requestId: String,
}

impl ReportPlainResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportPlainResult"; }

impl crate::CdpCommand for ReportPlainResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportPlainResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardConnect| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportConnectResultParams {

    pub requestId: String,

    pub handle: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activeProtocol: Option<Protocol>,
}

impl ReportConnectResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportConnectResult"; }

impl crate::CdpCommand for ReportConnectResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportConnectResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a call that sends back data on success.
/// Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.
/// 
/// This maps to:
/// 1. SCardTransmit
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit
/// 
/// 2. SCardControl
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol
/// 
/// 3. SCardGetAttrib
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataResultParams {

    pub requestId: String,

    pub data: String,
}

impl ReportDataResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportDataResult"; }

impl crate::CdpCommand for ReportDataResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportDataResult";
    type Response = crate::EmptyReturns;
}

/// Reports the successful result of a |SCardStatus| call.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportStatusResultParams {

    pub requestId: String,

    pub readerName: String,

    pub state: ConnectionState,

    pub atr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

impl ReportStatusResultParams { pub const METHOD: &'static str = "SmartCardEmulation.reportStatusResult"; }

impl crate::CdpCommand for ReportStatusResultParams {
    const METHOD: &'static str = "SmartCardEmulation.reportStatusResult";
    type Response = crate::EmptyReturns;
}

/// Reports an error result for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportErrorParams {

    pub requestId: String,

    pub resultCode: ResultCode,
}

impl ReportErrorParams { pub const METHOD: &'static str = "SmartCardEmulation.reportError"; }

impl crate::CdpCommand for ReportErrorParams {
    const METHOD: &'static str = "SmartCardEmulation.reportError";
    type Response = crate::EmptyReturns;
}
