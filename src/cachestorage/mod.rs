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
    pub fn builder(requestURL: impl Into<Cow<'a, str>>, requestMethod: impl Into<Cow<'a, str>>, requestHeaders: Vec<Header<'a>>, responseTime: f64, responseStatus: i64, responseStatusText: impl Into<Cow<'a, str>>, responseType: impl Into<CachedResponseType>, responseHeaders: Vec<Header<'a>>) -> DataEntryBuilder<'a> {
        DataEntryBuilder {
            requestURL: requestURL.into(),
            requestMethod: requestMethod.into(),
            requestHeaders: requestHeaders,
            responseTime: responseTime,
            responseStatus: responseStatus,
            responseStatusText: responseStatusText.into(),
            responseType: responseType.into(),
            responseHeaders: responseHeaders,
        }
    }
    pub fn requestURL(&self) -> &str { self.requestURL.as_ref() }
    pub fn requestMethod(&self) -> &str { self.requestMethod.as_ref() }
    pub fn requestHeaders(&self) -> &[Header<'a>] { &self.requestHeaders }
    pub fn responseTime(&self) -> f64 { self.responseTime }
    pub fn responseStatus(&self) -> i64 { self.responseStatus }
    pub fn responseStatusText(&self) -> &str { self.responseStatusText.as_ref() }
    pub fn responseType(&self) -> &CachedResponseType { &self.responseType }
    pub fn responseHeaders(&self) -> &[Header<'a>] { &self.responseHeaders }
}


pub struct DataEntryBuilder<'a> {
    requestURL: Cow<'a, str>,
    requestMethod: Cow<'a, str>,
    requestHeaders: Vec<Header<'a>>,
    responseTime: f64,
    responseStatus: i64,
    responseStatusText: Cow<'a, str>,
    responseType: CachedResponseType,
    responseHeaders: Vec<Header<'a>>,
}

impl<'a> DataEntryBuilder<'a> {
    pub fn build(self) -> DataEntry<'a> {
        DataEntry {
            requestURL: self.requestURL,
            requestMethod: self.requestMethod,
            requestHeaders: self.requestHeaders,
            responseTime: self.responseTime,
            responseStatus: self.responseStatus,
            responseStatusText: self.responseStatusText,
            responseType: self.responseType,
            responseHeaders: self.responseHeaders,
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
    pub fn builder(cacheId: impl Into<CacheId<'a>>, securityOrigin: impl Into<Cow<'a, str>>, storageKey: impl Into<Cow<'a, str>>, cacheName: impl Into<Cow<'a, str>>) -> CacheBuilder<'a> {
        CacheBuilder {
            cacheId: cacheId.into(),
            securityOrigin: securityOrigin.into(),
            storageKey: storageKey.into(),
            storageBucket: None,
            cacheName: cacheName.into(),
        }
    }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn securityOrigin(&self) -> &str { self.securityOrigin.as_ref() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn cacheName(&self) -> &str { self.cacheName.as_ref() }
}


pub struct CacheBuilder<'a> {
    cacheId: CacheId<'a>,
    securityOrigin: Cow<'a, str>,
    storageKey: Cow<'a, str>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    cacheName: Cow<'a, str>,
}

impl<'a> CacheBuilder<'a> {
    /// Storage bucket of the cache.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> Cache<'a> {
        Cache {
            cacheId: self.cacheId,
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            cacheName: self.cacheName,
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
    pub fn builder(body: impl Into<Cow<'a, str>>) -> CachedResponseBuilder<'a> {
        CachedResponseBuilder {
            body: body.into(),
        }
    }
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
    cacheId: CacheId<'a>,
}

impl<'a> DeleteCacheParams<'a> {
    pub fn builder(cacheId: impl Into<CacheId<'a>>) -> DeleteCacheParamsBuilder<'a> {
        DeleteCacheParamsBuilder {
            cacheId: cacheId.into(),
        }
    }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
}


pub struct DeleteCacheParamsBuilder<'a> {
    cacheId: CacheId<'a>,
}

impl<'a> DeleteCacheParamsBuilder<'a> {
    pub fn build(self) -> DeleteCacheParams<'a> {
        DeleteCacheParams {
            cacheId: self.cacheId,
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
    pub fn builder(cacheId: impl Into<CacheId<'a>>, request: impl Into<Cow<'a, str>>) -> DeleteEntryParamsBuilder<'a> {
        DeleteEntryParamsBuilder {
            cacheId: cacheId.into(),
            request: request.into(),
        }
    }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn request(&self) -> &str { self.request.as_ref() }
}


pub struct DeleteEntryParamsBuilder<'a> {
    cacheId: CacheId<'a>,
    request: Cow<'a, str>,
}

impl<'a> DeleteEntryParamsBuilder<'a> {
    pub fn build(self) -> DeleteEntryParams<'a> {
        DeleteEntryParams {
            cacheId: self.cacheId,
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
    pub fn builder() -> RequestCacheNamesParamsBuilder<'a> {
        RequestCacheNamesParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
        }
    }
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
    pub fn builder(caches: Vec<Cache<'a>>) -> RequestCacheNamesReturnsBuilder<'a> {
        RequestCacheNamesReturnsBuilder {
            caches: caches,
        }
    }
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
    cacheId: CacheId<'a>,
    /// URL spec of the request.
    requestURL: Cow<'a, str>,
    /// headers of the request.
    requestHeaders: Vec<Header<'a>>,
}

impl<'a> RequestCachedResponseParams<'a> {
    pub fn builder(cacheId: impl Into<CacheId<'a>>, requestURL: impl Into<Cow<'a, str>>, requestHeaders: Vec<Header<'a>>) -> RequestCachedResponseParamsBuilder<'a> {
        RequestCachedResponseParamsBuilder {
            cacheId: cacheId.into(),
            requestURL: requestURL.into(),
            requestHeaders: requestHeaders,
        }
    }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn requestURL(&self) -> &str { self.requestURL.as_ref() }
    pub fn requestHeaders(&self) -> &[Header<'a>] { &self.requestHeaders }
}


pub struct RequestCachedResponseParamsBuilder<'a> {
    cacheId: CacheId<'a>,
    requestURL: Cow<'a, str>,
    requestHeaders: Vec<Header<'a>>,
}

impl<'a> RequestCachedResponseParamsBuilder<'a> {
    pub fn build(self) -> RequestCachedResponseParams<'a> {
        RequestCachedResponseParams {
            cacheId: self.cacheId,
            requestURL: self.requestURL,
            requestHeaders: self.requestHeaders,
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
    pub fn builder(response: CachedResponse<'a>) -> RequestCachedResponseReturnsBuilder<'a> {
        RequestCachedResponseReturnsBuilder {
            response: response,
        }
    }
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
    pub fn builder(cacheId: impl Into<CacheId<'a>>) -> RequestEntriesParamsBuilder<'a> {
        RequestEntriesParamsBuilder {
            cacheId: cacheId.into(),
            skipCount: None,
            pageSize: None,
            pathFilter: None,
        }
    }
    pub fn cacheId(&self) -> &CacheId<'a> { &self.cacheId }
    pub fn skipCount(&self) -> Option<u64> { self.skipCount }
    pub fn pageSize(&self) -> Option<u64> { self.pageSize }
    pub fn pathFilter(&self) -> Option<&str> { self.pathFilter.as_deref() }
}


pub struct RequestEntriesParamsBuilder<'a> {
    cacheId: CacheId<'a>,
    skipCount: Option<u64>,
    pageSize: Option<u64>,
    pathFilter: Option<Cow<'a, str>>,
}

impl<'a> RequestEntriesParamsBuilder<'a> {
    /// Number of records to skip.
    pub fn skipCount(mut self, skipCount: u64) -> Self { self.skipCount = Some(skipCount); self }
    /// Number of records to fetch.
    pub fn pageSize(mut self, pageSize: u64) -> Self { self.pageSize = Some(pageSize); self }
    /// If present, only return the entries containing this substring in the path
    pub fn pathFilter(mut self, pathFilter: impl Into<Cow<'a, str>>) -> Self { self.pathFilter = Some(pathFilter.into()); self }
    pub fn build(self) -> RequestEntriesParams<'a> {
        RequestEntriesParams {
            cacheId: self.cacheId,
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
    pub fn builder(cacheDataEntries: Vec<DataEntry<'a>>, returnCount: f64) -> RequestEntriesReturnsBuilder<'a> {
        RequestEntriesReturnsBuilder {
            cacheDataEntries: cacheDataEntries,
            returnCount: returnCount,
        }
    }
    pub fn cacheDataEntries(&self) -> &[DataEntry<'a>] { &self.cacheDataEntries }
    pub fn returnCount(&self) -> f64 { self.returnCount }
}


pub struct RequestEntriesReturnsBuilder<'a> {
    cacheDataEntries: Vec<DataEntry<'a>>,
    returnCount: f64,
}

impl<'a> RequestEntriesReturnsBuilder<'a> {
    pub fn build(self) -> RequestEntriesReturns<'a> {
        RequestEntriesReturns {
            cacheDataEntries: self.cacheDataEntries,
            returnCount: self.returnCount,
        }
    }
}

impl<'a> RequestEntriesParams<'a> { pub const METHOD: &'static str = "CacheStorage.requestEntries"; }

impl<'a> crate::CdpCommand<'a> for RequestEntriesParams<'a> {
    const METHOD: &'static str = "CacheStorage.requestEntries";
    type Response = RequestEntriesReturns<'a>;
}
