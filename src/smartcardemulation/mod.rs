use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Indicates the PC/SC error code.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__ErrorCodes.html>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/secauthn/authentication-return-values>

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ResultCode {
    #[default]
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "removed-card")]
    RemovedCard,
    #[serde(rename = "reset-card")]
    ResetCard,
    #[serde(rename = "unpowered-card")]
    UnpoweredCard,
    #[serde(rename = "unresponsive-card")]
    UnresponsiveCard,
    #[serde(rename = "unsupported-card")]
    UnsupportedCard,
    #[serde(rename = "reader-unavailable")]
    ReaderUnavailable,
    #[serde(rename = "sharing-violation")]
    SharingViolation,
    #[serde(rename = "not-transacted")]
    NotTransacted,
    #[serde(rename = "no-smartcard")]
    NoSmartcard,
    #[serde(rename = "proto-mismatch")]
    ProtoMismatch,
    #[serde(rename = "system-cancelled")]
    SystemCancelled,
    #[serde(rename = "not-ready")]
    NotReady,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "insufficient-buffer")]
    InsufficientBuffer,
    #[serde(rename = "invalid-handle")]
    InvalidHandle,
    #[serde(rename = "invalid-parameter")]
    InvalidParameter,
    #[serde(rename = "invalid-value")]
    InvalidValue,
    #[serde(rename = "no-memory")]
    NoMemory,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "unknown-reader")]
    UnknownReader,
    #[serde(rename = "unsupported-feature")]
    UnsupportedFeature,
    #[serde(rename = "no-readers-available")]
    NoReadersAvailable,
    #[serde(rename = "service-stopped")]
    ServiceStopped,
    #[serde(rename = "no-service")]
    NoService,
    #[serde(rename = "comm-error")]
    CommError,
    #[serde(rename = "internal-error")]
    InternalError,
    #[serde(rename = "server-too-busy")]
    ServerTooBusy,
    #[serde(rename = "unexpected")]
    Unexpected,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "unknown-card")]
    UnknownCard,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Maps to the |SCARD_SHARE_*| values.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ShareMode {
    #[default]
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "direct")]
    Direct,
}

/// Indicates what the reader should do with the card.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Disposition {
    #[default]
    #[serde(rename = "leave-card")]
    LeaveCard,
    #[serde(rename = "reset-card")]
    ResetCard,
    #[serde(rename = "unpower-card")]
    UnpowerCard,
    #[serde(rename = "eject-card")]
    EjectCard,
}

/// Maps to |SCARD_*| connection state values.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ConnectionState {
    #[default]
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "swallowed")]
    Swallowed,
    #[serde(rename = "powered")]
    Powered,
    #[serde(rename = "negotiable")]
    Negotiable,
    #[serde(rename = "specific")]
    Specific,
}

/// Maps to the |SCARD_STATE_*| flags.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
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
    #[serde(rename = "t0")]
    T0,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "raw")]
    Raw,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateIn<'a> {
    pub reader: Cow<'a, str>,
    #[serde(rename = "currentState")]
    pub current_state: ReaderStateFlags,
    #[serde(rename = "currentInsertionCount")]
    pub current_insertion_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateOut<'a> {
    pub reader: Cow<'a, str>,
    #[serde(rename = "eventState")]
    pub event_state: ReaderStateFlags,
    #[serde(rename = "eventCount")]
    pub event_count: u64,
    pub atr: Cow<'a, str>,
}
/// Enables the |SmartCardEmulation| domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.enable")]
pub struct EnableParams {

}
/// Disables the |SmartCardEmulation| domain.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.disable")]
pub struct DisableParams {

}
/// Reports the successful result of a |SCardEstablishContext| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportEstablishContextResult")]
pub struct ReportEstablishContextResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
}
/// Reports the successful result of a |SCardReleaseContext| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportReleaseContextResult")]
pub struct ReportReleaseContextResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
}
/// Reports the successful result of a |SCardListReaders| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportListReadersResult")]
pub struct ReportListReadersResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub readers: Vec<Cow<'a, str>>,
}
/// Reports the successful result of a |SCardGetStatusChange| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportGetStatusChangeResult")]
pub struct ReportGetStatusChangeResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "readerStates")]
    pub reader_states: Vec<ReaderStateOut<'a>>,
}
/// Reports the result of a |SCardBeginTransaction| call.
/// On success, this creates a new transaction object.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportBeginTransactionResult")]
pub struct ReportBeginTransactionResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
}
/// Reports the successful result of a call that returns only a result code.
/// Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.
/// 
/// This maps to:
/// 1. SCardCancel
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel>
/// 
/// 2. SCardDisconnect
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect>
/// 
/// 3. SCardSetAttrib
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib>
/// 
/// 4. SCardEndTransaction
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportPlainResult")]
pub struct ReportPlainResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
}
/// Reports the successful result of a |SCardConnect| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportConnectResult")]
pub struct ReportConnectResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeProtocol")]
    pub active_protocol: Option<Protocol>,
}
/// Reports the successful result of a call that sends back data on success.
/// Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.
/// 
/// This maps to:
/// 1. SCardTransmit
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit>
/// 
/// 2. SCardControl
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol>
/// 
/// 3. SCardGetAttrib
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportDataResult")]
pub struct ReportDataResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub data: Cow<'a, str>,
}
/// Reports the successful result of a |SCardStatus| call.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportStatusResult")]
pub struct ReportStatusResultParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "readerName")]
    pub reader_name: Cow<'a, str>,
    pub state: ConnectionState,
    pub atr: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}
/// Reports an error result for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.reportError")]
pub struct ReportErrorParams<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "resultCode")]
    pub result_code: ResultCode,
}
/// Fired when |SCardEstablishContext| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.establishContextRequested")]
pub struct EstablishContextRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
}
/// Fired when |SCardReleaseContext| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.releaseContextRequested")]
pub struct ReleaseContextRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
}
/// Fired when |SCardListReaders| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.listReadersRequested")]
pub struct ListReadersRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
}
/// Fired when |SCardGetStatusChange| is called. Timeout is specified in milliseconds.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.getStatusChangeRequested")]
pub struct GetStatusChangeRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
    #[serde(rename = "readerStates")]
    pub reader_states: Vec<ReaderStateIn<'a>>,
    /// in milliseconds, if absent, it means "infinite"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
/// Fired when |SCardCancel| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.cancelRequested")]
pub struct CancelRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
}
/// Fired when |SCardConnect| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.connectRequested")]
pub struct ConnectRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    #[serde(rename = "contextId")]
    pub context_id: u64,
    pub reader: Cow<'a, str>,
    #[serde(rename = "shareMode")]
    pub share_mode: ShareMode,
    #[serde(rename = "preferredProtocols")]
    pub preferred_protocols: ProtocolSet,
}
/// Fired when |SCardDisconnect| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.disconnectRequested")]
pub struct DisconnectRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    pub disposition: Disposition,
}
/// Fired when |SCardTransmit| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.transmitRequested")]
pub struct TransmitRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    pub data: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}
/// Fired when |SCardControl| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.controlRequested")]
pub struct ControlRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    #[serde(rename = "controlCode")]
    pub control_code: i64,
    pub data: Cow<'a, str>,
}
/// Fired when |SCardGetAttrib| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.getAttribRequested")]
pub struct GetAttribRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    #[serde(rename = "attribId")]
    pub attrib_id: u64,
}
/// Fired when |SCardSetAttrib| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.setAttribRequested")]
pub struct SetAttribRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    #[serde(rename = "attribId")]
    pub attrib_id: u64,
    pub data: Cow<'a, str>,
}
/// Fired when |SCardStatus| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.statusRequested")]
pub struct StatusRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
}
/// Fired when |SCardBeginTransaction| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.beginTransactionRequested")]
pub struct BeginTransactionRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
}
/// Fired when |SCardEndTransaction| is called.
/// 
/// This maps to:
/// PC/SC Lite: <https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b>
/// Microsoft: <https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction>

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "SmartCardEmulation.endTransactionRequested")]
pub struct EndTransactionRequested<'a> {
    #[serde(rename = "requestId")]
    pub request_id: Cow<'a, str>,
    pub handle: i64,
    pub disposition: Disposition,
}