use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;


pub type SerializedStorageKey = String;

/// Enum of possible storage types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageType {
    #[default]
    Cookies,
    FileSystems,
    Indexeddb,
    LocalStorage,
    ShaderCache,
    Websql,
    ServiceWorkers,
    CacheStorage,
    InterestGroups,
    SharedStorage,
    StorageBuckets,
    All,
    Other,
}

/// Usage for a storage type.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageForType {
    /// Name of storage type.

    pub storageType: StorageType,
    /// Storage usage (bytes).

    pub usage: f64,
}

/// Pair of issuer origin and number of available (signed, but not used) Trust
/// Tokens from that issuer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokens {

    pub issuerOrigin: String,

    pub count: f64,
}

/// Protected audience interest group auction identifier.

pub type InterestGroupAuctionId = String;

/// Enum of interest group access types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAccessType {
    #[default]
    Join,
    Leave,
    Update,
    Loaded,
    Bid,
    Win,
    AdditionalBid,
    AdditionalBidWin,
    TopLevelBid,
    TopLevelAdditionalBid,
    Clear,
}

/// Enum of auction events.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAuctionEventType {
    #[default]
    Started,
    ConfigResolved,
}

/// Enum of network fetches auctions can do.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAuctionFetchType {
    #[default]
    BidderJs,
    BidderWasm,
    SellerJs,
    BidderTrustedSignals,
    SellerTrustedSignals,
}

/// Enum of shared storage access scopes.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedStorageAccessScope {
    #[default]
    Window,
    SharedStorageWorklet,
    ProtectedAudienceWorklet,
    Header,
}

/// Enum of shared storage access methods.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedStorageAccessMethod {
    #[default]
    AddModule,
    CreateWorklet,
    SelectURL,
    Run,
    BatchUpdate,
    Set,
    Append,
    Delete,
    Clear,
    Get,
    Keys,
    Values,
    Entries,
    Length,
    RemainingBudget,
}

/// Struct for a single key-value pair in an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageEntry {

    pub key: String,

    pub value: String,
}

/// Details for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageMetadata {
    /// Time when the origin's shared storage was last created.

    pub creationTime: crate::network::TimeSinceEpoch,
    /// Number of key-value pairs stored in origin's shared storage.

    pub length: u64,
    /// Current amount of bits of entropy remaining in the navigation budget.

    pub remainingBudget: f64,
    /// Total number of bytes stored as key-value pairs in origin's shared
    /// storage.

    pub bytesUsed: i64,
}

/// Represents a dictionary object passed in as privateAggregationConfig to
/// run or selectURL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStoragePrivateAggregationConfig {
    /// The chosen aggregation service deployment.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregationCoordinatorOrigin: Option<String>,
    /// The context ID provided.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextId: Option<String>,
    /// Configures the maximum size allowed for filtering IDs.

    pub filteringIdMaxBytes: u64,
    /// The limit on the number of contributions in the final report.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxContributions: Option<i64>,
}

/// Pair of reporting metadata details for a candidate URL for 'selectURL()'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageReportingMetadata {

    pub eventType: String,

    pub reportingUrl: String,
}

/// Bundles a candidate URL with its reporting metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageUrlWithMetadata {
    /// Spec of candidate URL.

    pub url: String,
    /// Any associated reporting metadata.

    pub reportingMetadata: Vec<SharedStorageReportingMetadata>,
}

/// Bundles the parameters for shared storage access events whose
/// presence/absence can vary according to SharedStorageAccessType.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageAccessParams {
    /// Spec of the module script URL.
    /// Present only for SharedStorageAccessMethods: addModule and
    /// createWorklet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptSourceUrl: Option<String>,
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataOrigin: Option<String>,
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operationName: Option<String>,
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operationId: Option<String>,
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepAlive: Option<bool>,
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privateAggregationConfig: Option<SharedStoragePrivateAggregationConfig>,
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializedData: Option<String>,
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlsWithMetadata: Option<Vec<SharedStorageUrlWithMetadata>>,
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urnUuid: Option<String>,
    /// Key for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set, append, delete, and
    /// get.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Value for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set and append.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whether or not to set an entry for a key if that key is already present.
    /// Present only for SharedStorageAccessMethod: set.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignoreIfPresent: Option<bool>,
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workletOrdinal: Option<i64>,
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub workletTargetId: Option<crate::target::TargetID>,
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub withLock: Option<String>,
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchUpdateId: Option<String>,
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batchSize: Option<u64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageBucketsDurability {
    #[default]
    Relaxed,
    Strict,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucket {

    pub storageKey: SerializedStorageKey,
    /// If not specified, it is the default bucket of the storageKey.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucketInfo {

    pub bucket: StorageBucket,

    pub id: String,

    pub expiration: crate::network::TimeSinceEpoch,
    /// Storage quota (bytes).

    pub quota: f64,

    pub persistent: bool,

    pub durability: StorageBucketsDurability,
}

/// A single Related Website Set object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedWebsiteSet {
    /// The primary site of this set, along with the ccTLDs if there is any.

    pub primarySites: Vec<String>,
    /// The associated sites of this set, along with the ccTLDs if there is any.

    pub associatedSites: Vec<String>,
    /// The service sites of this set, along with the ccTLDs if there is any.

    pub serviceSites: Vec<String>,
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameParams {

    pub frameId: crate::page::FrameId,
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameReturns {

    pub storageKey: SerializedStorageKey,
}

impl GetStorageKeyForFrameParams { pub const METHOD: &'static str = "Storage.getStorageKeyForFrame"; }

impl crate::CdpCommand for GetStorageKeyForFrameParams {
    const METHOD: &'static str = "Storage.getStorageKeyForFrame";
    type Response = GetStorageKeyForFrameReturns;
}

/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameId: Option<crate::page::FrameId>,
}

/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyReturns {

    pub storageKey: SerializedStorageKey,
}

impl GetStorageKeyParams { pub const METHOD: &'static str = "Storage.getStorageKey"; }

impl crate::CdpCommand for GetStorageKeyParams {
    const METHOD: &'static str = "Storage.getStorageKey";
    type Response = GetStorageKeyReturns;
}

/// Clears storage for origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForOriginParams {
    /// Security origin.

    pub origin: String,
    /// Comma separated list of StorageType to clear.

    pub storageTypes: String,
}

impl ClearDataForOriginParams { pub const METHOD: &'static str = "Storage.clearDataForOrigin"; }

impl crate::CdpCommand for ClearDataForOriginParams {
    const METHOD: &'static str = "Storage.clearDataForOrigin";
    type Response = crate::EmptyReturns;
}

/// Clears storage for storage key.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForStorageKeyParams {
    /// Storage key.

    pub storageKey: String,
    /// Comma separated list of StorageType to clear.

    pub storageTypes: String,
}

impl ClearDataForStorageKeyParams { pub const METHOD: &'static str = "Storage.clearDataForStorageKey"; }

impl crate::CdpCommand for ClearDataForStorageKeyParams {
    const METHOD: &'static str = "Storage.clearDataForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams {
    /// Browser context to use when called on the browser endpoint.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<crate::browser::BrowserContextID>,
}

/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns {
    /// Array of cookie objects.

    pub cookies: Vec<crate::network::Cookie>,
}

impl GetCookiesParams { pub const METHOD: &'static str = "Storage.getCookies"; }

impl crate::CdpCommand for GetCookiesParams {
    const METHOD: &'static str = "Storage.getCookies";
    type Response = GetCookiesReturns;
}

/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams {
    /// Cookies to be set.

    pub cookies: Vec<crate::network::CookieParam>,
    /// Browser context to use when called on the browser endpoint.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<crate::browser::BrowserContextID>,
}

impl SetCookiesParams { pub const METHOD: &'static str = "Storage.setCookies"; }

impl crate::CdpCommand for SetCookiesParams {
    const METHOD: &'static str = "Storage.setCookies";
    type Response = crate::EmptyReturns;
}

/// Clears cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCookiesParams {
    /// Browser context to use when called on the browser endpoint.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browserContextId: Option<crate::browser::BrowserContextID>,
}

impl ClearCookiesParams { pub const METHOD: &'static str = "Storage.clearCookies"; }

impl crate::CdpCommand for ClearCookiesParams {
    const METHOD: &'static str = "Storage.clearCookies";
    type Response = crate::EmptyReturns;
}

/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageAndQuotaParams {
    /// Security origin.

    pub origin: String,
}

/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageAndQuotaReturns {
    /// Storage usage (bytes).

    pub usage: f64,
    /// Storage quota (bytes).

    pub quota: f64,
    /// Whether or not the origin has an active storage quota override

    pub overrideActive: bool,
    /// Storage usage per type (bytes).

    pub usageBreakdown: Vec<UsageForType>,
}

impl GetUsageAndQuotaParams { pub const METHOD: &'static str = "Storage.getUsageAndQuota"; }

impl crate::CdpCommand for GetUsageAndQuotaParams {
    const METHOD: &'static str = "Storage.getUsageAndQuota";
    type Response = GetUsageAndQuotaReturns;
}

/// Override quota for the specified origin

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverrideQuotaForOriginParams {
    /// Security origin.

    pub origin: String,
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotaSize: Option<f64>,
}

impl OverrideQuotaForOriginParams { pub const METHOD: &'static str = "Storage.overrideQuotaForOrigin"; }

impl crate::CdpCommand for OverrideQuotaForOriginParams {
    const METHOD: &'static str = "Storage.overrideQuotaForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers origin to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForOriginParams {
    /// Security origin.

    pub origin: String,
}

impl TrackCacheStorageForOriginParams { pub const METHOD: &'static str = "Storage.trackCacheStorageForOrigin"; }

impl crate::CdpCommand for TrackCacheStorageForOriginParams {
    const METHOD: &'static str = "Storage.trackCacheStorageForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers storage key to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForStorageKeyParams {
    /// Storage key.

    pub storageKey: String,
}

impl TrackCacheStorageForStorageKeyParams { pub const METHOD: &'static str = "Storage.trackCacheStorageForStorageKey"; }

impl crate::CdpCommand for TrackCacheStorageForStorageKeyParams {
    const METHOD: &'static str = "Storage.trackCacheStorageForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Registers origin to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForOriginParams {
    /// Security origin.

    pub origin: String,
}

impl TrackIndexedDBForOriginParams { pub const METHOD: &'static str = "Storage.trackIndexedDBForOrigin"; }

impl crate::CdpCommand for TrackIndexedDBForOriginParams {
    const METHOD: &'static str = "Storage.trackIndexedDBForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers storage key to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForStorageKeyParams {
    /// Storage key.

    pub storageKey: String,
}

impl TrackIndexedDBForStorageKeyParams { pub const METHOD: &'static str = "Storage.trackIndexedDBForStorageKey"; }

impl crate::CdpCommand for TrackIndexedDBForStorageKeyParams {
    const METHOD: &'static str = "Storage.trackIndexedDBForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Unregisters origin from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForOriginParams {
    /// Security origin.

    pub origin: String,
}

impl UntrackCacheStorageForOriginParams { pub const METHOD: &'static str = "Storage.untrackCacheStorageForOrigin"; }

impl crate::CdpCommand for UntrackCacheStorageForOriginParams {
    const METHOD: &'static str = "Storage.untrackCacheStorageForOrigin";
    type Response = crate::EmptyReturns;
}

/// Unregisters storage key from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForStorageKeyParams {
    /// Storage key.

    pub storageKey: String,
}

impl UntrackCacheStorageForStorageKeyParams { pub const METHOD: &'static str = "Storage.untrackCacheStorageForStorageKey"; }

impl crate::CdpCommand for UntrackCacheStorageForStorageKeyParams {
    const METHOD: &'static str = "Storage.untrackCacheStorageForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Unregisters origin from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForOriginParams {
    /// Security origin.

    pub origin: String,
}

impl UntrackIndexedDBForOriginParams { pub const METHOD: &'static str = "Storage.untrackIndexedDBForOrigin"; }

impl crate::CdpCommand for UntrackIndexedDBForOriginParams {
    const METHOD: &'static str = "Storage.untrackIndexedDBForOrigin";
    type Response = crate::EmptyReturns;
}

/// Unregisters storage key from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForStorageKeyParams {
    /// Storage key.

    pub storageKey: String,
}

impl UntrackIndexedDBForStorageKeyParams { pub const METHOD: &'static str = "Storage.untrackIndexedDBForStorageKey"; }

impl crate::CdpCommand for UntrackIndexedDBForStorageKeyParams {
    const METHOD: &'static str = "Storage.untrackIndexedDBForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Returns the number of stored Trust Tokens per issuer for the
/// current browsing context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrustTokensReturns {

    pub tokens: Vec<TrustTokens>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTrustTokensParams {}

impl GetTrustTokensParams { pub const METHOD: &'static str = "Storage.getTrustTokens"; }

impl crate::CdpCommand for GetTrustTokensParams {
    const METHOD: &'static str = "Storage.getTrustTokens";
    type Response = GetTrustTokensReturns;
}

/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensParams {

    pub issuerOrigin: String,
}

/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensReturns {
    /// True if any tokens were deleted, false otherwise.

    pub didDeleteTokens: bool,
}

impl ClearTrustTokensParams { pub const METHOD: &'static str = "Storage.clearTrustTokens"; }

impl crate::CdpCommand for ClearTrustTokensParams {
    const METHOD: &'static str = "Storage.clearTrustTokens";
    type Response = ClearTrustTokensReturns;
}

/// Gets details for a named interest group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestGroupDetailsParams {

    pub ownerOrigin: String,

    pub name: String,
}

/// Gets details for a named interest group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestGroupDetailsReturns {
    /// This largely corresponds to:
    /// https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup
    /// but has absolute expirationTime instead of relative lifetimeMs and
    /// also adds joiningOrigin.

    pub details: serde_json::Map<String, JsonValue>,
}

impl GetInterestGroupDetailsParams { pub const METHOD: &'static str = "Storage.getInterestGroupDetails"; }

impl crate::CdpCommand for GetInterestGroupDetailsParams {
    const METHOD: &'static str = "Storage.getInterestGroupDetails";
    type Response = GetInterestGroupDetailsReturns;
}

/// Enables/Disables issuing of interestGroupAccessed events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupTrackingParams {

    pub enable: bool,
}

impl SetInterestGroupTrackingParams { pub const METHOD: &'static str = "Storage.setInterestGroupTracking"; }

impl crate::CdpCommand for SetInterestGroupTrackingParams {
    const METHOD: &'static str = "Storage.setInterestGroupTracking";
    type Response = crate::EmptyReturns;
}

/// Enables/Disables issuing of interestGroupAuctionEventOccurred and
/// interestGroupAuctionNetworkRequestCreated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupAuctionTrackingParams {

    pub enable: bool,
}

impl SetInterestGroupAuctionTrackingParams { pub const METHOD: &'static str = "Storage.setInterestGroupAuctionTracking"; }

impl crate::CdpCommand for SetInterestGroupAuctionTrackingParams {
    const METHOD: &'static str = "Storage.setInterestGroupAuctionTracking";
    type Response = crate::EmptyReturns;
}

/// Gets metadata for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageMetadataParams {

    pub ownerOrigin: String,
}

/// Gets metadata for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageMetadataReturns {

    pub metadata: SharedStorageMetadata,
}

impl GetSharedStorageMetadataParams { pub const METHOD: &'static str = "Storage.getSharedStorageMetadata"; }

impl crate::CdpCommand for GetSharedStorageMetadataParams {
    const METHOD: &'static str = "Storage.getSharedStorageMetadata";
    type Response = GetSharedStorageMetadataReturns;
}

/// Gets the entries in an given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageEntriesParams {

    pub ownerOrigin: String,
}

/// Gets the entries in an given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageEntriesReturns {

    pub entries: Vec<SharedStorageEntry>,
}

impl GetSharedStorageEntriesParams { pub const METHOD: &'static str = "Storage.getSharedStorageEntries"; }

impl crate::CdpCommand for GetSharedStorageEntriesParams {
    const METHOD: &'static str = "Storage.getSharedStorageEntries";
    type Response = GetSharedStorageEntriesReturns;
}

/// Sets entry with 'key' and 'value' for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageEntryParams {

    pub ownerOrigin: String,

    pub key: String,

    pub value: String,
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignoreIfPresent: Option<bool>,
}

impl SetSharedStorageEntryParams { pub const METHOD: &'static str = "Storage.setSharedStorageEntry"; }

impl crate::CdpCommand for SetSharedStorageEntryParams {
    const METHOD: &'static str = "Storage.setSharedStorageEntry";
    type Response = crate::EmptyReturns;
}

/// Deletes entry for 'key' (if it exists) for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSharedStorageEntryParams {

    pub ownerOrigin: String,

    pub key: String,
}

impl DeleteSharedStorageEntryParams { pub const METHOD: &'static str = "Storage.deleteSharedStorageEntry"; }

impl crate::CdpCommand for DeleteSharedStorageEntryParams {
    const METHOD: &'static str = "Storage.deleteSharedStorageEntry";
    type Response = crate::EmptyReturns;
}

/// Clears all entries for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearSharedStorageEntriesParams {

    pub ownerOrigin: String,
}

impl ClearSharedStorageEntriesParams { pub const METHOD: &'static str = "Storage.clearSharedStorageEntries"; }

impl crate::CdpCommand for ClearSharedStorageEntriesParams {
    const METHOD: &'static str = "Storage.clearSharedStorageEntries";
    type Response = crate::EmptyReturns;
}

/// Resets the budget for 'ownerOrigin' by clearing all budget withdrawals.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetSharedStorageBudgetParams {

    pub ownerOrigin: String,
}

impl ResetSharedStorageBudgetParams { pub const METHOD: &'static str = "Storage.resetSharedStorageBudget"; }

impl crate::CdpCommand for ResetSharedStorageBudgetParams {
    const METHOD: &'static str = "Storage.resetSharedStorageBudget";
    type Response = crate::EmptyReturns;
}

/// Enables/disables issuing of sharedStorageAccessed events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageTrackingParams {

    pub enable: bool,
}

impl SetSharedStorageTrackingParams { pub const METHOD: &'static str = "Storage.setSharedStorageTracking"; }

impl crate::CdpCommand for SetSharedStorageTrackingParams {
    const METHOD: &'static str = "Storage.setSharedStorageTracking";
    type Response = crate::EmptyReturns;
}

/// Set tracking for a storage key's buckets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageBucketTrackingParams {

    pub storageKey: String,

    pub enable: bool,
}

impl SetStorageBucketTrackingParams { pub const METHOD: &'static str = "Storage.setStorageBucketTracking"; }

impl crate::CdpCommand for SetStorageBucketTrackingParams {
    const METHOD: &'static str = "Storage.setStorageBucketTracking";
    type Response = crate::EmptyReturns;
}

/// Deletes the Storage Bucket with the given storage key and bucket name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStorageBucketParams {

    pub bucket: StorageBucket,
}

impl DeleteStorageBucketParams { pub const METHOD: &'static str = "Storage.deleteStorageBucket"; }

impl crate::CdpCommand for DeleteStorageBucketParams {
    const METHOD: &'static str = "Storage.deleteStorageBucket";
    type Response = crate::EmptyReturns;
}

/// Deletes state for sites identified as potential bounce trackers, immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunBounceTrackingMitigationsReturns {

    pub deletedSites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunBounceTrackingMitigationsParams {}

impl RunBounceTrackingMitigationsParams { pub const METHOD: &'static str = "Storage.runBounceTrackingMitigations"; }

impl crate::CdpCommand for RunBounceTrackingMitigationsParams {
    const METHOD: &'static str = "Storage.runBounceTrackingMitigations";
    type Response = RunBounceTrackingMitigationsReturns;
}

/// Returns the effective Related Website Sets in use by this profile for the browser
/// session. The effective Related Website Sets will not change during a browser session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelatedWebsiteSetsReturns {

    pub sets: Vec<RelatedWebsiteSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRelatedWebsiteSetsParams {}

impl GetRelatedWebsiteSetsParams { pub const METHOD: &'static str = "Storage.getRelatedWebsiteSets"; }

impl crate::CdpCommand for GetRelatedWebsiteSetsParams {
    const METHOD: &'static str = "Storage.getRelatedWebsiteSets";
    type Response = GetRelatedWebsiteSetsReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetProtectedAudienceKAnonymityParams {

    pub owner: String,

    pub name: String,

    pub hashes: Vec<String>,
}

impl SetProtectedAudienceKAnonymityParams { pub const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity"; }

impl crate::CdpCommand for SetProtectedAudienceKAnonymityParams {
    const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity";
    type Response = crate::EmptyReturns;
}
