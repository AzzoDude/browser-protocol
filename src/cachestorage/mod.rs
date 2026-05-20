use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry<'a> {
    /// Request URL.
    #[serde(rename = "requestURL")]
    request_url: Cow<'a, str>,
    /// Request method.
    #[serde(rename = "requestMethod")]
    request_method: Cow<'a, str>,
    /// Request headers
    #[serde(rename = "requestHeaders")]
    request_headers: Vec<Header<'a>>,
    /// Number of seconds since epoch.
    #[serde(rename = "responseTime")]
    response_time: f64,
    /// HTTP response status code.
    #[serde(rename = "responseStatus")]
    response_status: i64,
    /// HTTP response status text.
    #[serde(rename = "responseStatusText")]
    response_status_text: Cow<'a, str>,
    /// HTTP response type
    #[serde(rename = "responseType")]
    response_type: CachedResponseType,
    /// Response headers
    #[serde(rename = "responseHeaders")]
    response_headers: Vec<Header<'a>>,
}

impl<'a> DataEntry<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `request_url`: Request URL.
    /// * `request_method`: Request method.
    /// * `request_headers`: Request headers
    /// * `response_time`: Number of seconds since epoch.
    /// * `response_status`: HTTP response status code.
    /// * `response_status_text`: HTTP response status text.
    /// * `response_type`: HTTP response type
    /// * `response_headers`: Response headers
    pub fn builder(request_url: impl Into<Cow<'a, str>>, request_method: impl Into<Cow<'a, str>>, request_headers: Vec<Header<'a>>, response_time: f64, response_status: i64, response_status_text: impl Into<Cow<'a, str>>, response_type: impl Into<CachedResponseType>, response_headers: Vec<Header<'a>>) -> DataEntryBuilder<'a> {
        DataEntryBuilder {
            request_url: request_url.into(),
            request_method: request_method.into(),
            request_headers: request_headers,
            response_time: response_time,
            response_status: response_status,
            response_status_text: response_status_text.into(),
            response_type: response_type.into(),
            response_headers: response_headers,
        }
    }
    /// Request URL.
    pub fn request_url(&self) -> &str { self.request_url.as_ref() }
    /// Request method.
    pub fn request_method(&self) -> &str { self.request_method.as_ref() }
    /// Request headers
    pub fn request_headers(&self) -> &[Header<'a>] { &self.request_headers }
    /// Number of seconds since epoch.
    pub fn response_time(&self) -> f64 { self.response_time }
    /// HTTP response status code.
    pub fn response_status(&self) -> i64 { self.response_status }
    /// HTTP response status text.
    pub fn response_status_text(&self) -> &str { self.response_status_text.as_ref() }
    /// HTTP response type
    pub fn response_type(&self) -> &CachedResponseType { &self.response_type }
    /// Response headers
    pub fn response_headers(&self) -> &[Header<'a>] { &self.response_headers }
}


pub struct DataEntryBuilder<'a> {
    request_url: Cow<'a, str>,
    request_method: Cow<'a, str>,
    request_headers: Vec<Header<'a>>,
    response_time: f64,
    response_status: i64,
    response_status_text: Cow<'a, str>,
    response_type: CachedResponseType,
    response_headers: Vec<Header<'a>>,
}

impl<'a> DataEntryBuilder<'a> {
    pub fn build(self) -> DataEntry<'a> {
        DataEntry {
            request_url: self.request_url,
            request_method: self.request_method,
            request_headers: self.request_headers,
            response_time: self.response_time,
            response_status: self.response_status,
            response_status_text: self.response_status_text,
            response_type: self.response_type,
            response_headers: self.response_headers,
        }
    }
}

/// Cache identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cache<'a> {
    /// An opaque unique id of the cache.
    #[serde(rename = "cacheId")]
    cache_id: CacheId<'a>,
    /// Security origin of the cache.
    #[serde(rename = "securityOrigin")]
    security_origin: Cow<'a, str>,
    /// Storage key of the cache.
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
    /// Storage bucket of the cache.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// The name of the cache.
    #[serde(rename = "cacheName")]
    cache_name: Cow<'a, str>,
}

impl<'a> Cache<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_id`: An opaque unique id of the cache.
    /// * `security_origin`: Security origin of the cache.
    /// * `storage_key`: Storage key of the cache.
    /// * `cache_name`: The name of the cache.
    pub fn builder(cache_id: impl Into<CacheId<'a>>, security_origin: impl Into<Cow<'a, str>>, storage_key: impl Into<Cow<'a, str>>, cache_name: impl Into<Cow<'a, str>>) -> CacheBuilder<'a> {
        CacheBuilder {
            cache_id: cache_id.into(),
            security_origin: security_origin.into(),
            storage_key: storage_key.into(),
            storage_bucket: None,
            cache_name: cache_name.into(),
        }
    }
    /// An opaque unique id of the cache.
    pub fn cache_id(&self) -> &CacheId<'a> { &self.cache_id }
    /// Security origin of the cache.
    pub fn security_origin(&self) -> &str { self.security_origin.as_ref() }
    /// Storage key of the cache.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
    /// Storage bucket of the cache.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// The name of the cache.
    pub fn cache_name(&self) -> &str { self.cache_name.as_ref() }
}


pub struct CacheBuilder<'a> {
    cache_id: CacheId<'a>,
    security_origin: Cow<'a, str>,
    storage_key: Cow<'a, str>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    cache_name: Cow<'a, str>,
}

impl<'a> CacheBuilder<'a> {
    /// Storage bucket of the cache.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> Cache<'a> {
        Cache {
            cache_id: self.cache_id,
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            cache_name: self.cache_name,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Header<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> Header<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `value`: 
    pub fn builder(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> HeaderBuilder<'a> {
        HeaderBuilder {
            name: name.into(),
            value: value.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct HeaderBuilder<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> HeaderBuilder<'a> {
    pub fn build(self) -> Header<'a> {
        Header {
            name: self.name,
            value: self.value,
        }
    }
}

/// Cached response

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CachedResponse<'a> {
    /// Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)
    body: Cow<'a, str>,
}

impl<'a> CachedResponse<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `body`: Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)
    pub fn builder(body: impl Into<Cow<'a, str>>) -> CachedResponseBuilder<'a> {
        CachedResponseBuilder {
            body: body.into(),
        }
    }
    /// Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)
    pub fn body(&self) -> &str { self.body.as_ref() }
}


pub struct CachedResponseBuilder<'a> {
    body: Cow<'a, str>,
}

impl<'a> CachedResponseBuilder<'a> {
    pub fn build(self) -> CachedResponse<'a> {
        CachedResponse {
            body: self.body,
        }
    }
}

/// Deletes a cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCacheParams<'a> {
    /// Id of cache for deletion.
    #[serde(rename = "cacheId")]
    cache_id: CacheId<'a>,
}

impl<'a> DeleteCacheParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_id`: Id of cache for deletion.
    pub fn builder(cache_id: impl Into<CacheId<'a>>) -> DeleteCacheParamsBuilder<'a> {
        DeleteCacheParamsBuilder {
            cache_id: cache_id.into(),
        }
    }
    /// Id of cache for deletion.
    pub fn cache_id(&self) -> &CacheId<'a> { &self.cache_id }
}


pub struct DeleteCacheParamsBuilder<'a> {
    cache_id: CacheId<'a>,
}

impl<'a> DeleteCacheParamsBuilder<'a> {
    pub fn build(self) -> DeleteCacheParams<'a> {
        DeleteCacheParams {
            cache_id: self.cache_id,
        }
    }
}

impl<'a> DeleteCacheParams<'a> { pub const METHOD: &'static str = "CacheStorage.deleteCache"; }

impl<'a> crate::CdpCommand<'a> for DeleteCacheParams<'a> {
    const METHOD: &'static str = "CacheStorage.deleteCache";
    type Response = crate::EmptyReturns;
}

/// Deletes a cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEntryParams<'a> {
    /// Id of cache where the entry will be deleted.
    #[serde(rename = "cacheId")]
    cache_id: CacheId<'a>,
    /// URL spec of the request.
    request: Cow<'a, str>,
}

impl<'a> DeleteEntryParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_id`: Id of cache where the entry will be deleted.
    /// * `request`: URL spec of the request.
    pub fn builder(cache_id: impl Into<CacheId<'a>>, request: impl Into<Cow<'a, str>>) -> DeleteEntryParamsBuilder<'a> {
        DeleteEntryParamsBuilder {
            cache_id: cache_id.into(),
            request: request.into(),
        }
    }
    /// Id of cache where the entry will be deleted.
    pub fn cache_id(&self) -> &CacheId<'a> { &self.cache_id }
    /// URL spec of the request.
    pub fn request(&self) -> &str { self.request.as_ref() }
}


pub struct DeleteEntryParamsBuilder<'a> {
    cache_id: CacheId<'a>,
    request: Cow<'a, str>,
}

impl<'a> DeleteEntryParamsBuilder<'a> {
    pub fn build(self) -> DeleteEntryParams<'a> {
        DeleteEntryParams {
            cache_id: self.cache_id,
            request: self.request,
        }
    }
}

impl<'a> DeleteEntryParams<'a> { pub const METHOD: &'static str = "CacheStorage.deleteEntry"; }

impl<'a> crate::CdpCommand<'a> for DeleteEntryParams<'a> {
    const METHOD: &'static str = "CacheStorage.deleteEntry";
    type Response = crate::EmptyReturns;
}

/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestCacheNamesParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> RequestCacheNamesParamsBuilder<'a> {
        RequestCacheNamesParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
}

#[derive(Default)]
pub struct RequestCacheNamesParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestCacheNamesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> RequestCacheNamesParams<'a> {
        RequestCacheNamesParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
        }
    }
}

/// Requests cache names.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCacheNamesReturns<'a> {
    /// Caches for the security origin.
    caches: Vec<Cache<'a>>,
}

impl<'a> RequestCacheNamesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `caches`: Caches for the security origin.
    pub fn builder(caches: Vec<Cache<'a>>) -> RequestCacheNamesReturnsBuilder<'a> {
        RequestCacheNamesReturnsBuilder {
            caches: caches,
        }
    }
    /// Caches for the security origin.
    pub fn caches(&self) -> &[Cache<'a>] { &self.caches }
}


pub struct RequestCacheNamesReturnsBuilder<'a> {
    caches: Vec<Cache<'a>>,
}

impl<'a> RequestCacheNamesReturnsBuilder<'a> {
    pub fn build(self) -> RequestCacheNamesReturns<'a> {
        RequestCacheNamesReturns {
            caches: self.caches,
        }
    }
}

impl<'a> RequestCacheNamesParams<'a> { pub const METHOD: &'static str = "CacheStorage.requestCacheNames"; }

impl<'a> crate::CdpCommand<'a> for RequestCacheNamesParams<'a> {
    const METHOD: &'static str = "CacheStorage.requestCacheNames";
    type Response = RequestCacheNamesReturns<'a>;
}

/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseParams<'a> {
    /// Id of cache that contains the entry.
    #[serde(rename = "cacheId")]
    cache_id: CacheId<'a>,
    /// URL spec of the request.
    #[serde(rename = "requestURL")]
    request_url: Cow<'a, str>,
    /// headers of the request.
    #[serde(rename = "requestHeaders")]
    request_headers: Vec<Header<'a>>,
}

impl<'a> RequestCachedResponseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_id`: Id of cache that contains the entry.
    /// * `request_url`: URL spec of the request.
    /// * `request_headers`: headers of the request.
    pub fn builder(cache_id: impl Into<CacheId<'a>>, request_url: impl Into<Cow<'a, str>>, request_headers: Vec<Header<'a>>) -> RequestCachedResponseParamsBuilder<'a> {
        RequestCachedResponseParamsBuilder {
            cache_id: cache_id.into(),
            request_url: request_url.into(),
            request_headers: request_headers,
        }
    }
    /// Id of cache that contains the entry.
    pub fn cache_id(&self) -> &CacheId<'a> { &self.cache_id }
    /// URL spec of the request.
    pub fn request_url(&self) -> &str { self.request_url.as_ref() }
    /// headers of the request.
    pub fn request_headers(&self) -> &[Header<'a>] { &self.request_headers }
}


pub struct RequestCachedResponseParamsBuilder<'a> {
    cache_id: CacheId<'a>,
    request_url: Cow<'a, str>,
    request_headers: Vec<Header<'a>>,
}

impl<'a> RequestCachedResponseParamsBuilder<'a> {
    pub fn build(self) -> RequestCachedResponseParams<'a> {
        RequestCachedResponseParams {
            cache_id: self.cache_id,
            request_url: self.request_url,
            request_headers: self.request_headers,
        }
    }
}

/// Fetches cache entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestCachedResponseReturns<'a> {
    /// Response read from the cache.
    response: CachedResponse<'a>,
}

impl<'a> RequestCachedResponseReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `response`: Response read from the cache.
    pub fn builder(response: CachedResponse<'a>) -> RequestCachedResponseReturnsBuilder<'a> {
        RequestCachedResponseReturnsBuilder {
            response: response,
        }
    }
    /// Response read from the cache.
    pub fn response(&self) -> &CachedResponse<'a> { &self.response }
}


pub struct RequestCachedResponseReturnsBuilder<'a> {
    response: CachedResponse<'a>,
}

impl<'a> RequestCachedResponseReturnsBuilder<'a> {
    pub fn build(self) -> RequestCachedResponseReturns<'a> {
        RequestCachedResponseReturns {
            response: self.response,
        }
    }
}

impl<'a> RequestCachedResponseParams<'a> { pub const METHOD: &'static str = "CacheStorage.requestCachedResponse"; }

impl<'a> crate::CdpCommand<'a> for RequestCachedResponseParams<'a> {
    const METHOD: &'static str = "CacheStorage.requestCachedResponse";
    type Response = RequestCachedResponseReturns<'a>;
}

/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesParams<'a> {
    /// ID of cache to get entries from.
    #[serde(rename = "cacheId")]
    cache_id: CacheId<'a>,
    /// Number of records to skip.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipCount")]
    skip_count: Option<u64>,
    /// Number of records to fetch.
    #[serde(skip_serializing_if = "Option::is_none", rename = "pageSize")]
    page_size: Option<u64>,
    /// If present, only return the entries containing this substring in the path
    #[serde(skip_serializing_if = "Option::is_none", rename = "pathFilter")]
    path_filter: Option<Cow<'a, str>>,
}

impl<'a> RequestEntriesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_id`: ID of cache to get entries from.
    pub fn builder(cache_id: impl Into<CacheId<'a>>) -> RequestEntriesParamsBuilder<'a> {
        RequestEntriesParamsBuilder {
            cache_id: cache_id.into(),
            skip_count: None,
            page_size: None,
            path_filter: None,
        }
    }
    /// ID of cache to get entries from.
    pub fn cache_id(&self) -> &CacheId<'a> { &self.cache_id }
    /// Number of records to skip.
    pub fn skip_count(&self) -> Option<u64> { self.skip_count }
    /// Number of records to fetch.
    pub fn page_size(&self) -> Option<u64> { self.page_size }
    /// If present, only return the entries containing this substring in the path
    pub fn path_filter(&self) -> Option<&str> { self.path_filter.as_deref() }
}


pub struct RequestEntriesParamsBuilder<'a> {
    cache_id: CacheId<'a>,
    skip_count: Option<u64>,
    page_size: Option<u64>,
    path_filter: Option<Cow<'a, str>>,
}

impl<'a> RequestEntriesParamsBuilder<'a> {
    /// Number of records to skip.
    pub fn skip_count(mut self, skip_count: u64) -> Self { self.skip_count = Some(skip_count); self }
    /// Number of records to fetch.
    pub fn page_size(mut self, page_size: u64) -> Self { self.page_size = Some(page_size); self }
    /// If present, only return the entries containing this substring in the path
    pub fn path_filter(mut self, path_filter: impl Into<Cow<'a, str>>) -> Self { self.path_filter = Some(path_filter.into()); self }
    pub fn build(self) -> RequestEntriesParams<'a> {
        RequestEntriesParams {
            cache_id: self.cache_id,
            skip_count: self.skip_count,
            page_size: self.page_size,
            path_filter: self.path_filter,
        }
    }
}

/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesReturns<'a> {
    /// Array of object store data entries.
    #[serde(rename = "cacheDataEntries")]
    cache_data_entries: Vec<DataEntry<'a>>,
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.
    #[serde(rename = "returnCount")]
    return_count: f64,
}

impl<'a> RequestEntriesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cache_data_entries`: Array of object store data entries.
    /// * `return_count`: Count of returned entries from this storage. If pathFilter is empty, it is the count of all entries from this storage.
    pub fn builder(cache_data_entries: Vec<DataEntry<'a>>, return_count: f64) -> RequestEntriesReturnsBuilder<'a> {
        RequestEntriesReturnsBuilder {
            cache_data_entries: cache_data_entries,
            return_count: return_count,
        }
    }
    /// Array of object store data entries.
    pub fn cache_data_entries(&self) -> &[DataEntry<'a>] { &self.cache_data_entries }
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.
    pub fn return_count(&self) -> f64 { self.return_count }
}


pub struct RequestEntriesReturnsBuilder<'a> {
    cache_data_entries: Vec<DataEntry<'a>>,
    return_count: f64,
}

impl<'a> RequestEntriesReturnsBuilder<'a> {
    pub fn build(self) -> RequestEntriesReturns<'a> {
        RequestEntriesReturns {
            cache_data_entries: self.cache_data_entries,
            return_count: self.return_count,
        }
    }
}

impl<'a> RequestEntriesParams<'a> { pub const METHOD: &'static str = "CacheStorage.requestEntries"; }

impl<'a> crate::CdpCommand<'a> for RequestEntriesParams<'a> {
    const METHOD: &'static str = "CacheStorage.requestEntries";
    type Response = RequestEntriesReturns<'a>;
}
