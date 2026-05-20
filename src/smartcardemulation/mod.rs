use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Indicates the PC/SC error code.
/// 
/// This maps to:
/// PC/SC Lite: https://pcsclite.apdu.fr/api/group__ErrorCodes.html
/// Microsoft: https://learn.microsoft.com/en-us/windows/win32/secauthn/authentication-return-values

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateFlags {
    #[serde(skip_serializing_if = "Option::is_none")]
    unaware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unknown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unavailable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    empty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inuse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unpowered: Option<bool>,
}

impl ReaderStateFlags {
    pub fn builder() -> ReaderStateFlagsBuilder {
        ReaderStateFlagsBuilder {
            unaware: None,
            ignore: None,
            changed: None,
            unknown: None,
            unavailable: None,
            empty: None,
            present: None,
            exclusive: None,
            inuse: None,
            mute: None,
            unpowered: None,
        }
    }
    pub fn unaware(&self) -> Option<bool> { self.unaware }
    pub fn ignore(&self) -> Option<bool> { self.ignore }
    pub fn changed(&self) -> Option<bool> { self.changed }
    pub fn unknown(&self) -> Option<bool> { self.unknown }
    pub fn unavailable(&self) -> Option<bool> { self.unavailable }
    pub fn empty(&self) -> Option<bool> { self.empty }
    pub fn present(&self) -> Option<bool> { self.present }
    pub fn exclusive(&self) -> Option<bool> { self.exclusive }
    pub fn inuse(&self) -> Option<bool> { self.inuse }
    pub fn mute(&self) -> Option<bool> { self.mute }
    pub fn unpowered(&self) -> Option<bool> { self.unpowered }
}

#[derive(Default)]
pub struct ReaderStateFlagsBuilder {
    unaware: Option<bool>,
    ignore: Option<bool>,
    changed: Option<bool>,
    unknown: Option<bool>,
    unavailable: Option<bool>,
    empty: Option<bool>,
    present: Option<bool>,
    exclusive: Option<bool>,
    inuse: Option<bool>,
    mute: Option<bool>,
    unpowered: Option<bool>,
}

impl ReaderStateFlagsBuilder {
    pub fn unaware(mut self, unaware: bool) -> Self { self.unaware = Some(unaware); self }
    pub fn ignore(mut self, ignore: bool) -> Self { self.ignore = Some(ignore); self }
    pub fn changed(mut self, changed: bool) -> Self { self.changed = Some(changed); self }
    pub fn unknown(mut self, unknown: bool) -> Self { self.unknown = Some(unknown); self }
    pub fn unavailable(mut self, unavailable: bool) -> Self { self.unavailable = Some(unavailable); self }
    pub fn empty(mut self, empty: bool) -> Self { self.empty = Some(empty); self }
    pub fn present(mut self, present: bool) -> Self { self.present = Some(present); self }
    pub fn exclusive(mut self, exclusive: bool) -> Self { self.exclusive = Some(exclusive); self }
    pub fn inuse(mut self, inuse: bool) -> Self { self.inuse = Some(inuse); self }
    pub fn mute(mut self, mute: bool) -> Self { self.mute = Some(mute); self }
    pub fn unpowered(mut self, unpowered: bool) -> Self { self.unpowered = Some(unpowered); self }
    pub fn build(self) -> ReaderStateFlags {
        ReaderStateFlags {
            unaware: self.unaware,
            ignore: self.ignore,
            changed: self.changed,
            unknown: self.unknown,
            unavailable: self.unavailable,
            empty: self.empty,
            present: self.present,
            exclusive: self.exclusive,
            inuse: self.inuse,
            mute: self.mute,
            unpowered: self.unpowered,
        }
    }
}

/// Maps to the |SCARD_PROTOCOL_*| flags.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    t0: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    t1: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw: Option<bool>,
}

impl ProtocolSet {
    pub fn builder() -> ProtocolSetBuilder {
        ProtocolSetBuilder {
            t0: None,
            t1: None,
            raw: None,
        }
    }
    pub fn t0(&self) -> Option<bool> { self.t0 }
    pub fn t1(&self) -> Option<bool> { self.t1 }
    pub fn raw(&self) -> Option<bool> { self.raw }
}

#[derive(Default)]
pub struct ProtocolSetBuilder {
    t0: Option<bool>,
    t1: Option<bool>,
    raw: Option<bool>,
}

impl ProtocolSetBuilder {
    pub fn t0(mut self, t0: bool) -> Self { self.t0 = Some(t0); self }
    pub fn t1(mut self, t1: bool) -> Self { self.t1 = Some(t1); self }
    pub fn raw(mut self, raw: bool) -> Self { self.raw = Some(raw); self }
    pub fn build(self) -> ProtocolSet {
        ProtocolSet {
            t0: self.t0,
            t1: self.t1,
            raw: self.raw,
        }
    }
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateIn<'a> {
    reader: Cow<'a, str>,
    currentState: ReaderStateFlags,
    currentInsertionCount: u64,
}

impl<'a> ReaderStateIn<'a> {
    pub fn builder(reader: impl Into<Cow<'a, str>>, currentState: ReaderStateFlags, currentInsertionCount: u64) -> ReaderStateInBuilder<'a> {
        ReaderStateInBuilder {
            reader: reader.into(),
            currentState: currentState,
            currentInsertionCount: currentInsertionCount,
        }
    }
    pub fn reader(&self) -> &str { self.reader.as_ref() }
    pub fn currentState(&self) -> &ReaderStateFlags { &self.currentState }
    pub fn currentInsertionCount(&self) -> u64 { self.currentInsertionCount }
}


pub struct ReaderStateInBuilder<'a> {
    reader: Cow<'a, str>,
    currentState: ReaderStateFlags,
    currentInsertionCount: u64,
}

impl<'a> ReaderStateInBuilder<'a> {
    pub fn build(self) -> ReaderStateIn<'a> {
        ReaderStateIn {
            reader: self.reader,
            currentState: self.currentState,
            currentInsertionCount: self.currentInsertionCount,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateOut<'a> {
    reader: Cow<'a, str>,
    eventState: ReaderStateFlags,
    eventCount: u64,
    atr: Cow<'a, str>,
}

impl<'a> ReaderStateOut<'a> {
    pub fn builder(reader: impl Into<Cow<'a, str>>, eventState: ReaderStateFlags, eventCount: u64, atr: impl Into<Cow<'a, str>>) -> ReaderStateOutBuilder<'a> {
        ReaderStateOutBuilder {
            reader: reader.into(),
            eventState: eventState,
            eventCount: eventCount,
            atr: atr.into(),
        }
    }
    pub fn reader(&self) -> &str { self.reader.as_ref() }
    pub fn eventState(&self) -> &ReaderStateFlags { &self.eventState }
    pub fn eventCount(&self) -> u64 { self.eventCount }
    pub fn atr(&self) -> &str { self.atr.as_ref() }
}


pub struct ReaderStateOutBuilder<'a> {
    reader: Cow<'a, str>,
    eventState: ReaderStateFlags,
    eventCount: u64,
    atr: Cow<'a, str>,
}

impl<'a> ReaderStateOutBuilder<'a> {
    pub fn build(self) -> ReaderStateOut<'a> {
        ReaderStateOut {
            reader: self.reader,
            eventState: self.eventState,
            eventCount: self.eventCount,
            atr: self.atr,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "SmartCardEmulation.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "SmartCardEmulation.enable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "SmartCardEmulation.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
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
pub struct ReportEstablishContextResultParams<'a> {
    requestId: Cow<'a, str>,
    contextId: u64,
}

impl<'a> ReportEstablishContextResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, contextId: u64) -> ReportEstablishContextResultParamsBuilder<'a> {
        ReportEstablishContextResultParamsBuilder {
            requestId: requestId.into(),
            contextId: contextId,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn contextId(&self) -> u64 { self.contextId }
}


pub struct ReportEstablishContextResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    contextId: u64,
}

impl<'a> ReportEstablishContextResultParamsBuilder<'a> {
    pub fn build(self) -> ReportEstablishContextResultParams<'a> {
        ReportEstablishContextResultParams {
            requestId: self.requestId,
            contextId: self.contextId,
        }
    }
}

impl<'a> ReportEstablishContextResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportEstablishContextResult"; }

impl<'a> crate::CdpCommand<'a> for ReportEstablishContextResultParams<'a> {
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
pub struct ReportReleaseContextResultParams<'a> {
    requestId: Cow<'a, str>,
}

impl<'a> ReportReleaseContextResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>) -> ReportReleaseContextResultParamsBuilder<'a> {
        ReportReleaseContextResultParamsBuilder {
            requestId: requestId.into(),
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
}


pub struct ReportReleaseContextResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
}

impl<'a> ReportReleaseContextResultParamsBuilder<'a> {
    pub fn build(self) -> ReportReleaseContextResultParams<'a> {
        ReportReleaseContextResultParams {
            requestId: self.requestId,
        }
    }
}

impl<'a> ReportReleaseContextResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportReleaseContextResult"; }

impl<'a> crate::CdpCommand<'a> for ReportReleaseContextResultParams<'a> {
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
pub struct ReportListReadersResultParams<'a> {
    requestId: Cow<'a, str>,
    readers: Vec<Cow<'a, str>>,
}

impl<'a> ReportListReadersResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, readers: Vec<Cow<'a, str>>) -> ReportListReadersResultParamsBuilder<'a> {
        ReportListReadersResultParamsBuilder {
            requestId: requestId.into(),
            readers: readers,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn readers(&self) -> &[Cow<'a, str>] { &self.readers }
}


pub struct ReportListReadersResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    readers: Vec<Cow<'a, str>>,
}

impl<'a> ReportListReadersResultParamsBuilder<'a> {
    pub fn build(self) -> ReportListReadersResultParams<'a> {
        ReportListReadersResultParams {
            requestId: self.requestId,
            readers: self.readers,
        }
    }
}

impl<'a> ReportListReadersResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportListReadersResult"; }

impl<'a> crate::CdpCommand<'a> for ReportListReadersResultParams<'a> {
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
pub struct ReportGetStatusChangeResultParams<'a> {
    requestId: Cow<'a, str>,
    readerStates: Vec<ReaderStateOut<'a>>,
}

impl<'a> ReportGetStatusChangeResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, readerStates: Vec<ReaderStateOut<'a>>) -> ReportGetStatusChangeResultParamsBuilder<'a> {
        ReportGetStatusChangeResultParamsBuilder {
            requestId: requestId.into(),
            readerStates: readerStates,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn readerStates(&self) -> &[ReaderStateOut<'a>] { &self.readerStates }
}


pub struct ReportGetStatusChangeResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    readerStates: Vec<ReaderStateOut<'a>>,
}

impl<'a> ReportGetStatusChangeResultParamsBuilder<'a> {
    pub fn build(self) -> ReportGetStatusChangeResultParams<'a> {
        ReportGetStatusChangeResultParams {
            requestId: self.requestId,
            readerStates: self.readerStates,
        }
    }
}

impl<'a> ReportGetStatusChangeResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportGetStatusChangeResult"; }

impl<'a> crate::CdpCommand<'a> for ReportGetStatusChangeResultParams<'a> {
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
pub struct ReportBeginTransactionResultParams<'a> {
    requestId: Cow<'a, str>,
    handle: i64,
}

impl<'a> ReportBeginTransactionResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, handle: i64) -> ReportBeginTransactionResultParamsBuilder<'a> {
        ReportBeginTransactionResultParamsBuilder {
            requestId: requestId.into(),
            handle: handle,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn handle(&self) -> i64 { self.handle }
}


pub struct ReportBeginTransactionResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    handle: i64,
}

impl<'a> ReportBeginTransactionResultParamsBuilder<'a> {
    pub fn build(self) -> ReportBeginTransactionResultParams<'a> {
        ReportBeginTransactionResultParams {
            requestId: self.requestId,
            handle: self.handle,
        }
    }
}

impl<'a> ReportBeginTransactionResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportBeginTransactionResult"; }

impl<'a> crate::CdpCommand<'a> for ReportBeginTransactionResultParams<'a> {
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
pub struct ReportPlainResultParams<'a> {
    requestId: Cow<'a, str>,
}

impl<'a> ReportPlainResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>) -> ReportPlainResultParamsBuilder<'a> {
        ReportPlainResultParamsBuilder {
            requestId: requestId.into(),
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
}


pub struct ReportPlainResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
}

impl<'a> ReportPlainResultParamsBuilder<'a> {
    pub fn build(self) -> ReportPlainResultParams<'a> {
        ReportPlainResultParams {
            requestId: self.requestId,
        }
    }
}

impl<'a> ReportPlainResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportPlainResult"; }

impl<'a> crate::CdpCommand<'a> for ReportPlainResultParams<'a> {
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
pub struct ReportConnectResultParams<'a> {
    requestId: Cow<'a, str>,
    handle: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    activeProtocol: Option<Protocol>,
}

impl<'a> ReportConnectResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, handle: i64) -> ReportConnectResultParamsBuilder<'a> {
        ReportConnectResultParamsBuilder {
            requestId: requestId.into(),
            handle: handle,
            activeProtocol: None,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn handle(&self) -> i64 { self.handle }
    pub fn activeProtocol(&self) -> Option<&Protocol> { self.activeProtocol.as_ref() }
}


pub struct ReportConnectResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    handle: i64,
    activeProtocol: Option<Protocol>,
}

impl<'a> ReportConnectResultParamsBuilder<'a> {
    pub fn activeProtocol(mut self, activeProtocol: impl Into<Protocol>) -> Self { self.activeProtocol = Some(activeProtocol.into()); self }
    pub fn build(self) -> ReportConnectResultParams<'a> {
        ReportConnectResultParams {
            requestId: self.requestId,
            handle: self.handle,
            activeProtocol: self.activeProtocol,
        }
    }
}

impl<'a> ReportConnectResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportConnectResult"; }

impl<'a> crate::CdpCommand<'a> for ReportConnectResultParams<'a> {
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
pub struct ReportDataResultParams<'a> {
    requestId: Cow<'a, str>,
    data: Cow<'a, str>,
}

impl<'a> ReportDataResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, data: impl Into<Cow<'a, str>>) -> ReportDataResultParamsBuilder<'a> {
        ReportDataResultParamsBuilder {
            requestId: requestId.into(),
            data: data.into(),
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn data(&self) -> &str { self.data.as_ref() }
}


pub struct ReportDataResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    data: Cow<'a, str>,
}

impl<'a> ReportDataResultParamsBuilder<'a> {
    pub fn build(self) -> ReportDataResultParams<'a> {
        ReportDataResultParams {
            requestId: self.requestId,
            data: self.data,
        }
    }
}

impl<'a> ReportDataResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportDataResult"; }

impl<'a> crate::CdpCommand<'a> for ReportDataResultParams<'a> {
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
pub struct ReportStatusResultParams<'a> {
    requestId: Cow<'a, str>,
    readerName: Cow<'a, str>,
    state: ConnectionState,
    atr: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<Protocol>,
}

impl<'a> ReportStatusResultParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, readerName: impl Into<Cow<'a, str>>, state: impl Into<ConnectionState>, atr: impl Into<Cow<'a, str>>) -> ReportStatusResultParamsBuilder<'a> {
        ReportStatusResultParamsBuilder {
            requestId: requestId.into(),
            readerName: readerName.into(),
            state: state.into(),
            atr: atr.into(),
            protocol: None,
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn readerName(&self) -> &str { self.readerName.as_ref() }
    pub fn state(&self) -> &ConnectionState { &self.state }
    pub fn atr(&self) -> &str { self.atr.as_ref() }
    pub fn protocol(&self) -> Option<&Protocol> { self.protocol.as_ref() }
}


pub struct ReportStatusResultParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    readerName: Cow<'a, str>,
    state: ConnectionState,
    atr: Cow<'a, str>,
    protocol: Option<Protocol>,
}

impl<'a> ReportStatusResultParamsBuilder<'a> {
    pub fn protocol(mut self, protocol: impl Into<Protocol>) -> Self { self.protocol = Some(protocol.into()); self }
    pub fn build(self) -> ReportStatusResultParams<'a> {
        ReportStatusResultParams {
            requestId: self.requestId,
            readerName: self.readerName,
            state: self.state,
            atr: self.atr,
            protocol: self.protocol,
        }
    }
}

impl<'a> ReportStatusResultParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportStatusResult"; }

impl<'a> crate::CdpCommand<'a> for ReportStatusResultParams<'a> {
    const METHOD: &'static str = "SmartCardEmulation.reportStatusResult";
    type Response = crate::EmptyReturns;
}

/// Reports an error result for the given request.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportErrorParams<'a> {
    requestId: Cow<'a, str>,
    resultCode: ResultCode,
}

impl<'a> ReportErrorParams<'a> {
    pub fn builder(requestId: impl Into<Cow<'a, str>>, resultCode: impl Into<ResultCode>) -> ReportErrorParamsBuilder<'a> {
        ReportErrorParamsBuilder {
            requestId: requestId.into(),
            resultCode: resultCode.into(),
        }
    }
    pub fn requestId(&self) -> &str { self.requestId.as_ref() }
    pub fn resultCode(&self) -> &ResultCode { &self.resultCode }
}


pub struct ReportErrorParamsBuilder<'a> {
    requestId: Cow<'a, str>,
    resultCode: ResultCode,
}

impl<'a> ReportErrorParamsBuilder<'a> {
    pub fn build(self) -> ReportErrorParams<'a> {
        ReportErrorParams {
            requestId: self.requestId,
            resultCode: self.resultCode,
        }
    }
}

impl<'a> ReportErrorParams<'a> { pub const METHOD: &'static str = "SmartCardEmulation.reportError"; }

impl<'a> crate::CdpCommand<'a> for ReportErrorParams<'a> {
    const METHOD: &'static str = "SmartCardEmulation.reportError";
    type Response = crate::EmptyReturns;
}
