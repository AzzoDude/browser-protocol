//! Input/Output operations for streams produced by DevTools.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// This is either obtained from another method or specified as 'blob:<uuid>' where
/// '<uuid>' is an UUID of a Blob.

pub type StreamHandle<'a> = Cow<'a, str>;

/// Close the stream, discard any temporary backing storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CloseParams<'a> {
    /// Handle of the stream to close.
    handle: StreamHandle<'a>,
}

impl<'a> CloseParams<'a> {
    pub fn builder(handle: impl Into<StreamHandle<'a>>) -> CloseParamsBuilder<'a> {
        CloseParamsBuilder {
            handle: handle.into(),
        }
    }
    pub fn handle(&self) -> &StreamHandle<'a> { &self.handle }
}


pub struct CloseParamsBuilder<'a> {
    handle: StreamHandle<'a>,
}

impl<'a> CloseParamsBuilder<'a> {
    pub fn build(self) -> CloseParams<'a> {
        CloseParams {
            handle: self.handle,
        }
    }
}

impl<'a> CloseParams<'a> { pub const METHOD: &'static str = "IO.close"; }

impl<'a> crate::CdpCommand<'a> for CloseParams<'a> {
    const METHOD: &'static str = "IO.close";
    type Response = crate::EmptyReturns;
}

/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadParams<'a> {
    /// Handle of the stream to read.
    handle: StreamHandle<'a>,
    /// Seek to the specified offset before reading (if not specified, proceed with offset
    /// following the last read). Some types of streams may only support sequential reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    /// Maximum number of bytes to read (left upon the agent discretion if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
}

impl<'a> ReadParams<'a> {
    pub fn builder(handle: impl Into<StreamHandle<'a>>) -> ReadParamsBuilder<'a> {
        ReadParamsBuilder {
            handle: handle.into(),
            offset: None,
            size: None,
        }
    }
    pub fn handle(&self) -> &StreamHandle<'a> { &self.handle }
    pub fn offset(&self) -> Option<i32> { self.offset }
    pub fn size(&self) -> Option<u64> { self.size }
}


pub struct ReadParamsBuilder<'a> {
    handle: StreamHandle<'a>,
    offset: Option<i32>,
    size: Option<u64>,
}

impl<'a> ReadParamsBuilder<'a> {
    /// Seek to the specified offset before reading (if not specified, proceed with offset
    /// following the last read). Some types of streams may only support sequential reads.
    pub fn offset(mut self, offset: i32) -> Self { self.offset = Some(offset); self }
    /// Maximum number of bytes to read (left upon the agent discretion if not specified).
    pub fn size(mut self, size: u64) -> Self { self.size = Some(size); self }
    pub fn build(self) -> ReadParams<'a> {
        ReadParams {
            handle: self.handle,
            offset: self.offset,
            size: self.size,
        }
    }
}

/// Read a chunk of the stream

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadReturns<'a> {
    /// Set if the data is base64-encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    base64Encoded: Option<bool>,
    /// Data that were read.
    data: Cow<'a, str>,
    /// Set if the end-of-file condition occurred while reading.
    eof: bool,
}

impl<'a> ReadReturns<'a> {
    pub fn builder(data: impl Into<Cow<'a, str>>, eof: bool) -> ReadReturnsBuilder<'a> {
        ReadReturnsBuilder {
            base64Encoded: None,
            data: data.into(),
            eof: eof,
        }
    }
    pub fn base64Encoded(&self) -> Option<bool> { self.base64Encoded }
    pub fn data(&self) -> &str { self.data.as_ref() }
    pub fn eof(&self) -> bool { self.eof }
}


pub struct ReadReturnsBuilder<'a> {
    base64Encoded: Option<bool>,
    data: Cow<'a, str>,
    eof: bool,
}

impl<'a> ReadReturnsBuilder<'a> {
    /// Set if the data is base64-encoded
    pub fn base64Encoded(mut self, base64Encoded: bool) -> Self { self.base64Encoded = Some(base64Encoded); self }
    pub fn build(self) -> ReadReturns<'a> {
        ReadReturns {
            base64Encoded: self.base64Encoded,
            data: self.data,
            eof: self.eof,
        }
    }
}

impl<'a> ReadParams<'a> { pub const METHOD: &'static str = "IO.read"; }

impl<'a> crate::CdpCommand<'a> for ReadParams<'a> {
    const METHOD: &'static str = "IO.read";
    type Response = ReadReturns<'a>;
}

/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobParams<'a> {
    /// Object id of a Blob object wrapper.
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> ResolveBlobParams<'a> {
    pub fn builder(objectId: crate::runtime::RemoteObjectId<'a>) -> ResolveBlobParamsBuilder<'a> {
        ResolveBlobParamsBuilder {
            objectId: objectId,
        }
    }
    pub fn objectId(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.objectId }
}


pub struct ResolveBlobParamsBuilder<'a> {
    objectId: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> ResolveBlobParamsBuilder<'a> {
    pub fn build(self) -> ResolveBlobParams<'a> {
        ResolveBlobParams {
            objectId: self.objectId,
        }
    }
}

/// Return UUID of Blob object specified by a remote object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveBlobReturns<'a> {
    /// UUID of the specified Blob.
    uuid: Cow<'a, str>,
}

impl<'a> ResolveBlobReturns<'a> {
    pub fn builder(uuid: impl Into<Cow<'a, str>>) -> ResolveBlobReturnsBuilder<'a> {
        ResolveBlobReturnsBuilder {
            uuid: uuid.into(),
        }
    }
    pub fn uuid(&self) -> &str { self.uuid.as_ref() }
}


pub struct ResolveBlobReturnsBuilder<'a> {
    uuid: Cow<'a, str>,
}

impl<'a> ResolveBlobReturnsBuilder<'a> {
    pub fn build(self) -> ResolveBlobReturns<'a> {
        ResolveBlobReturns {
            uuid: self.uuid,
        }
    }
}

impl<'a> ResolveBlobParams<'a> { pub const METHOD: &'static str = "IO.resolveBlob"; }

impl<'a> crate::CdpCommand<'a> for ResolveBlobParams<'a> {
    const METHOD: &'static str = "IO.resolveBlob";
    type Response = ResolveBlobReturns<'a>;
}
