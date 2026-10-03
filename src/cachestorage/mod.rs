use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique identifier of the Cache object.

pub type CacheId<'a> = Cow<'a, str>;

/// type of HTTP response cached

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CachedResponseType {
    #[default]
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "cors")]
    Cors,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "opaqueResponse")]
    OpaqueResponse,
    #[serde(rename = "opaqueRedirect")]
    OpaqueRedirect,
}

/// Data entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry<'a> {
    /// Request URL.
    #[serde(rename = "requestURL")]
    pub request_url: Cow<'a, str>,
    /// Request method.
    #[serde(rename = "requestMethod")]
    pub request_method: Cow<'a, str>,
    /// Request headers
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<Header<'a>>,
    /// Number of seconds since epoch.
    #[serde(rename = "responseTime")]
    pub response_time: f64,
    /// HTTP response status code.
    #[serde(rename = "responseStatus")]
    pub response_status: i64,
    /// HTTP response status text.
    #[serde(rename = "responseStatusText")]
    pub response_status_text: Cow<'a, str>,
    /// HTTP response type
    #[serde(rename = "responseType")]
    pub response_type: CachedResponseType,
    /// Response headers
    #[serde(rename = "responseHeaders")]
    pub response_headers: Vec<Header<'a>>,
}
/// Cache identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Cache<'a> {
    /// An opaque unique id of the cache.
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId<'a>,
    /// Security origin of the cache.
    #[serde(rename = "securityOrigin")]
    pub security_origin: Cow<'a, str>,
    /// Storage key of the cache.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Storage bucket of the cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// The name of the cache.
    #[serde(rename = "cacheName")]
    pub cache_name: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Header<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}
/// Cached response

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CachedResponse<'a> {
    /// Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)
    pub body: Cow<'a, str>,
}
/// Deletes a cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CacheStorage.deleteCache")]
pub struct DeleteCacheParams<'a> {
    /// Id of cache for deletion.
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId<'a>,
}
/// Deletes a cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CacheStorage.deleteEntry")]
pub struct DeleteEntryParams<'a> {
    /// Id of cache where the entry will be deleted.
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId<'a>,
    /// URL spec of the request.
    pub request: Cow<'a, str>,
}
/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CacheStorage.requestCacheNames", response = "RequestCacheNamesReturns<'a>")]
pub struct RequestCacheNamesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}
/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesReturns<'a> {
    /// Caches for the security origin.
    pub caches: Vec<Cache<'a>>,
}
/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CacheStorage.requestCachedResponse", response = "RequestCachedResponseReturns<'a>")]
pub struct RequestCachedResponseParams<'a> {
    /// Id of cache that contains the entry.
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId<'a>,
    /// URL spec of the request.
    #[serde(rename = "requestURL")]
    pub request_url: Cow<'a, str>,
    /// headers of the request.
    #[serde(rename = "requestHeaders")]
    pub request_headers: Vec<Header<'a>>,
}
/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseReturns<'a> {
    /// Response read from the cache.
    pub response: CachedResponse<'a>,
}
/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "CacheStorage.requestEntries", response = "RequestEntriesReturns<'a>")]
pub struct RequestEntriesParams<'a> {
    /// ID of cache to get entries from.
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId<'a>,
    /// Number of records to skip.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipCount")]
    pub skip_count: Option<u64>,
    /// Number of records to fetch.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    pub page_size: Option<u64>,
    /// If present, only return the entries containing this substring in the path
    #[serde(skip_serializing_if = "Option::is_none", rename = "pathFilter")]
    pub path_filter: Option<Cow<'a, str>>,
}
/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesReturns<'a> {
    /// Array of object store data entries.
    #[serde(rename = "cacheDataEntries")]
    pub cache_data_entries: Vec<DataEntry<'a>>,
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.
    #[serde(rename = "returnCount")]
    pub return_count: f64,
}