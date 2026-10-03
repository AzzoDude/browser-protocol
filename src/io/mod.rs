//! Input/Output operations for streams produced by DevTools.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// This is either obtained from another method or specified as 'blob:\<uuid\>' where
/// '\<uuid\>' is an UUID of a Blob.

pub type StreamHandle<'a> = Cow<'a, str>;

/// Close the stream, discard any temporary backing storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IO.close")]
pub struct CloseParams<'a> {
    /// Handle of the stream to close.
    pub handle: StreamHandle<'a>,
}
/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IO.read", response = "ReadReturns<'a>")]
pub struct ReadParams<'a> {
    /// Handle of the stream to read.
    pub handle: StreamHandle<'a>,
    /// Seek to the specified offset before reading (if not specified, proceed with offset
    /// following the last read). Some types of streams may only support sequential reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of bytes to read (left upon the agent discretion if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}
/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ReadReturns<'a> {
    /// Set if the data is base64-encoded
    #[serde(skip_serializing_if = "Option::is_none", rename = "base64Encoded")]
    pub base64_encoded: Option<bool>,
    /// Data that were read.
    pub data: Cow<'a, str>,
    /// Set if the end-of-file condition occurred while reading.
    pub eof: bool,
}
/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IO.resolveBlob", response = "ResolveBlobReturns<'a>")]
pub struct ResolveBlobParams<'a> {
    /// Object id of a Blob object wrapper.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
}
/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobReturns<'a> {
    /// UUID of the specified Blob.
    pub uuid: Cow<'a, str>,
}