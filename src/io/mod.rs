//! Input/Output operations for streams produced by DevTools.

use serde::{Serialize, Deserialize};

/// This is either obtained from another method or specified as 'blob:\<uuid\>' where
/// '\<uuid\>' is an UUID of a Blob.

pub type StreamHandle = String;

/// Close the stream, discard any temporary backing storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseParams {
    /// Handle of the stream to close.

    pub handle: StreamHandle,
}

/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadParams {
    /// Handle of the stream to read.

    pub handle: StreamHandle,
    /// Seek to the specified offset before reading (if not specified, proceed with offset
    /// following the last read). Some types of streams may only support sequential reads.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Maximum number of bytes to read (left upon the agent discretion if not specified).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadReturns {
    /// Set if the data is base64-encoded

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64Encoded: Option<bool>,
    /// Data that were read.

    pub data: String,
    /// Set if the end-of-file condition occurred while reading.

    pub eof: bool,
}

/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobParams {
    /// Object id of a Blob object wrapper.

    pub objectId: crate::runtime::RemoteObjectId,
}

/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobReturns {
    /// UUID of the specified Blob.

    pub uuid: String,
}
