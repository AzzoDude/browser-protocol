use serde::{Serialize, Deserialize};

/// Unique identifier of the Cache object.

pub type CacheId = String;

/// type of HTTP response cached

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CachedResponseType {
    #[default]
    Basic,
    Cors,
    Default,
    Error,
    OpaqueResponse,
    OpaqueRedirect,
}

/// Data entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry {
    /// Request URL.

    pub requestURL: String,
    /// Request method.

    pub requestMethod: String,
    /// Request headers

    pub requestHeaders: Vec<Header>,
    /// Number of seconds since epoch.

    pub responseTime: f64,
    /// HTTP response status code.

    pub responseStatus: i64,
    /// HTTP response status text.

    pub responseStatusText: String,
    /// HTTP response type

    pub responseType: CachedResponseType,
    /// Response headers

    pub responseHeaders: Vec<Header>,
}

/// Cache identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    /// An opaque unique id of the cache.

    pub cacheId: CacheId,
    /// Security origin of the cache.

    pub securityOrigin: String,
    /// Storage key of the cache.

    pub storageKey: String,
    /// Storage bucket of the cache.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// The name of the cache.

    pub cacheName: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Header {

    pub name: String,

    pub value: String,
}

/// Cached response

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CachedResponse {
    /// Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)

    pub body: String,
}

/// Deletes a cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCacheParams {
    /// Id of cache for deletion.

    pub cacheId: CacheId,
}

/// Deletes a cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntryParams {
    /// Id of cache where the entry will be deleted.

    pub cacheId: CacheId,
    /// URL spec of the request.

    pub request: String,
}

/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesParams {
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
}

/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesReturns {
    /// Caches for the security origin.

    pub caches: Vec<Cache>,
}

/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseParams {
    /// Id of cache that contains the entry.

    pub cacheId: CacheId,
    /// URL spec of the request.

    pub requestURL: String,
    /// headers of the request.

    pub requestHeaders: Vec<Header>,
}

/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseReturns {
    /// Response read from the cache.

    pub response: CachedResponse,
}

/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesParams {
    /// ID of cache to get entries from.

    pub cacheId: CacheId,
    /// Number of records to skip.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipCount: Option<u64>,
    /// Number of records to fetch.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageSize: Option<u64>,
    /// If present, only return the entries containing this substring in the path

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathFilter: Option<String>,
}

/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesReturns {
    /// Array of object store data entries.

    pub cacheDataEntries: Vec<DataEntry>,
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.

    pub returnCount: f64,
}
