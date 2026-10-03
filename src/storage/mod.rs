use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


pub type SerializedStorageKey<'a> = Cow<'a, str>;

/// Enum of possible storage types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageType {
    #[default]
    #[serde(rename = "cookies")]
    Cookies,
    #[serde(rename = "file_systems")]
    FileSystems,
    #[serde(rename = "indexeddb")]
    Indexeddb,
    #[serde(rename = "local_storage")]
    LocalStorage,
    #[serde(rename = "shader_cache")]
    ShaderCache,
    #[serde(rename = "websql")]
    Websql,
    #[serde(rename = "service_workers")]
    ServiceWorkers,
    #[serde(rename = "cache_storage")]
    CacheStorage,
    #[serde(rename = "storage_buckets")]
    StorageBuckets,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "other")]
    Other,
}

/// Usage for a storage type.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UsageForType {
    /// Name of storage type.
    #[serde(rename = "storageType")]
    pub storage_type: StorageType,
    /// Storage usage (bytes).
    pub usage: f64,
}
/// Pair of issuer origin and number of available (signed, but not used) Trust
/// Tokens from that issuer.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokens<'a> {
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: Cow<'a, str>,
    pub count: f64,
}
/// Details of a stored Private Verification Token.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PrivateVerificationToken<'a> {
    /// Unique identifier of the token in the database.
    pub id: Cow<'a, str>,
    /// Origin of the token issuer.
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: Cow<'a, str>,
    /// Public key ID used to issue the token.
    #[serde(rename = "keyId")]
    pub key_id: u64,
    /// Expiration timestamp in seconds since the epoch.
    pub expiration: crate::network::TimeSinceEpoch,
    /// Token creation timestamp in seconds since the epoch.
    #[serde(rename = "creationTime")]
    pub creation_time: crate::network::TimeSinceEpoch,
    /// Token protocol version.
    pub version: i64,
    /// Base64-encoded serialized token.
    pub token: Cow<'a, str>,
}
/// Configuration for a Private Verification Tokens issuer.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PrivateVerificationTokensIssuerConfig<'a> {
    /// Origin of the token issuer.
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: Cow<'a, str>,
    /// Origins authorized to redeem tokens from this issuer.
    #[serde(rename = "redeemerOrigins")]
    pub redeemer_origins: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageBucketsDurability {
    #[default]
    #[serde(rename = "relaxed")]
    Relaxed,
    #[serde(rename = "strict")]
    Strict,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucket<'a> {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey<'a>,
    /// If not specified, it is the default bucket of the storageKey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucketInfo<'a> {
    pub bucket: StorageBucket<'a>,
    pub id: Cow<'a, str>,
    pub expiration: crate::network::TimeSinceEpoch,
    /// Storage quota (bytes).
    pub quota: f64,
    pub persistent: bool,
    pub durability: StorageBucketsDurability,
}
/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getStorageKeyForFrame", response = "GetStorageKeyForFrameReturns<'a>")]
pub struct GetStorageKeyForFrameParams<'a> {
    #[serde(rename = "frameId")]
    pub frame_id: crate::page::FrameId<'a>,
}
/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameReturns<'a> {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey<'a>,
}
/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getStorageKey", response = "GetStorageKeyReturns<'a>")]
pub struct GetStorageKeyParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    pub frame_id: Option<crate::page::FrameId<'a>>,
}
/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyReturns<'a> {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey<'a>,
}
/// Clears storage for origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.clearDataForOrigin")]
pub struct ClearDataForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
    /// Comma separated list of StorageType to clear.
    #[serde(rename = "storageTypes")]
    pub storage_types: Cow<'a, str>,
}
/// Clears storage for storage key.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.clearDataForStorageKey")]
pub struct ClearDataForStorageKeyParams<'a> {
    /// Storage key.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Comma separated list of StorageType to clear.
    #[serde(rename = "storageTypes")]
    pub storage_types: Cow<'a, str>,
}
/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getCookies", response = "GetCookiesReturns<'a>")]
pub struct GetCookiesParams<'a> {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}
/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns<'a> {
    /// Array of cookie objects.
    pub cookies: Vec<crate::network::Cookie<'a>>,
}
/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.setCookies")]
pub struct SetCookiesParams<'a> {
    /// Cookies to be set.
    pub cookies: Vec<crate::network::CookieParam<'a>>,
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}
/// Clears cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.clearCookies")]
pub struct ClearCookiesParams<'a> {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    pub browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}
/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getUsageAndQuota", response = "GetUsageAndQuotaReturns")]
pub struct GetUsageAndQuotaParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
}
/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageAndQuotaReturns {
    /// Storage usage (bytes).
    pub usage: f64,
    /// Storage quota (bytes).
    pub quota: f64,
    /// Whether or not the origin has an active storage quota override
    #[serde(rename = "overrideActive")]
    pub override_active: bool,
    /// Storage usage per type (bytes).
    #[serde(rename = "usageBreakdown")]
    pub usage_breakdown: Vec<UsageForType>,
}
/// Override quota for the specified origin

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.overrideQuotaForOrigin")]
pub struct OverrideQuotaForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).
    #[serde(skip_serializing_if = "Option::is_none", rename = "quotaSize")]
    pub quota_size: Option<f64>,
}
/// Registers origin to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.trackCacheStorageForOrigin")]
pub struct TrackCacheStorageForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
}
/// Registers storage key to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.trackCacheStorageForStorageKey")]
pub struct TrackCacheStorageForStorageKeyParams<'a> {
    /// Storage key.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
}
/// Registers origin to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.trackIndexedDBForOrigin")]
pub struct TrackIndexedDBForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
}
/// Registers storage key to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.trackIndexedDBForStorageKey")]
pub struct TrackIndexedDBForStorageKeyParams<'a> {
    /// Storage key.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
}
/// Unregisters origin from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.untrackCacheStorageForOrigin")]
pub struct UntrackCacheStorageForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
}
/// Unregisters storage key from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.untrackCacheStorageForStorageKey")]
pub struct UntrackCacheStorageForStorageKeyParams<'a> {
    /// Storage key.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
}
/// Unregisters origin from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.untrackIndexedDBForOrigin")]
pub struct UntrackIndexedDBForOriginParams<'a> {
    /// Security origin.
    pub origin: Cow<'a, str>,
}
/// Unregisters storage key from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.untrackIndexedDBForStorageKey")]
pub struct UntrackIndexedDBForStorageKeyParams<'a> {
    /// Storage key.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
}
/// Returns the number of stored Trust Tokens per issuer for the
/// current browsing context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getTrustTokens", response = "GetTrustTokensReturns<'a>")]
pub struct GetTrustTokensParams {

}
/// Returns the number of stored Trust Tokens per issuer for the
/// current browsing context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetTrustTokensReturns<'a> {
    pub tokens: Vec<TrustTokens<'a>>,
}
/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.clearTrustTokens", response = "ClearTrustTokensReturns")]
pub struct ClearTrustTokensParams<'a> {
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: Cow<'a, str>,
}
/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensReturns {
    /// True if any tokens were deleted, false otherwise.
    #[serde(rename = "didDeleteTokens")]
    pub did_delete_tokens: bool,
}
/// Returns all stored Private Verification Tokens for the current browsing
/// context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getPrivateVerificationTokens", response = "GetPrivateVerificationTokensReturns<'a>")]
pub struct GetPrivateVerificationTokensParams {

}
/// Returns all stored Private Verification Tokens for the current browsing
/// context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPrivateVerificationTokensReturns<'a> {
    pub tokens: Vec<PrivateVerificationToken<'a>>,
}
/// Returns the configured Private Verification Tokens issuers and their redeemer
/// origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.getPrivateVerificationTokensIssuerConfigs", response = "GetPrivateVerificationTokensIssuerConfigsReturns<'a>")]
pub struct GetPrivateVerificationTokensIssuerConfigsParams {

}
/// Returns the configured Private Verification Tokens issuers and their redeemer
/// origins.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPrivateVerificationTokensIssuerConfigsReturns<'a> {
    pub configs: Vec<PrivateVerificationTokensIssuerConfig<'a>>,
}
/// Removes all Private Verification Tokens issued by the provided issuerOrigin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.clearPrivateVerificationTokens")]
pub struct ClearPrivateVerificationTokensParams<'a> {
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: Cow<'a, str>,
}
/// Removes a specific Private Verification Token by its ID.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.deletePrivateVerificationToken")]
pub struct DeletePrivateVerificationTokenParams<'a> {
    #[serde(rename = "tokenId")]
    pub token_id: Cow<'a, str>,
}
/// Set tracking for Private Verification Tokens.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.setPrivateVerificationTokensTracking")]
pub struct SetPrivateVerificationTokensTrackingParams {
    pub enable: bool,
}
/// Set tracking for a storage key's buckets.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.setStorageBucketTracking")]
pub struct SetStorageBucketTrackingParams<'a> {
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    pub enable: bool,
}
/// Deletes the Storage Bucket with the given storage key and bucket name.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.deleteStorageBucket")]
pub struct DeleteStorageBucketParams<'a> {
    pub bucket: StorageBucket<'a>,
}
/// Deletes state for sites identified as potential bounce trackers, immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.runBounceTrackingMitigations", response = "RunBounceTrackingMitigationsReturns<'a>")]
pub struct RunBounceTrackingMitigationsParams {

}
/// Deletes state for sites identified as potential bounce trackers, immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RunBounceTrackingMitigationsReturns<'a> {
    #[serde(rename = "deletedSites")]
    pub deleted_sites: Vec<Cow<'a, str>>,
}
/// A cache's contents have been modified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.cacheStorageContentUpdated")]
pub struct CacheStorageContentUpdated<'a> {
    /// Origin to update.
    pub origin: Cow<'a, str>,
    /// Storage key to update.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Storage bucket to update.
    #[serde(rename = "bucketId")]
    pub bucket_id: Cow<'a, str>,
    /// Name of cache in origin.
    #[serde(rename = "cacheName")]
    pub cache_name: Cow<'a, str>,
}
/// A cache has been added/deleted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.cacheStorageListUpdated")]
pub struct CacheStorageListUpdated<'a> {
    /// Origin to update.
    pub origin: Cow<'a, str>,
    /// Storage key to update.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Storage bucket to update.
    #[serde(rename = "bucketId")]
    pub bucket_id: Cow<'a, str>,
}
/// The origin's IndexedDB object store has been modified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.indexedDBContentUpdated")]
pub struct IndexedDBContentUpdated<'a> {
    /// Origin to update.
    pub origin: Cow<'a, str>,
    /// Storage key to update.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Storage bucket to update.
    #[serde(rename = "bucketId")]
    pub bucket_id: Cow<'a, str>,
    /// Database to update.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
    /// ObjectStore to update.
    #[serde(rename = "objectStoreName")]
    pub object_store_name: Cow<'a, str>,
}
/// The origin's IndexedDB database list has been modified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.indexedDBListUpdated")]
pub struct IndexedDBListUpdated<'a> {
    /// Origin to update.
    pub origin: Cow<'a, str>,
    /// Storage key to update.
    #[serde(rename = "storageKey")]
    pub storage_key: Cow<'a, str>,
    /// Storage bucket to update.
    #[serde(rename = "bucketId")]
    pub bucket_id: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.storageBucketCreatedOrUpdated")]
pub struct StorageBucketCreatedOrUpdated<'a> {
    #[serde(rename = "bucketInfo")]
    pub bucket_info: StorageBucketInfo<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.storageBucketDeleted")]
pub struct StorageBucketDeleted<'a> {
    #[serde(rename = "bucketId")]
    pub bucket_id: Cow<'a, str>,
}
/// Private Verification Tokens have been stored or deleted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Storage.privateVerificationTokensUpdated")]
pub struct PrivateVerificationTokensUpdated {

}