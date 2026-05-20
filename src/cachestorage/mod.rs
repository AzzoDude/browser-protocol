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
    requestURL: Cow<'a, str>,
    /// Request method.
    requestMethod: Cow<'a, str>,
    /// Request headers
    requestHeaders: Vec<Header<'a>>,
    /// Number of seconds since epoch.
    responseTime: f64,
    /// HTTP response status code.
    responseStatus: i64,
    /// HTTP response status text.
    responseStatusText: Cow<'a, str>,
    /// HTTP response type
    responseType: CachedResponseType,
    /// Response headers
    responseHeaders: Vec<Header<'a>>,
}

impl<'a> DataEntry<'a> {
    pub fn builder() -> DataEntryBuilder<'a> { DataEntryBuilder::default() }
    pub fn requestURL(&self) -> &str { self.requestURL.as_ref() }
    pub fn requestMethod(&self) -> &str { self.requestMethod.as_ref() }
    pub fn requestHeaders(&self) -> &[Header<'a>] { &self.requestHeaders }
    pub fn responseTime(&self) -> f64 { self.responseTime }
    pub fn responseStatus(&self) -> i64 { self.responseStatus }
    pub fn responseStatusText(&self) -> &str { self.responseStatusText.as_ref() }
    pub fn responseType(&self) -> &CachedResponseType { &self.responseType }
    pub fn responseHeaders(&self) -> &[Header<'a>] { &self.responseHeaders }
}

#[derive(Default)]
pub struct DataEntryBuilder<'a> {
    requestURL: Option<Cow<'a, str>>,
    requestMethod: Option<Cow<'a, str>>,
    requestHeaders: Option<Vec<Header<'a>>>,
    responseTime: Option<f64>,
    responseStatus: Option<i64>,
    responseStatusText: Option<Cow<'a, str>>,
    responseType: Option<CachedResponseType>,
    responseHeaders: Option<Vec<Header<'a>>>,
}

impl<'a> DataEntryBuilder<'a> {
    /// Request URL.
    pub fn requestURL(mut self, requestURL: impl Into<Cow<'a, str>>) -> Self { self.requestURL = Some(requestURL.into()); self }
    /// Request method.
    pub fn requestMethod(mut self, requestMethod: impl Into<Cow<'a, str>>) -> Self { self.requestMethod = Some(requestMethod.into()); self }
    /// Request headers
    pub fn requestHeaders(mut self, requestHeaders: Vec<Header<'a>>) -> Self { self.requestHeaders = Some(requestHeaders); self }
    /// Number of seconds since epoch.
    pub fn responseTime(mut self, responseTime: f64) -> Self { self.responseTime = Some(responseTime); self }
    /// HTTP response status code.
    pub fn responseStatus(mut self, responseStatus: i64) -> Self { self.responseStatus = Some(responseStatus); self }
    /// HTTP response status text.
    pub fn responseStatusText(mut self, responseStatusText: impl Into<Cow<'a, str>>) -> Self { self.responseStatusText = Some(responseStatusText.into()); self }
    /// HTTP response type
    pub fn responseType(mut self, responseType: CachedResponseType) -> Self { self.responseType = Some(responseType); self }
    /// Response headers
    pub fn responseHeaders(mut self, responseHeaders: Vec<Header<'a>>) -> Self { self.responseHeaders = Some(responseHeaders); self }
    pub fn build(self) -> DataEntry<'a> {
        DataEntry {
            requestURL: self.requestURL.unwrap_or_default(),
            requestMethod: self.requestMethod.unwrap_or_default(),
            requestHeaders: self.requestHeaders.unwrap_or_default(),
            responseTime: self.responseTime.unwrap_or_default(),
            responseStatus: self.responseStatus.unwrap_or_default(),
            responseStatusText: self.responseStatusText.unwrap_or_default(),
            responseType: self.responseType.unwrap_or_default(),
            responseHeaders: self.responseHeaders.unwrap_or_default(),
        }
    }
}

/// Cache identifier.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cache<'a> {
    /// An opaque unique id of the cache.
    cacheId: CacheId<'a>,
    /// Security origin of the cache.
    securityOrigin: Cow<'a, str>,
    /// Storage key of the cache.
    storageKey: Cow<'a, str>,
    /// Storage bucket of the cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// The name of the cache.
    cacheName: Cow<'a, str>,
}

impl<'a> Cache<'a> {
    pub fn builder() -> CacheBuilder<'a> { CacheBuilder::default() }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn securityOrigin(&self) -> &str { self.securityOrigin.as_ref() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn cacheName(&self) -> &str { self.cacheName.as_ref() }
}

#[derive(Default)]
pub struct CacheBuilder<'a> {
    cacheId: Option<CacheId<'a>>,
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    cacheName: Option<Cow<'a, str>>,
}

impl<'a> CacheBuilder<'a> {
    /// An opaque unique id of the cache.
    pub fn cacheId(mut self, cacheId: CacheId<'a>) -> Self { self.cacheId = Some(cacheId); self }
    /// Security origin of the cache.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key of the cache.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket of the cache.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// The name of the cache.
    pub fn cacheName(mut self, cacheName: impl Into<Cow<'a, str>>) -> Self { self.cacheName = Some(cacheName.into()); self }
    pub fn build(self) -> Cache<'a> {
        Cache {
            cacheId: self.cacheId.unwrap_or_default(),
            securityOrigin: self.securityOrigin.unwrap_or_default(),
            storageKey: self.storageKey.unwrap_or_default(),
            storageBucket: self.storageBucket,
            cacheName: self.cacheName.unwrap_or_default(),
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
    pub fn builder() -> HeaderBuilder<'a> { HeaderBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct HeaderBuilder<'a> {
    name: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> HeaderBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> Header<'a> {
        Header {
            name: self.name.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> CachedResponseBuilder<'a> { CachedResponseBuilder::default() }
    pub fn body(&self) -> &str { self.body.as_ref() }
}

#[derive(Default)]
pub struct CachedResponseBuilder<'a> {
    body: Option<Cow<'a, str>>,
}

impl<'a> CachedResponseBuilder<'a> {
    /// Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self { self.body = Some(body.into()); self }
    pub fn build(self) -> CachedResponse<'a> {
        CachedResponse {
            body: self.body.unwrap_or_default(),
        }
    }
}

/// Deletes a cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCacheParams<'a> {
    /// Id of cache for deletion.
    cacheId: CacheId<'a>,
}

impl<'a> DeleteCacheParams<'a> {
    pub fn builder() -> DeleteCacheParamsBuilder<'a> { DeleteCacheParamsBuilder::default() }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
}

#[derive(Default)]
pub struct DeleteCacheParamsBuilder<'a> {
    cacheId: Option<CacheId<'a>>,
}

impl<'a> DeleteCacheParamsBuilder<'a> {
    /// Id of cache for deletion.
    pub fn cacheId(mut self, cacheId: CacheId<'a>) -> Self { self.cacheId = Some(cacheId); self }
    pub fn build(self) -> DeleteCacheParams<'a> {
        DeleteCacheParams {
            cacheId: self.cacheId.unwrap_or_default(),
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
    cacheId: CacheId<'a>,
    /// URL spec of the request.
    request: Cow<'a, str>,
}

impl<'a> DeleteEntryParams<'a> {
    pub fn builder() -> DeleteEntryParamsBuilder<'a> { DeleteEntryParamsBuilder::default() }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn request(&self) -> &str { self.request.as_ref() }
}

#[derive(Default)]
pub struct DeleteEntryParamsBuilder<'a> {
    cacheId: Option<CacheId<'a>>,
    request: Option<Cow<'a, str>>,
}

impl<'a> DeleteEntryParamsBuilder<'a> {
    /// Id of cache where the entry will be deleted.
    pub fn cacheId(mut self, cacheId: CacheId<'a>) -> Self { self.cacheId = Some(cacheId); self }
    /// URL spec of the request.
    pub fn request(mut self, request: impl Into<Cow<'a, str>>) -> Self { self.request = Some(request.into()); self }
    pub fn build(self) -> DeleteEntryParams<'a> {
        DeleteEntryParams {
            cacheId: self.cacheId.unwrap_or_default(),
            request: self.request.unwrap_or_default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestCacheNamesParams<'a> {
    pub fn builder() -> RequestCacheNamesParamsBuilder<'a> { RequestCacheNamesParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
}

#[derive(Default)]
pub struct RequestCacheNamesParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestCacheNamesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> RequestCacheNamesParams<'a> {
        RequestCacheNamesParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
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
    pub fn builder() -> RequestCacheNamesReturnsBuilder<'a> { RequestCacheNamesReturnsBuilder::default() }
    pub fn caches(&self) -> &[Cache<'a>] { &self.caches }
}

#[derive(Default)]
pub struct RequestCacheNamesReturnsBuilder<'a> {
    caches: Option<Vec<Cache<'a>>>,
}

impl<'a> RequestCacheNamesReturnsBuilder<'a> {
    /// Caches for the security origin.
    pub fn caches(mut self, caches: Vec<Cache<'a>>) -> Self { self.caches = Some(caches); self }
    pub fn build(self) -> RequestCacheNamesReturns<'a> {
        RequestCacheNamesReturns {
            caches: self.caches.unwrap_or_default(),
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
    cacheId: CacheId<'a>,
    /// URL spec of the request.
    requestURL: Cow<'a, str>,
    /// headers of the request.
    requestHeaders: Vec<Header<'a>>,
}

impl<'a> RequestCachedResponseParams<'a> {
    pub fn builder() -> RequestCachedResponseParamsBuilder<'a> { RequestCachedResponseParamsBuilder::default() }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn requestURL(&self) -> &str { self.requestURL.as_ref() }
    pub fn requestHeaders(&self) -> &[Header<'a>] { &self.requestHeaders }
}

#[derive(Default)]
pub struct RequestCachedResponseParamsBuilder<'a> {
    cacheId: Option<CacheId<'a>>,
    requestURL: Option<Cow<'a, str>>,
    requestHeaders: Option<Vec<Header<'a>>>,
}

impl<'a> RequestCachedResponseParamsBuilder<'a> {
    /// Id of cache that contains the entry.
    pub fn cacheId(mut self, cacheId: CacheId<'a>) -> Self { self.cacheId = Some(cacheId); self }
    /// URL spec of the request.
    pub fn requestURL(mut self, requestURL: impl Into<Cow<'a, str>>) -> Self { self.requestURL = Some(requestURL.into()); self }
    /// headers of the request.
    pub fn requestHeaders(mut self, requestHeaders: Vec<Header<'a>>) -> Self { self.requestHeaders = Some(requestHeaders); self }
    pub fn build(self) -> RequestCachedResponseParams<'a> {
        RequestCachedResponseParams {
            cacheId: self.cacheId.unwrap_or_default(),
            requestURL: self.requestURL.unwrap_or_default(),
            requestHeaders: self.requestHeaders.unwrap_or_default(),
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
    pub fn builder() -> RequestCachedResponseReturnsBuilder<'a> { RequestCachedResponseReturnsBuilder::default() }
    pub fn response(&self) -> &CachedResponse<'a> { &self.response }
}

#[derive(Default)]
pub struct RequestCachedResponseReturnsBuilder<'a> {
    response: Option<CachedResponse<'a>>,
}

impl<'a> RequestCachedResponseReturnsBuilder<'a> {
    /// Response read from the cache.
    pub fn response(mut self, response: CachedResponse<'a>) -> Self { self.response = Some(response); self }
    pub fn build(self) -> RequestCachedResponseReturns<'a> {
        RequestCachedResponseReturns {
            response: self.response.unwrap_or_default(),
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
    cacheId: CacheId<'a>,
    /// Number of records to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    skipCount: Option<u64>,
    /// Number of records to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pageSize: Option<u64>,
    /// If present, only return the entries containing this substring in the path
    #[serde(skip_serializing_if = "Option::is_none")]
    pathFilter: Option<Cow<'a, str>>,
}

impl<'a> RequestEntriesParams<'a> {
    pub fn builder() -> RequestEntriesParamsBuilder<'a> { RequestEntriesParamsBuilder::default() }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn skipCount(&self) -> Option<u64> { self.skipCount }
    pub fn pageSize(&self) -> Option<u64> { self.pageSize }
    pub fn pathFilter(&self) -> Option<&str> { self.pathFilter.as_deref() }
}

#[derive(Default)]
pub struct RequestEntriesParamsBuilder<'a> {
    cacheId: Option<CacheId<'a>>,
    skipCount: Option<u64>,
    pageSize: Option<u64>,
    pathFilter: Option<Cow<'a, str>>,
}

impl<'a> RequestEntriesParamsBuilder<'a> {
    /// ID of cache to get entries from.
    pub fn cacheId(mut self, cacheId: CacheId<'a>) -> Self { self.cacheId = Some(cacheId); self }
    /// Number of records to skip.
    pub fn skipCount(mut self, skipCount: u64) -> Self { self.skipCount = Some(skipCount); self }
    /// Number of records to fetch.
    pub fn pageSize(mut self, pageSize: u64) -> Self { self.pageSize = Some(pageSize); self }
    /// If present, only return the entries containing this substring in the path
    pub fn pathFilter(mut self, pathFilter: impl Into<Cow<'a, str>>) -> Self { self.pathFilter = Some(pathFilter.into()); self }
    pub fn build(self) -> RequestEntriesParams<'a> {
        RequestEntriesParams {
            cacheId: self.cacheId.unwrap_or_default(),
            skipCount: self.skipCount,
            pageSize: self.pageSize,
            pathFilter: self.pathFilter,
        }
    }
}

/// Requests data from cache.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestEntriesReturns<'a> {
    /// Array of object store data entries.
    cacheDataEntries: Vec<DataEntry<'a>>,
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.
    returnCount: f64,
}

impl<'a> RequestEntriesReturns<'a> {
    pub fn builder() -> RequestEntriesReturnsBuilder<'a> { RequestEntriesReturnsBuilder::default() }
    pub fn cacheDataEntries(&self) -> &[DataEntry<'a>] { &self.cacheDataEntries }
    pub fn returnCount(&self) -> f64 { self.returnCount }
}

#[derive(Default)]
pub struct RequestEntriesReturnsBuilder<'a> {
    cacheDataEntries: Option<Vec<DataEntry<'a>>>,
    returnCount: Option<f64>,
}

impl<'a> RequestEntriesReturnsBuilder<'a> {
    /// Array of object store data entries.
    pub fn cacheDataEntries(mut self, cacheDataEntries: Vec<DataEntry<'a>>) -> Self { self.cacheDataEntries = Some(cacheDataEntries); self }
    /// Count of returned entries from this storage. If pathFilter is empty, it
    /// is the count of all entries from this storage.
    pub fn returnCount(mut self, returnCount: f64) -> Self { self.returnCount = Some(returnCount); self }
    pub fn build(self) -> RequestEntriesReturns<'a> {
        RequestEntriesReturns {
            cacheDataEntries: self.cacheDataEntries.unwrap_or_default(),
            returnCount: self.returnCount.unwrap_or_default(),
        }
    }
}

impl<'a> RequestEntriesParams<'a> { pub const METHOD: &'static str = "CacheStorage.requestEntries"; }

impl<'a> crate::CdpCommand<'a> for RequestEntriesParams<'a> {
    const METHOD: &'static str = "CacheStorage.requestEntries";
    type Response = RequestEntriesReturns<'a>;
}
