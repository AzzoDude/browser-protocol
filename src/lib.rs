#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

pub use browser_protocol_macros::{CdpBuilder, CdpCommand, CdpEvent};

/// Trait for CDP commands that associate parameters with a method name and response type.
pub trait CdpCommand<'a>: Serialize {
    const METHOD: &'static str;
    type Response: Deserialize<'a>;
}

/// Marker trait implemented by every typed CDP event.
pub trait CdpEvent {
    const METHOD: &'static str;
}

/// A generic CDP command envelope.
#[derive(Serialize)]
pub struct Command<'a, T: CdpCommand<'a>> {
    pub id: u64,
    pub method: &'static str,
    pub params: &'a T,
}

impl<'a, T: CdpCommand<'a>> Command<'a, T> {
    pub fn new(id: u64, params: &'a T) -> Self {
        Self { id, method: T::METHOD, params }
    }
}

/// A generic CDP response envelope.
#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub id: u64,
    pub result: T,
}

/// An empty response for commands that don't return anything.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct EmptyReturns {}

/// A protocol-level error returned by the browser.
#[derive(Deserialize, Debug, Clone)]
pub struct CdpError {
    pub code: i64,
    pub message: String,
    pub data: Option<JsonValue>,
}

/// An error reply envelope: `{"id": N, "error": { ... }}`.
#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub id: u64,
    pub error: CdpError,
}

/// A reply that is either a typed result or a protocol error.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CdpReply<T> {
    Ok(Response<T>),
    Err(ErrorResponse),
}

#[cfg(feature = "runtime")]
pub mod runtime;
#[cfg(feature = "debugger")]
pub mod debugger;
#[cfg(feature = "heapprofiler")]
pub mod heapprofiler;
#[cfg(feature = "profiler")]
pub mod profiler;
#[cfg(feature = "accessibility")]
pub mod accessibility;
#[cfg(feature = "ads")]
pub mod ads;
#[cfg(feature = "animation")]
pub mod animation;
#[cfg(feature = "audits")]
pub mod audits;
#[cfg(feature = "autofill")]
pub mod autofill;
#[cfg(feature = "backgroundservice")]
pub mod backgroundservice;
#[cfg(feature = "bluetoothemulation")]
pub mod bluetoothemulation;
#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "css")]
pub mod css;
#[cfg(feature = "cachestorage")]
pub mod cachestorage;
#[cfg(feature = "cast")]
pub mod cast;
#[cfg(feature = "crashreportcontext")]
pub mod crashreportcontext;
#[cfg(feature = "dom")]
pub mod dom;
#[cfg(feature = "domdebugger")]
pub mod domdebugger;
#[cfg(feature = "domsnapshot")]
pub mod domsnapshot;
#[cfg(feature = "domstorage")]
pub mod domstorage;
#[cfg(feature = "deviceaccess")]
pub mod deviceaccess;
#[cfg(feature = "deviceorientation")]
pub mod deviceorientation;
#[cfg(feature = "digitalcredentials")]
pub mod digitalcredentials;
#[cfg(feature = "emulation")]
pub mod emulation;
#[cfg(feature = "eventbreakpoints")]
pub mod eventbreakpoints;
#[cfg(feature = "extensions")]
pub mod extensions;
#[cfg(feature = "fedcm")]
pub mod fedcm;
#[cfg(feature = "fetch")]
pub mod fetch;
#[cfg(feature = "filesystem")]
pub mod filesystem;
#[cfg(feature = "findinpage")]
pub mod findinpage;
#[cfg(feature = "headlessexperimental")]
pub mod headlessexperimental;
#[cfg(feature = "io")]
pub mod io;
#[cfg(feature = "indexeddb")]
pub mod indexeddb;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "inspector")]
pub mod inspector;
#[cfg(feature = "layertree")]
pub mod layertree;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "media")]
pub mod media;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "overlay")]
pub mod overlay;
#[cfg(feature = "pwa")]
pub mod pwa;
#[cfg(feature = "page")]
pub mod page;
#[cfg(feature = "performance")]
pub mod performance;
#[cfg(feature = "performancetimeline")]
pub mod performancetimeline;
#[cfg(feature = "preload")]
pub mod preload;
#[cfg(feature = "security")]
pub mod security;
#[cfg(feature = "serviceworker")]
pub mod serviceworker;
#[cfg(feature = "smartcardemulation")]
pub mod smartcardemulation;
#[cfg(feature = "storage")]
pub mod storage;
#[cfg(feature = "systeminfo")]
pub mod systeminfo;
#[cfg(feature = "target")]
pub mod target;
#[cfg(feature = "tethering")]
pub mod tethering;
#[cfg(feature = "tracing")]
pub mod tracing;
#[cfg(feature = "webaudio")]
pub mod webaudio;
#[cfg(feature = "webauthn")]
pub mod webauthn;