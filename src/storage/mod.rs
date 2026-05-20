use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


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
    #[serde(rename = "interest_groups")]
    InterestGroups,
    #[serde(rename = "shared_storage")]
    SharedStorage,
    #[serde(rename = "storage_buckets")]
    StorageBuckets,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "other")]
    Other,
}

/// Usage for a storage type.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageForType {
    /// Name of storage type.
    storageType: StorageType,
    /// Storage usage (bytes).
    usage: f64,
}

impl UsageForType {
    pub fn builder() -> UsageForTypeBuilder { UsageForTypeBuilder::default() }
    pub fn storageType(&self) -> &StorageType { &self.storageType }
    pub fn usage(&self) -> f64 { self.usage }
}

#[derive(Default)]
pub struct UsageForTypeBuilder {
    storageType: Option<StorageType>,
    usage: Option<f64>,
}

impl UsageForTypeBuilder {
    /// Name of storage type.
    pub fn storageType(mut self, storageType: StorageType) -> Self { self.storageType = Some(storageType); self }
    /// Storage usage (bytes).
    pub fn usage(mut self, usage: f64) -> Self { self.usage = Some(usage); self }
    pub fn build(self) -> UsageForType {
        UsageForType {
            storageType: self.storageType.unwrap_or_default(),
            usage: self.usage.unwrap_or_default(),
        }
    }
}

/// Pair of issuer origin and number of available (signed, but not used) Trust
/// Tokens from that issuer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokens<'a> {
    issuerOrigin: Cow<'a, str>,
    count: f64,
}

impl<'a> TrustTokens<'a> {
    pub fn builder() -> TrustTokensBuilder<'a> { TrustTokensBuilder::default() }
    pub fn issuerOrigin(&self) -> &str { self.issuerOrigin.as_ref() }
    pub fn count(&self) -> f64 { self.count }
}

#[derive(Default)]
pub struct TrustTokensBuilder<'a> {
    issuerOrigin: Option<Cow<'a, str>>,
    count: Option<f64>,
}

impl<'a> TrustTokensBuilder<'a> {
    pub fn issuerOrigin(mut self, issuerOrigin: impl Into<Cow<'a, str>>) -> Self { self.issuerOrigin = Some(issuerOrigin.into()); self }
    pub fn count(mut self, count: f64) -> Self { self.count = Some(count); self }
    pub fn build(self) -> TrustTokens<'a> {
        TrustTokens {
            issuerOrigin: self.issuerOrigin.unwrap_or_default(),
            count: self.count.unwrap_or_default(),
        }
    }
}

/// Protected audience interest group auction identifier.

pub type InterestGroupAuctionId<'a> = Cow<'a, str>;

/// Enum of interest group access types.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAccessType {
    #[default]
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "leave")]
    Leave,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "additionalBid")]
    AdditionalBid,
    #[serde(rename = "additionalBidWin")]
    AdditionalBidWin,
    #[serde(rename = "topLevelBid")]
    TopLevelBid,
    #[serde(rename = "topLevelAdditionalBid")]
    TopLevelAdditionalBid,
    #[serde(rename = "clear")]
    Clear,
}

/// Enum of auction events.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAuctionEventType {
    #[default]
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "configResolved")]
    ConfigResolved,
}

/// Enum of network fetches auctions can do.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum InterestGroupAuctionFetchType {
    #[default]
    #[serde(rename = "bidderJs")]
    BidderJs,
    #[serde(rename = "bidderWasm")]
    BidderWasm,
    #[serde(rename = "sellerJs")]
    SellerJs,
    #[serde(rename = "bidderTrustedSignals")]
    BidderTrustedSignals,
    #[serde(rename = "sellerTrustedSignals")]
    SellerTrustedSignals,
}

/// Enum of shared storage access scopes.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedStorageAccessScope {
    #[default]
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "sharedStorageWorklet")]
    SharedStorageWorklet,
    #[serde(rename = "protectedAudienceWorklet")]
    ProtectedAudienceWorklet,
    #[serde(rename = "header")]
    Header,
}

/// Enum of shared storage access methods.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum SharedStorageAccessMethod {
    #[default]
    #[serde(rename = "addModule")]
    AddModule,
    #[serde(rename = "createWorklet")]
    CreateWorklet,
    #[serde(rename = "selectURL")]
    SelectURL,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "batchUpdate")]
    BatchUpdate,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "keys")]
    Keys,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "entries")]
    Entries,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "remainingBudget")]
    RemainingBudget,
}

/// Struct for a single key-value pair in an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageEntry<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SharedStorageEntry<'a> {
    pub fn builder() -> SharedStorageEntryBuilder<'a> { SharedStorageEntryBuilder::default() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}

#[derive(Default)]
pub struct SharedStorageEntryBuilder<'a> {
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
}

impl<'a> SharedStorageEntryBuilder<'a> {
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    pub fn build(self) -> SharedStorageEntry<'a> {
        SharedStorageEntry {
            key: self.key.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
        }
    }
}

/// Details for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageMetadata {
    /// Time when the origin's shared storage was last created.
    creationTime: crate::network::TimeSinceEpoch,
    /// Number of key-value pairs stored in origin's shared storage.
    length: u64,
    /// Current amount of bits of entropy remaining in the navigation budget.
    remainingBudget: f64,
    /// Total number of bytes stored as key-value pairs in origin's shared
    /// storage.
    bytesUsed: i64,
}

impl SharedStorageMetadata {
    pub fn builder() -> SharedStorageMetadataBuilder { SharedStorageMetadataBuilder::default() }
    pub fn creationTime(&self) -> &crate::network::TimeSinceEpoch { &self.creationTime }
    pub fn length(&self) -> u64 { self.length }
    pub fn remainingBudget(&self) -> f64 { self.remainingBudget }
    pub fn bytesUsed(&self) -> i64 { self.bytesUsed }
}

#[derive(Default)]
pub struct SharedStorageMetadataBuilder {
    creationTime: Option<crate::network::TimeSinceEpoch>,
    length: Option<u64>,
    remainingBudget: Option<f64>,
    bytesUsed: Option<i64>,
}

impl SharedStorageMetadataBuilder {
    /// Time when the origin's shared storage was last created.
    pub fn creationTime(mut self, creationTime: crate::network::TimeSinceEpoch) -> Self { self.creationTime = Some(creationTime); self }
    /// Number of key-value pairs stored in origin's shared storage.
    pub fn length(mut self, length: u64) -> Self { self.length = Some(length); self }
    /// Current amount of bits of entropy remaining in the navigation budget.
    pub fn remainingBudget(mut self, remainingBudget: f64) -> Self { self.remainingBudget = Some(remainingBudget); self }
    /// Total number of bytes stored as key-value pairs in origin's shared
    /// storage.
    pub fn bytesUsed(mut self, bytesUsed: i64) -> Self { self.bytesUsed = Some(bytesUsed); self }
    pub fn build(self) -> SharedStorageMetadata {
        SharedStorageMetadata {
            creationTime: self.creationTime.unwrap_or_default(),
            length: self.length.unwrap_or_default(),
            remainingBudget: self.remainingBudget.unwrap_or_default(),
            bytesUsed: self.bytesUsed.unwrap_or_default(),
        }
    }
}

/// Represents a dictionary object passed in as privateAggregationConfig to
/// run or selectURL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStoragePrivateAggregationConfig<'a> {
    /// The chosen aggregation service deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregationCoordinatorOrigin: Option<Cow<'a, str>>,
    /// The context ID provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    contextId: Option<Cow<'a, str>>,
    /// Configures the maximum size allowed for filtering IDs.
    filteringIdMaxBytes: u64,
    /// The limit on the number of contributions in the final report.
    #[serde(skip_serializing_if = "Option::is_none")]
    maxContributions: Option<i64>,
}

impl<'a> SharedStoragePrivateAggregationConfig<'a> {
    pub fn builder() -> SharedStoragePrivateAggregationConfigBuilder<'a> { SharedStoragePrivateAggregationConfigBuilder::default() }
    pub fn aggregationCoordinatorOrigin(&self) -> Option<&str> { self.aggregationCoordinatorOrigin.as_deref() }
    pub fn contextId(&self) -> Option<&str> { self.contextId.as_deref() }
    pub fn filteringIdMaxBytes(&self) -> u64 { self.filteringIdMaxBytes }
    pub fn maxContributions(&self) -> Option<i64> { self.maxContributions }
}

#[derive(Default)]
pub struct SharedStoragePrivateAggregationConfigBuilder<'a> {
    aggregationCoordinatorOrigin: Option<Cow<'a, str>>,
    contextId: Option<Cow<'a, str>>,
    filteringIdMaxBytes: Option<u64>,
    maxContributions: Option<i64>,
}

impl<'a> SharedStoragePrivateAggregationConfigBuilder<'a> {
    /// The chosen aggregation service deployment.
    pub fn aggregationCoordinatorOrigin(mut self, aggregationCoordinatorOrigin: impl Into<Cow<'a, str>>) -> Self { self.aggregationCoordinatorOrigin = Some(aggregationCoordinatorOrigin.into()); self }
    /// The context ID provided.
    pub fn contextId(mut self, contextId: impl Into<Cow<'a, str>>) -> Self { self.contextId = Some(contextId.into()); self }
    /// Configures the maximum size allowed for filtering IDs.
    pub fn filteringIdMaxBytes(mut self, filteringIdMaxBytes: u64) -> Self { self.filteringIdMaxBytes = Some(filteringIdMaxBytes); self }
    /// The limit on the number of contributions in the final report.
    pub fn maxContributions(mut self, maxContributions: i64) -> Self { self.maxContributions = Some(maxContributions); self }
    pub fn build(self) -> SharedStoragePrivateAggregationConfig<'a> {
        SharedStoragePrivateAggregationConfig {
            aggregationCoordinatorOrigin: self.aggregationCoordinatorOrigin,
            contextId: self.contextId,
            filteringIdMaxBytes: self.filteringIdMaxBytes.unwrap_or_default(),
            maxContributions: self.maxContributions,
        }
    }
}

/// Pair of reporting metadata details for a candidate URL for 'selectURL()'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageReportingMetadata<'a> {
    eventType: Cow<'a, str>,
    reportingUrl: Cow<'a, str>,
}

impl<'a> SharedStorageReportingMetadata<'a> {
    pub fn builder() -> SharedStorageReportingMetadataBuilder<'a> { SharedStorageReportingMetadataBuilder::default() }
    pub fn eventType(&self) -> &str { self.eventType.as_ref() }
    pub fn reportingUrl(&self) -> &str { self.reportingUrl.as_ref() }
}

#[derive(Default)]
pub struct SharedStorageReportingMetadataBuilder<'a> {
    eventType: Option<Cow<'a, str>>,
    reportingUrl: Option<Cow<'a, str>>,
}

impl<'a> SharedStorageReportingMetadataBuilder<'a> {
    pub fn eventType(mut self, eventType: impl Into<Cow<'a, str>>) -> Self { self.eventType = Some(eventType.into()); self }
    pub fn reportingUrl(mut self, reportingUrl: impl Into<Cow<'a, str>>) -> Self { self.reportingUrl = Some(reportingUrl.into()); self }
    pub fn build(self) -> SharedStorageReportingMetadata<'a> {
        SharedStorageReportingMetadata {
            eventType: self.eventType.unwrap_or_default(),
            reportingUrl: self.reportingUrl.unwrap_or_default(),
        }
    }
}

/// Bundles a candidate URL with its reporting metadata.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageUrlWithMetadata<'a> {
    /// Spec of candidate URL.
    url: Cow<'a, str>,
    /// Any associated reporting metadata.
    reportingMetadata: Vec<SharedStorageReportingMetadata<'a>>,
}

impl<'a> SharedStorageUrlWithMetadata<'a> {
    pub fn builder() -> SharedStorageUrlWithMetadataBuilder<'a> { SharedStorageUrlWithMetadataBuilder::default() }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn reportingMetadata(&self) -> &[SharedStorageReportingMetadata<'a>] { &self.reportingMetadata }
}

#[derive(Default)]
pub struct SharedStorageUrlWithMetadataBuilder<'a> {
    url: Option<Cow<'a, str>>,
    reportingMetadata: Option<Vec<SharedStorageReportingMetadata<'a>>>,
}

impl<'a> SharedStorageUrlWithMetadataBuilder<'a> {
    /// Spec of candidate URL.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Any associated reporting metadata.
    pub fn reportingMetadata(mut self, reportingMetadata: Vec<SharedStorageReportingMetadata<'a>>) -> Self { self.reportingMetadata = Some(reportingMetadata); self }
    pub fn build(self) -> SharedStorageUrlWithMetadata<'a> {
        SharedStorageUrlWithMetadata {
            url: self.url.unwrap_or_default(),
            reportingMetadata: self.reportingMetadata.unwrap_or_default(),
        }
    }
}

/// Bundles the parameters for shared storage access events whose
/// presence/absence can vary according to SharedStorageAccessType.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageAccessParams<'a> {
    /// Spec of the module script URL.
    /// Present only for SharedStorageAccessMethods: addModule and
    /// createWorklet.
    #[serde(skip_serializing_if = "Option::is_none")]
    scriptSourceUrl: Option<Cow<'a, str>>,
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.
    #[serde(skip_serializing_if = "Option::is_none")]
    dataOrigin: Option<Cow<'a, str>>,
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    operationName: Option<Cow<'a, str>>,
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    operationId: Option<Cow<'a, str>>,
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    keepAlive: Option<bool>,
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    privateAggregationConfig: Option<SharedStoragePrivateAggregationConfig<'a>>,
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.
    #[serde(skip_serializing_if = "Option::is_none")]
    serializedData: Option<Cow<'a, str>>,
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    urlsWithMetadata: Option<Vec<SharedStorageUrlWithMetadata<'a>>>,
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.
    #[serde(skip_serializing_if = "Option::is_none")]
    urnUuid: Option<Cow<'a, str>>,
    /// Key for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set, append, delete, and
    /// get.
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Cow<'a, str>>,
    /// Value for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set and append.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Cow<'a, str>>,
    /// Whether or not to set an entry for a key if that key is already present.
    /// Present only for SharedStorageAccessMethod: set.
    #[serde(skip_serializing_if = "Option::is_none")]
    ignoreIfPresent: Option<bool>,
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.
    #[serde(skip_serializing_if = "Option::is_none")]
    workletOrdinal: Option<i64>,
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.
    #[serde(skip_serializing_if = "Option::is_none")]
    workletTargetId: Option<crate::target::TargetID<'a>>,
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    withLock: Option<Cow<'a, str>>,
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    batchUpdateId: Option<Cow<'a, str>>,
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.
    #[serde(skip_serializing_if = "Option::is_none")]
    batchSize: Option<u64>,
}

impl<'a> SharedStorageAccessParams<'a> {
    pub fn builder() -> SharedStorageAccessParamsBuilder<'a> { SharedStorageAccessParamsBuilder::default() }
    pub fn scriptSourceUrl(&self) -> Option<&str> { self.scriptSourceUrl.as_deref() }
    pub fn dataOrigin(&self) -> Option<&str> { self.dataOrigin.as_deref() }
    pub fn operationName(&self) -> Option<&str> { self.operationName.as_deref() }
    pub fn operationId(&self) -> Option<&str> { self.operationId.as_deref() }
    pub fn keepAlive(&self) -> Option<bool> { self.keepAlive }
    pub fn privateAggregationConfig(&self) -> Option<&SharedStoragePrivateAggregationConfig<'a>> { self.privateAggregationConfig.as_ref() }
    pub fn serializedData(&self) -> Option<&str> { self.serializedData.as_deref() }
    pub fn urlsWithMetadata(&self) -> Option<&[SharedStorageUrlWithMetadata<'a>]> { self.urlsWithMetadata.as_deref() }
    pub fn urnUuid(&self) -> Option<&str> { self.urnUuid.as_deref() }
    pub fn key(&self) -> Option<&str> { self.key.as_deref() }
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }
    pub fn ignoreIfPresent(&self) -> Option<bool> { self.ignoreIfPresent }
    pub fn workletOrdinal(&self) -> Option<i64> { self.workletOrdinal }
    pub fn workletTargetId(&self) -> Option<&crate::target::TargetID<'a>> { self.workletTargetId.as_ref() }
    pub fn withLock(&self) -> Option<&str> { self.withLock.as_deref() }
    pub fn batchUpdateId(&self) -> Option<&str> { self.batchUpdateId.as_deref() }
    pub fn batchSize(&self) -> Option<u64> { self.batchSize }
}

#[derive(Default)]
pub struct SharedStorageAccessParamsBuilder<'a> {
    scriptSourceUrl: Option<Cow<'a, str>>,
    dataOrigin: Option<Cow<'a, str>>,
    operationName: Option<Cow<'a, str>>,
    operationId: Option<Cow<'a, str>>,
    keepAlive: Option<bool>,
    privateAggregationConfig: Option<SharedStoragePrivateAggregationConfig<'a>>,
    serializedData: Option<Cow<'a, str>>,
    urlsWithMetadata: Option<Vec<SharedStorageUrlWithMetadata<'a>>>,
    urnUuid: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    ignoreIfPresent: Option<bool>,
    workletOrdinal: Option<i64>,
    workletTargetId: Option<crate::target::TargetID<'a>>,
    withLock: Option<Cow<'a, str>>,
    batchUpdateId: Option<Cow<'a, str>>,
    batchSize: Option<u64>,
}

impl<'a> SharedStorageAccessParamsBuilder<'a> {
    /// Spec of the module script URL.
    /// Present only for SharedStorageAccessMethods: addModule and
    /// createWorklet.
    pub fn scriptSourceUrl(mut self, scriptSourceUrl: impl Into<Cow<'a, str>>) -> Self { self.scriptSourceUrl = Some(scriptSourceUrl.into()); self }
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.
    pub fn dataOrigin(mut self, dataOrigin: impl Into<Cow<'a, str>>) -> Self { self.dataOrigin = Some(dataOrigin.into()); self }
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operationName(mut self, operationName: impl Into<Cow<'a, str>>) -> Self { self.operationName = Some(operationName.into()); self }
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operationId(mut self, operationId: impl Into<Cow<'a, str>>) -> Self { self.operationId = Some(operationId.into()); self }
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn keepAlive(mut self, keepAlive: bool) -> Self { self.keepAlive = Some(keepAlive); self }
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn privateAggregationConfig(mut self, privateAggregationConfig: SharedStoragePrivateAggregationConfig<'a>) -> Self { self.privateAggregationConfig = Some(privateAggregationConfig); self }
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.
    pub fn serializedData(mut self, serializedData: impl Into<Cow<'a, str>>) -> Self { self.serializedData = Some(serializedData.into()); self }
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urlsWithMetadata(mut self, urlsWithMetadata: Vec<SharedStorageUrlWithMetadata<'a>>) -> Self { self.urlsWithMetadata = Some(urlsWithMetadata); self }
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urnUuid(mut self, urnUuid: impl Into<Cow<'a, str>>) -> Self { self.urnUuid = Some(urnUuid.into()); self }
    /// Key for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set, append, delete, and
    /// get.
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    /// Value for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set and append.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Whether or not to set an entry for a key if that key is already present.
    /// Present only for SharedStorageAccessMethod: set.
    pub fn ignoreIfPresent(mut self, ignoreIfPresent: bool) -> Self { self.ignoreIfPresent = Some(ignoreIfPresent); self }
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.
    pub fn workletOrdinal(mut self, workletOrdinal: i64) -> Self { self.workletOrdinal = Some(workletOrdinal); self }
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.
    pub fn workletTargetId(mut self, workletTargetId: crate::target::TargetID<'a>) -> Self { self.workletTargetId = Some(workletTargetId); self }
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.
    pub fn withLock(mut self, withLock: impl Into<Cow<'a, str>>) -> Self { self.withLock = Some(withLock.into()); self }
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.
    pub fn batchUpdateId(mut self, batchUpdateId: impl Into<Cow<'a, str>>) -> Self { self.batchUpdateId = Some(batchUpdateId.into()); self }
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.
    pub fn batchSize(mut self, batchSize: u64) -> Self { self.batchSize = Some(batchSize); self }
    pub fn build(self) -> SharedStorageAccessParams<'a> {
        SharedStorageAccessParams {
            scriptSourceUrl: self.scriptSourceUrl,
            dataOrigin: self.dataOrigin,
            operationName: self.operationName,
            operationId: self.operationId,
            keepAlive: self.keepAlive,
            privateAggregationConfig: self.privateAggregationConfig,
            serializedData: self.serializedData,
            urlsWithMetadata: self.urlsWithMetadata,
            urnUuid: self.urnUuid,
            key: self.key,
            value: self.value,
            ignoreIfPresent: self.ignoreIfPresent,
            workletOrdinal: self.workletOrdinal,
            workletTargetId: self.workletTargetId,
            withLock: self.withLock,
            batchUpdateId: self.batchUpdateId,
            batchSize: self.batchSize,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum StorageBucketsDurability {
    #[default]
    #[serde(rename = "relaxed")]
    Relaxed,
    #[serde(rename = "strict")]
    Strict,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucket<'a> {
    storageKey: SerializedStorageKey<'a>,
    /// If not specified, it is the default bucket of the storageKey.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
}

impl<'a> StorageBucket<'a> {
    pub fn builder() -> StorageBucketBuilder<'a> { StorageBucketBuilder::default() }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}

#[derive(Default)]
pub struct StorageBucketBuilder<'a> {
    storageKey: Option<SerializedStorageKey<'a>>,
    name: Option<Cow<'a, str>>,
}

impl<'a> StorageBucketBuilder<'a> {
    pub fn storageKey(mut self, storageKey: SerializedStorageKey<'a>) -> Self { self.storageKey = Some(storageKey); self }
    /// If not specified, it is the default bucket of the storageKey.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> StorageBucket<'a> {
        StorageBucket {
            storageKey: self.storageKey.unwrap_or_default(),
            name: self.name,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageBucketInfo<'a> {
    bucket: StorageBucket<'a>,
    id: Cow<'a, str>,
    expiration: crate::network::TimeSinceEpoch,
    /// Storage quota (bytes).
    quota: f64,
    persistent: bool,
    durability: StorageBucketsDurability,
}

impl<'a> StorageBucketInfo<'a> {
    pub fn builder() -> StorageBucketInfoBuilder<'a> { StorageBucketInfoBuilder::default() }
    pub fn bucket(&self) -> &StorageBucket<'a> { &self.bucket }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn expiration(&self) -> &crate::network::TimeSinceEpoch { &self.expiration }
    pub fn quota(&self) -> f64 { self.quota }
    pub fn persistent(&self) -> bool { self.persistent }
    pub fn durability(&self) -> &StorageBucketsDurability { &self.durability }
}

#[derive(Default)]
pub struct StorageBucketInfoBuilder<'a> {
    bucket: Option<StorageBucket<'a>>,
    id: Option<Cow<'a, str>>,
    expiration: Option<crate::network::TimeSinceEpoch>,
    quota: Option<f64>,
    persistent: Option<bool>,
    durability: Option<StorageBucketsDurability>,
}

impl<'a> StorageBucketInfoBuilder<'a> {
    pub fn bucket(mut self, bucket: StorageBucket<'a>) -> Self { self.bucket = Some(bucket); self }
    pub fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self { self.id = Some(id.into()); self }
    pub fn expiration(mut self, expiration: crate::network::TimeSinceEpoch) -> Self { self.expiration = Some(expiration); self }
    /// Storage quota (bytes).
    pub fn quota(mut self, quota: f64) -> Self { self.quota = Some(quota); self }
    pub fn persistent(mut self, persistent: bool) -> Self { self.persistent = Some(persistent); self }
    pub fn durability(mut self, durability: StorageBucketsDurability) -> Self { self.durability = Some(durability); self }
    pub fn build(self) -> StorageBucketInfo<'a> {
        StorageBucketInfo {
            bucket: self.bucket.unwrap_or_default(),
            id: self.id.unwrap_or_default(),
            expiration: self.expiration.unwrap_or_default(),
            quota: self.quota.unwrap_or_default(),
            persistent: self.persistent.unwrap_or_default(),
            durability: self.durability.unwrap_or_default(),
        }
    }
}

/// A single Related Website Set object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelatedWebsiteSet<'a> {
    /// The primary site of this set, along with the ccTLDs if there is any.
    primarySites: Vec<Cow<'a, str>>,
    /// The associated sites of this set, along with the ccTLDs if there is any.
    associatedSites: Vec<Cow<'a, str>>,
    /// The service sites of this set, along with the ccTLDs if there is any.
    serviceSites: Vec<Cow<'a, str>>,
}

impl<'a> RelatedWebsiteSet<'a> {
    pub fn builder() -> RelatedWebsiteSetBuilder<'a> { RelatedWebsiteSetBuilder::default() }
    pub fn primarySites(&self) -> &[Cow<'a, str>] { &self.primarySites }
    pub fn associatedSites(&self) -> &[Cow<'a, str>] { &self.associatedSites }
    pub fn serviceSites(&self) -> &[Cow<'a, str>] { &self.serviceSites }
}

#[derive(Default)]
pub struct RelatedWebsiteSetBuilder<'a> {
    primarySites: Option<Vec<Cow<'a, str>>>,
    associatedSites: Option<Vec<Cow<'a, str>>>,
    serviceSites: Option<Vec<Cow<'a, str>>>,
}

impl<'a> RelatedWebsiteSetBuilder<'a> {
    /// The primary site of this set, along with the ccTLDs if there is any.
    pub fn primarySites(mut self, primarySites: Vec<Cow<'a, str>>) -> Self { self.primarySites = Some(primarySites); self }
    /// The associated sites of this set, along with the ccTLDs if there is any.
    pub fn associatedSites(mut self, associatedSites: Vec<Cow<'a, str>>) -> Self { self.associatedSites = Some(associatedSites); self }
    /// The service sites of this set, along with the ccTLDs if there is any.
    pub fn serviceSites(mut self, serviceSites: Vec<Cow<'a, str>>) -> Self { self.serviceSites = Some(serviceSites); self }
    pub fn build(self) -> RelatedWebsiteSet<'a> {
        RelatedWebsiteSet {
            primarySites: self.primarySites.unwrap_or_default(),
            associatedSites: self.associatedSites.unwrap_or_default(),
            serviceSites: self.serviceSites.unwrap_or_default(),
        }
    }
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameParams<'a> {
    frameId: crate::page::FrameId<'a>,
}

impl<'a> GetStorageKeyForFrameParams<'a> {
    pub fn builder() -> GetStorageKeyForFrameParamsBuilder<'a> { GetStorageKeyForFrameParamsBuilder::default() }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}

#[derive(Default)]
pub struct GetStorageKeyForFrameParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetStorageKeyForFrameParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetStorageKeyForFrameParams<'a> {
        GetStorageKeyForFrameParams {
            frameId: self.frameId.unwrap_or_default(),
        }
    }
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameReturns<'a> {
    storageKey: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyForFrameReturns<'a> {
    pub fn builder() -> GetStorageKeyForFrameReturnsBuilder<'a> { GetStorageKeyForFrameReturnsBuilder::default() }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
}

#[derive(Default)]
pub struct GetStorageKeyForFrameReturnsBuilder<'a> {
    storageKey: Option<SerializedStorageKey<'a>>,
}

impl<'a> GetStorageKeyForFrameReturnsBuilder<'a> {
    pub fn storageKey(mut self, storageKey: SerializedStorageKey<'a>) -> Self { self.storageKey = Some(storageKey); self }
    pub fn build(self) -> GetStorageKeyForFrameReturns<'a> {
        GetStorageKeyForFrameReturns {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> GetStorageKeyForFrameParams<'a> { pub const METHOD: &'static str = "Storage.getStorageKeyForFrame"; }

impl<'a> crate::CdpCommand<'a> for GetStorageKeyForFrameParams<'a> {
    const METHOD: &'static str = "Storage.getStorageKeyForFrame";
    type Response = GetStorageKeyForFrameReturns<'a>;
}

/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetStorageKeyParams<'a> {
    pub fn builder() -> GetStorageKeyParamsBuilder<'a> { GetStorageKeyParamsBuilder::default() }
    pub fn frameId(&self) -> Option<&crate::page::FrameId<'a>> { self.frameId.as_ref() }
}

#[derive(Default)]
pub struct GetStorageKeyParamsBuilder<'a> {
    frameId: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetStorageKeyParamsBuilder<'a> {
    pub fn frameId(mut self, frameId: crate::page::FrameId<'a>) -> Self { self.frameId = Some(frameId); self }
    pub fn build(self) -> GetStorageKeyParams<'a> {
        GetStorageKeyParams {
            frameId: self.frameId,
        }
    }
}

/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyReturns<'a> {
    storageKey: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyReturns<'a> {
    pub fn builder() -> GetStorageKeyReturnsBuilder<'a> { GetStorageKeyReturnsBuilder::default() }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
}

#[derive(Default)]
pub struct GetStorageKeyReturnsBuilder<'a> {
    storageKey: Option<SerializedStorageKey<'a>>,
}

impl<'a> GetStorageKeyReturnsBuilder<'a> {
    pub fn storageKey(mut self, storageKey: SerializedStorageKey<'a>) -> Self { self.storageKey = Some(storageKey); self }
    pub fn build(self) -> GetStorageKeyReturns<'a> {
        GetStorageKeyReturns {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> GetStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.getStorageKey"; }

impl<'a> crate::CdpCommand<'a> for GetStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.getStorageKey";
    type Response = GetStorageKeyReturns<'a>;
}

/// Clears storage for origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
    /// Comma separated list of StorageType to clear.
    storageTypes: Cow<'a, str>,
}

impl<'a> ClearDataForOriginParams<'a> {
    pub fn builder() -> ClearDataForOriginParamsBuilder<'a> { ClearDataForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn storageTypes(&self) -> &str { self.storageTypes.as_ref() }
}

#[derive(Default)]
pub struct ClearDataForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    storageTypes: Option<Cow<'a, str>>,
}

impl<'a> ClearDataForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// Comma separated list of StorageType to clear.
    pub fn storageTypes(mut self, storageTypes: impl Into<Cow<'a, str>>) -> Self { self.storageTypes = Some(storageTypes.into()); self }
    pub fn build(self) -> ClearDataForOriginParams<'a> {
        ClearDataForOriginParams {
            origin: self.origin.unwrap_or_default(),
            storageTypes: self.storageTypes.unwrap_or_default(),
        }
    }
}

impl<'a> ClearDataForOriginParams<'a> { pub const METHOD: &'static str = "Storage.clearDataForOrigin"; }

impl<'a> crate::CdpCommand<'a> for ClearDataForOriginParams<'a> {
    const METHOD: &'static str = "Storage.clearDataForOrigin";
    type Response = crate::EmptyReturns;
}

/// Clears storage for storage key.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForStorageKeyParams<'a> {
    /// Storage key.
    storageKey: Cow<'a, str>,
    /// Comma separated list of StorageType to clear.
    storageTypes: Cow<'a, str>,
}

impl<'a> ClearDataForStorageKeyParams<'a> {
    pub fn builder() -> ClearDataForStorageKeyParamsBuilder<'a> { ClearDataForStorageKeyParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn storageTypes(&self) -> &str { self.storageTypes.as_ref() }
}

#[derive(Default)]
pub struct ClearDataForStorageKeyParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
    storageTypes: Option<Cow<'a, str>>,
}

impl<'a> ClearDataForStorageKeyParamsBuilder<'a> {
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Comma separated list of StorageType to clear.
    pub fn storageTypes(mut self, storageTypes: impl Into<Cow<'a, str>>) -> Self { self.storageTypes = Some(storageTypes.into()); self }
    pub fn build(self) -> ClearDataForStorageKeyParams<'a> {
        ClearDataForStorageKeyParams {
            storageKey: self.storageKey.unwrap_or_default(),
            storageTypes: self.storageTypes.unwrap_or_default(),
        }
    }
}

impl<'a> ClearDataForStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.clearDataForStorageKey"; }

impl<'a> crate::CdpCommand<'a> for ClearDataForStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.clearDataForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesParams<'a> {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetCookiesParams<'a> {
    pub fn builder() -> GetCookiesParamsBuilder<'a> { GetCookiesParamsBuilder::default() }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
}

#[derive(Default)]
pub struct GetCookiesParamsBuilder<'a> {
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> GetCookiesParams<'a> {
        GetCookiesParams {
            browserContextId: self.browserContextId,
        }
    }
}

/// Returns all browser cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetCookiesReturns<'a> {
    /// Array of cookie objects.
    cookies: Vec<crate::network::Cookie<'a>>,
}

impl<'a> GetCookiesReturns<'a> {
    pub fn builder() -> GetCookiesReturnsBuilder<'a> { GetCookiesReturnsBuilder::default() }
    pub fn cookies(&self) -> &[crate::network::Cookie<'a>] { &self.cookies }
}

#[derive(Default)]
pub struct GetCookiesReturnsBuilder<'a> {
    cookies: Option<Vec<crate::network::Cookie<'a>>>,
}

impl<'a> GetCookiesReturnsBuilder<'a> {
    /// Array of cookie objects.
    pub fn cookies(mut self, cookies: Vec<crate::network::Cookie<'a>>) -> Self { self.cookies = Some(cookies); self }
    pub fn build(self) -> GetCookiesReturns<'a> {
        GetCookiesReturns {
            cookies: self.cookies.unwrap_or_default(),
        }
    }
}

impl<'a> GetCookiesParams<'a> { pub const METHOD: &'static str = "Storage.getCookies"; }

impl<'a> crate::CdpCommand<'a> for GetCookiesParams<'a> {
    const METHOD: &'static str = "Storage.getCookies";
    type Response = GetCookiesReturns<'a>;
}

/// Sets given cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesParams<'a> {
    /// Cookies to be set.
    cookies: Vec<crate::network::CookieParam<'a>>,
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> SetCookiesParams<'a> {
    pub fn builder() -> SetCookiesParamsBuilder<'a> { SetCookiesParamsBuilder::default() }
    pub fn cookies(&self) -> &[crate::network::CookieParam<'a>] { &self.cookies }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
}

#[derive(Default)]
pub struct SetCookiesParamsBuilder<'a> {
    cookies: Option<Vec<crate::network::CookieParam<'a>>>,
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> SetCookiesParamsBuilder<'a> {
    /// Cookies to be set.
    pub fn cookies(mut self, cookies: Vec<crate::network::CookieParam<'a>>) -> Self { self.cookies = Some(cookies); self }
    /// Browser context to use when called on the browser endpoint.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> SetCookiesParams<'a> {
        SetCookiesParams {
            cookies: self.cookies.unwrap_or_default(),
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> SetCookiesParams<'a> { pub const METHOD: &'static str = "Storage.setCookies"; }

impl<'a> crate::CdpCommand<'a> for SetCookiesParams<'a> {
    const METHOD: &'static str = "Storage.setCookies";
    type Response = crate::EmptyReturns;
}

/// Clears cookies.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearCookiesParams<'a> {
    /// Browser context to use when called on the browser endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> ClearCookiesParams<'a> {
    pub fn builder() -> ClearCookiesParamsBuilder<'a> { ClearCookiesParamsBuilder::default() }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
}

#[derive(Default)]
pub struct ClearCookiesParamsBuilder<'a> {
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> ClearCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> ClearCookiesParams<'a> {
        ClearCookiesParams {
            browserContextId: self.browserContextId,
        }
    }
}

impl<'a> ClearCookiesParams<'a> { pub const METHOD: &'static str = "Storage.clearCookies"; }

impl<'a> crate::CdpCommand<'a> for ClearCookiesParams<'a> {
    const METHOD: &'static str = "Storage.clearCookies";
    type Response = crate::EmptyReturns;
}

/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageAndQuotaParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
}

impl<'a> GetUsageAndQuotaParams<'a> {
    pub fn builder() -> GetUsageAndQuotaParamsBuilder<'a> { GetUsageAndQuotaParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct GetUsageAndQuotaParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> GetUsageAndQuotaParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> GetUsageAndQuotaParams<'a> {
        GetUsageAndQuotaParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

/// Returns usage and quota in bytes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUsageAndQuotaReturns {
    /// Storage usage (bytes).
    usage: f64,
    /// Storage quota (bytes).
    quota: f64,
    /// Whether or not the origin has an active storage quota override
    overrideActive: bool,
    /// Storage usage per type (bytes).
    usageBreakdown: Vec<UsageForType>,
}

impl GetUsageAndQuotaReturns {
    pub fn builder() -> GetUsageAndQuotaReturnsBuilder { GetUsageAndQuotaReturnsBuilder::default() }
    pub fn usage(&self) -> f64 { self.usage }
    pub fn quota(&self) -> f64 { self.quota }
    pub fn overrideActive(&self) -> bool { self.overrideActive }
    pub fn usageBreakdown(&self) -> &[UsageForType] { &self.usageBreakdown }
}

#[derive(Default)]
pub struct GetUsageAndQuotaReturnsBuilder {
    usage: Option<f64>,
    quota: Option<f64>,
    overrideActive: Option<bool>,
    usageBreakdown: Option<Vec<UsageForType>>,
}

impl GetUsageAndQuotaReturnsBuilder {
    /// Storage usage (bytes).
    pub fn usage(mut self, usage: f64) -> Self { self.usage = Some(usage); self }
    /// Storage quota (bytes).
    pub fn quota(mut self, quota: f64) -> Self { self.quota = Some(quota); self }
    /// Whether or not the origin has an active storage quota override
    pub fn overrideActive(mut self, overrideActive: bool) -> Self { self.overrideActive = Some(overrideActive); self }
    /// Storage usage per type (bytes).
    pub fn usageBreakdown(mut self, usageBreakdown: Vec<UsageForType>) -> Self { self.usageBreakdown = Some(usageBreakdown); self }
    pub fn build(self) -> GetUsageAndQuotaReturns {
        GetUsageAndQuotaReturns {
            usage: self.usage.unwrap_or_default(),
            quota: self.quota.unwrap_or_default(),
            overrideActive: self.overrideActive.unwrap_or_default(),
            usageBreakdown: self.usageBreakdown.unwrap_or_default(),
        }
    }
}

impl<'a> GetUsageAndQuotaParams<'a> { pub const METHOD: &'static str = "Storage.getUsageAndQuota"; }

impl<'a> crate::CdpCommand<'a> for GetUsageAndQuotaParams<'a> {
    const METHOD: &'static str = "Storage.getUsageAndQuota";
    type Response = GetUsageAndQuotaReturns;
}

/// Override quota for the specified origin

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OverrideQuotaForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).
    #[serde(skip_serializing_if = "Option::is_none")]
    quotaSize: Option<f64>,
}

impl<'a> OverrideQuotaForOriginParams<'a> {
    pub fn builder() -> OverrideQuotaForOriginParamsBuilder<'a> { OverrideQuotaForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn quotaSize(&self) -> Option<f64> { self.quotaSize }
}

#[derive(Default)]
pub struct OverrideQuotaForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
    quotaSize: Option<f64>,
}

impl<'a> OverrideQuotaForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).
    pub fn quotaSize(mut self, quotaSize: f64) -> Self { self.quotaSize = Some(quotaSize); self }
    pub fn build(self) -> OverrideQuotaForOriginParams<'a> {
        OverrideQuotaForOriginParams {
            origin: self.origin.unwrap_or_default(),
            quotaSize: self.quotaSize,
        }
    }
}

impl<'a> OverrideQuotaForOriginParams<'a> { pub const METHOD: &'static str = "Storage.overrideQuotaForOrigin"; }

impl<'a> crate::CdpCommand<'a> for OverrideQuotaForOriginParams<'a> {
    const METHOD: &'static str = "Storage.overrideQuotaForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers origin to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForOriginParams<'a> {
    pub fn builder() -> TrackCacheStorageForOriginParamsBuilder<'a> { TrackCacheStorageForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct TrackCacheStorageForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> TrackCacheStorageForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> TrackCacheStorageForOriginParams<'a> {
        TrackCacheStorageForOriginParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

impl<'a> TrackCacheStorageForOriginParams<'a> { pub const METHOD: &'static str = "Storage.trackCacheStorageForOrigin"; }

impl<'a> crate::CdpCommand<'a> for TrackCacheStorageForOriginParams<'a> {
    const METHOD: &'static str = "Storage.trackCacheStorageForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers storage key to be notified when an update occurs to its cache storage list.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForStorageKeyParams<'a> {
    /// Storage key.
    storageKey: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForStorageKeyParams<'a> {
    pub fn builder() -> TrackCacheStorageForStorageKeyParamsBuilder<'a> { TrackCacheStorageForStorageKeyParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}

#[derive(Default)]
pub struct TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
}

impl<'a> TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn build(self) -> TrackCacheStorageForStorageKeyParams<'a> {
        TrackCacheStorageForStorageKeyParams {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> TrackCacheStorageForStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.trackCacheStorageForStorageKey"; }

impl<'a> crate::CdpCommand<'a> for TrackCacheStorageForStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.trackCacheStorageForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Registers origin to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForOriginParams<'a> {
    pub fn builder() -> TrackIndexedDBForOriginParamsBuilder<'a> { TrackIndexedDBForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct TrackIndexedDBForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> TrackIndexedDBForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> TrackIndexedDBForOriginParams<'a> {
        TrackIndexedDBForOriginParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

impl<'a> TrackIndexedDBForOriginParams<'a> { pub const METHOD: &'static str = "Storage.trackIndexedDBForOrigin"; }

impl<'a> crate::CdpCommand<'a> for TrackIndexedDBForOriginParams<'a> {
    const METHOD: &'static str = "Storage.trackIndexedDBForOrigin";
    type Response = crate::EmptyReturns;
}

/// Registers storage key to be notified when an update occurs to its IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForStorageKeyParams<'a> {
    /// Storage key.
    storageKey: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForStorageKeyParams<'a> {
    pub fn builder() -> TrackIndexedDBForStorageKeyParamsBuilder<'a> { TrackIndexedDBForStorageKeyParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}

#[derive(Default)]
pub struct TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
}

impl<'a> TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn build(self) -> TrackIndexedDBForStorageKeyParams<'a> {
        TrackIndexedDBForStorageKeyParams {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> TrackIndexedDBForStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.trackIndexedDBForStorageKey"; }

impl<'a> crate::CdpCommand<'a> for TrackIndexedDBForStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.trackIndexedDBForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Unregisters origin from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForOriginParams<'a> {
    pub fn builder() -> UntrackCacheStorageForOriginParamsBuilder<'a> { UntrackCacheStorageForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct UntrackCacheStorageForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> UntrackCacheStorageForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> UntrackCacheStorageForOriginParams<'a> {
        UntrackCacheStorageForOriginParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

impl<'a> UntrackCacheStorageForOriginParams<'a> { pub const METHOD: &'static str = "Storage.untrackCacheStorageForOrigin"; }

impl<'a> crate::CdpCommand<'a> for UntrackCacheStorageForOriginParams<'a> {
    const METHOD: &'static str = "Storage.untrackCacheStorageForOrigin";
    type Response = crate::EmptyReturns;
}

/// Unregisters storage key from receiving notifications for cache storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForStorageKeyParams<'a> {
    /// Storage key.
    storageKey: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForStorageKeyParams<'a> {
    pub fn builder() -> UntrackCacheStorageForStorageKeyParamsBuilder<'a> { UntrackCacheStorageForStorageKeyParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}

#[derive(Default)]
pub struct UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
}

impl<'a> UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn build(self) -> UntrackCacheStorageForStorageKeyParams<'a> {
        UntrackCacheStorageForStorageKeyParams {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> UntrackCacheStorageForStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.untrackCacheStorageForStorageKey"; }

impl<'a> crate::CdpCommand<'a> for UntrackCacheStorageForStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.untrackCacheStorageForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Unregisters origin from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForOriginParams<'a> {
    /// Security origin.
    origin: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForOriginParams<'a> {
    pub fn builder() -> UntrackIndexedDBForOriginParamsBuilder<'a> { UntrackIndexedDBForOriginParamsBuilder::default() }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}

#[derive(Default)]
pub struct UntrackIndexedDBForOriginParamsBuilder<'a> {
    origin: Option<Cow<'a, str>>,
}

impl<'a> UntrackIndexedDBForOriginParamsBuilder<'a> {
    /// Security origin.
    pub fn origin(mut self, origin: impl Into<Cow<'a, str>>) -> Self { self.origin = Some(origin.into()); self }
    pub fn build(self) -> UntrackIndexedDBForOriginParams<'a> {
        UntrackIndexedDBForOriginParams {
            origin: self.origin.unwrap_or_default(),
        }
    }
}

impl<'a> UntrackIndexedDBForOriginParams<'a> { pub const METHOD: &'static str = "Storage.untrackIndexedDBForOrigin"; }

impl<'a> crate::CdpCommand<'a> for UntrackIndexedDBForOriginParams<'a> {
    const METHOD: &'static str = "Storage.untrackIndexedDBForOrigin";
    type Response = crate::EmptyReturns;
}

/// Unregisters storage key from receiving notifications for IndexedDB.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForStorageKeyParams<'a> {
    /// Storage key.
    storageKey: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForStorageKeyParams<'a> {
    pub fn builder() -> UntrackIndexedDBForStorageKeyParamsBuilder<'a> { UntrackIndexedDBForStorageKeyParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}

#[derive(Default)]
pub struct UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
}

impl<'a> UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn build(self) -> UntrackIndexedDBForStorageKeyParams<'a> {
        UntrackIndexedDBForStorageKeyParams {
            storageKey: self.storageKey.unwrap_or_default(),
        }
    }
}

impl<'a> UntrackIndexedDBForStorageKeyParams<'a> { pub const METHOD: &'static str = "Storage.untrackIndexedDBForStorageKey"; }

impl<'a> crate::CdpCommand<'a> for UntrackIndexedDBForStorageKeyParams<'a> {
    const METHOD: &'static str = "Storage.untrackIndexedDBForStorageKey";
    type Response = crate::EmptyReturns;
}

/// Returns the number of stored Trust Tokens per issuer for the
/// current browsing context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTrustTokensReturns<'a> {
    tokens: Vec<TrustTokens<'a>>,
}

impl<'a> GetTrustTokensReturns<'a> {
    pub fn builder() -> GetTrustTokensReturnsBuilder<'a> { GetTrustTokensReturnsBuilder::default() }
    pub fn tokens(&self) -> &[TrustTokens<'a>] { &self.tokens }
}

#[derive(Default)]
pub struct GetTrustTokensReturnsBuilder<'a> {
    tokens: Option<Vec<TrustTokens<'a>>>,
}

impl<'a> GetTrustTokensReturnsBuilder<'a> {
    pub fn tokens(mut self, tokens: Vec<TrustTokens<'a>>) -> Self { self.tokens = Some(tokens); self }
    pub fn build(self) -> GetTrustTokensReturns<'a> {
        GetTrustTokensReturns {
            tokens: self.tokens.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTrustTokensParams {}

impl GetTrustTokensParams {
    pub fn builder() -> GetTrustTokensParamsBuilder {
        GetTrustTokensParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetTrustTokensParamsBuilder {}

impl GetTrustTokensParamsBuilder {
    pub fn build(self) -> GetTrustTokensParams {
        GetTrustTokensParams {}
    }
}

impl GetTrustTokensParams { pub const METHOD: &'static str = "Storage.getTrustTokens"; }

impl<'a> crate::CdpCommand<'a> for GetTrustTokensParams {
    const METHOD: &'static str = "Storage.getTrustTokens";
    type Response = GetTrustTokensReturns<'a>;
}

/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensParams<'a> {
    issuerOrigin: Cow<'a, str>,
}

impl<'a> ClearTrustTokensParams<'a> {
    pub fn builder() -> ClearTrustTokensParamsBuilder<'a> { ClearTrustTokensParamsBuilder::default() }
    pub fn issuerOrigin(&self) -> &str { self.issuerOrigin.as_ref() }
}

#[derive(Default)]
pub struct ClearTrustTokensParamsBuilder<'a> {
    issuerOrigin: Option<Cow<'a, str>>,
}

impl<'a> ClearTrustTokensParamsBuilder<'a> {
    pub fn issuerOrigin(mut self, issuerOrigin: impl Into<Cow<'a, str>>) -> Self { self.issuerOrigin = Some(issuerOrigin.into()); self }
    pub fn build(self) -> ClearTrustTokensParams<'a> {
        ClearTrustTokensParams {
            issuerOrigin: self.issuerOrigin.unwrap_or_default(),
        }
    }
}

/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensReturns {
    /// True if any tokens were deleted, false otherwise.
    didDeleteTokens: bool,
}

impl ClearTrustTokensReturns {
    pub fn builder() -> ClearTrustTokensReturnsBuilder { ClearTrustTokensReturnsBuilder::default() }
    pub fn didDeleteTokens(&self) -> bool { self.didDeleteTokens }
}

#[derive(Default)]
pub struct ClearTrustTokensReturnsBuilder {
    didDeleteTokens: Option<bool>,
}

impl ClearTrustTokensReturnsBuilder {
    /// True if any tokens were deleted, false otherwise.
    pub fn didDeleteTokens(mut self, didDeleteTokens: bool) -> Self { self.didDeleteTokens = Some(didDeleteTokens); self }
    pub fn build(self) -> ClearTrustTokensReturns {
        ClearTrustTokensReturns {
            didDeleteTokens: self.didDeleteTokens.unwrap_or_default(),
        }
    }
}

impl<'a> ClearTrustTokensParams<'a> { pub const METHOD: &'static str = "Storage.clearTrustTokens"; }

impl<'a> crate::CdpCommand<'a> for ClearTrustTokensParams<'a> {
    const METHOD: &'static str = "Storage.clearTrustTokens";
    type Response = ClearTrustTokensReturns;
}

/// Gets details for a named interest group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestGroupDetailsParams<'a> {
    ownerOrigin: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> GetInterestGroupDetailsParams<'a> {
    pub fn builder() -> GetInterestGroupDetailsParamsBuilder<'a> { GetInterestGroupDetailsParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
}

#[derive(Default)]
pub struct GetInterestGroupDetailsParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
}

impl<'a> GetInterestGroupDetailsParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> GetInterestGroupDetailsParams<'a> {
        GetInterestGroupDetailsParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
        }
    }
}

/// Gets details for a named interest group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestGroupDetailsReturns {
    /// This largely corresponds to:
    /// https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup
    /// but has absolute expirationTime instead of relative lifetimeMs and
    /// also adds joiningOrigin.
    details: serde_json::Map<String, JsonValue>,
}

impl GetInterestGroupDetailsReturns {
    pub fn builder() -> GetInterestGroupDetailsReturnsBuilder { GetInterestGroupDetailsReturnsBuilder::default() }
    pub fn details(&self) -> &serde_json::Map<String, JsonValue> { &self.details }
}

#[derive(Default)]
pub struct GetInterestGroupDetailsReturnsBuilder {
    details: Option<serde_json::Map<String, JsonValue>>,
}

impl GetInterestGroupDetailsReturnsBuilder {
    /// This largely corresponds to:
    /// https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup
    /// but has absolute expirationTime instead of relative lifetimeMs and
    /// also adds joiningOrigin.
    pub fn details(mut self, details: serde_json::Map<String, JsonValue>) -> Self { self.details = Some(details); self }
    pub fn build(self) -> GetInterestGroupDetailsReturns {
        GetInterestGroupDetailsReturns {
            details: self.details.unwrap_or_default(),
        }
    }
}

impl<'a> GetInterestGroupDetailsParams<'a> { pub const METHOD: &'static str = "Storage.getInterestGroupDetails"; }

impl<'a> crate::CdpCommand<'a> for GetInterestGroupDetailsParams<'a> {
    const METHOD: &'static str = "Storage.getInterestGroupDetails";
    type Response = GetInterestGroupDetailsReturns;
}

/// Enables/Disables issuing of interestGroupAccessed events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupTrackingParams {
    enable: bool,
}

impl SetInterestGroupTrackingParams {
    pub fn builder() -> SetInterestGroupTrackingParamsBuilder { SetInterestGroupTrackingParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct SetInterestGroupTrackingParamsBuilder {
    enable: Option<bool>,
}

impl SetInterestGroupTrackingParamsBuilder {
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> SetInterestGroupTrackingParams {
        SetInterestGroupTrackingParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl SetInterestGroupTrackingParams { pub const METHOD: &'static str = "Storage.setInterestGroupTracking"; }

impl<'a> crate::CdpCommand<'a> for SetInterestGroupTrackingParams {
    const METHOD: &'static str = "Storage.setInterestGroupTracking";
    type Response = crate::EmptyReturns;
}

/// Enables/Disables issuing of interestGroupAuctionEventOccurred and
/// interestGroupAuctionNetworkRequestCreated.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupAuctionTrackingParams {
    enable: bool,
}

impl SetInterestGroupAuctionTrackingParams {
    pub fn builder() -> SetInterestGroupAuctionTrackingParamsBuilder { SetInterestGroupAuctionTrackingParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct SetInterestGroupAuctionTrackingParamsBuilder {
    enable: Option<bool>,
}

impl SetInterestGroupAuctionTrackingParamsBuilder {
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> SetInterestGroupAuctionTrackingParams {
        SetInterestGroupAuctionTrackingParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl SetInterestGroupAuctionTrackingParams { pub const METHOD: &'static str = "Storage.setInterestGroupAuctionTracking"; }

impl<'a> crate::CdpCommand<'a> for SetInterestGroupAuctionTrackingParams {
    const METHOD: &'static str = "Storage.setInterestGroupAuctionTracking";
    type Response = crate::EmptyReturns;
}

/// Gets metadata for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageMetadataParams<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> GetSharedStorageMetadataParams<'a> {
    pub fn builder() -> GetSharedStorageMetadataParamsBuilder<'a> { GetSharedStorageMetadataParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}

#[derive(Default)]
pub struct GetSharedStorageMetadataParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
}

impl<'a> GetSharedStorageMetadataParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn build(self) -> GetSharedStorageMetadataParams<'a> {
        GetSharedStorageMetadataParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
        }
    }
}

/// Gets metadata for an origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageMetadataReturns {
    metadata: SharedStorageMetadata,
}

impl GetSharedStorageMetadataReturns {
    pub fn builder() -> GetSharedStorageMetadataReturnsBuilder { GetSharedStorageMetadataReturnsBuilder::default() }
    pub fn metadata(&self) -> &SharedStorageMetadata { &self.metadata }
}

#[derive(Default)]
pub struct GetSharedStorageMetadataReturnsBuilder {
    metadata: Option<SharedStorageMetadata>,
}

impl GetSharedStorageMetadataReturnsBuilder {
    pub fn metadata(mut self, metadata: SharedStorageMetadata) -> Self { self.metadata = Some(metadata); self }
    pub fn build(self) -> GetSharedStorageMetadataReturns {
        GetSharedStorageMetadataReturns {
            metadata: self.metadata.unwrap_or_default(),
        }
    }
}

impl<'a> GetSharedStorageMetadataParams<'a> { pub const METHOD: &'static str = "Storage.getSharedStorageMetadata"; }

impl<'a> crate::CdpCommand<'a> for GetSharedStorageMetadataParams<'a> {
    const METHOD: &'static str = "Storage.getSharedStorageMetadata";
    type Response = GetSharedStorageMetadataReturns;
}

/// Gets the entries in an given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageEntriesParams<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> GetSharedStorageEntriesParams<'a> {
    pub fn builder() -> GetSharedStorageEntriesParamsBuilder<'a> { GetSharedStorageEntriesParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}

#[derive(Default)]
pub struct GetSharedStorageEntriesParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
}

impl<'a> GetSharedStorageEntriesParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn build(self) -> GetSharedStorageEntriesParams<'a> {
        GetSharedStorageEntriesParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
        }
    }
}

/// Gets the entries in an given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSharedStorageEntriesReturns<'a> {
    entries: Vec<SharedStorageEntry<'a>>,
}

impl<'a> GetSharedStorageEntriesReturns<'a> {
    pub fn builder() -> GetSharedStorageEntriesReturnsBuilder<'a> { GetSharedStorageEntriesReturnsBuilder::default() }
    pub fn entries(&self) -> &[SharedStorageEntry<'a>] { &self.entries }
}

#[derive(Default)]
pub struct GetSharedStorageEntriesReturnsBuilder<'a> {
    entries: Option<Vec<SharedStorageEntry<'a>>>,
}

impl<'a> GetSharedStorageEntriesReturnsBuilder<'a> {
    pub fn entries(mut self, entries: Vec<SharedStorageEntry<'a>>) -> Self { self.entries = Some(entries); self }
    pub fn build(self) -> GetSharedStorageEntriesReturns<'a> {
        GetSharedStorageEntriesReturns {
            entries: self.entries.unwrap_or_default(),
        }
    }
}

impl<'a> GetSharedStorageEntriesParams<'a> { pub const METHOD: &'static str = "Storage.getSharedStorageEntries"; }

impl<'a> crate::CdpCommand<'a> for GetSharedStorageEntriesParams<'a> {
    const METHOD: &'static str = "Storage.getSharedStorageEntries";
    type Response = GetSharedStorageEntriesReturns<'a>;
}

/// Sets entry with 'key' and 'value' for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageEntryParams<'a> {
    ownerOrigin: Cow<'a, str>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    ignoreIfPresent: Option<bool>,
}

impl<'a> SetSharedStorageEntryParams<'a> {
    pub fn builder() -> SetSharedStorageEntryParamsBuilder<'a> { SetSharedStorageEntryParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn ignoreIfPresent(&self) -> Option<bool> { self.ignoreIfPresent }
}

#[derive(Default)]
pub struct SetSharedStorageEntryParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    ignoreIfPresent: Option<bool>,
}

impl<'a> SetSharedStorageEntryParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    pub fn ignoreIfPresent(mut self, ignoreIfPresent: bool) -> Self { self.ignoreIfPresent = Some(ignoreIfPresent); self }
    pub fn build(self) -> SetSharedStorageEntryParams<'a> {
        SetSharedStorageEntryParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            ignoreIfPresent: self.ignoreIfPresent,
        }
    }
}

impl<'a> SetSharedStorageEntryParams<'a> { pub const METHOD: &'static str = "Storage.setSharedStorageEntry"; }

impl<'a> crate::CdpCommand<'a> for SetSharedStorageEntryParams<'a> {
    const METHOD: &'static str = "Storage.setSharedStorageEntry";
    type Response = crate::EmptyReturns;
}

/// Deletes entry for 'key' (if it exists) for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSharedStorageEntryParams<'a> {
    ownerOrigin: Cow<'a, str>,
    key: Cow<'a, str>,
}

impl<'a> DeleteSharedStorageEntryParams<'a> {
    pub fn builder() -> DeleteSharedStorageEntryParamsBuilder<'a> { DeleteSharedStorageEntryParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
}

#[derive(Default)]
pub struct DeleteSharedStorageEntryParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
}

impl<'a> DeleteSharedStorageEntryParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    pub fn build(self) -> DeleteSharedStorageEntryParams<'a> {
        DeleteSharedStorageEntryParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
        }
    }
}

impl<'a> DeleteSharedStorageEntryParams<'a> { pub const METHOD: &'static str = "Storage.deleteSharedStorageEntry"; }

impl<'a> crate::CdpCommand<'a> for DeleteSharedStorageEntryParams<'a> {
    const METHOD: &'static str = "Storage.deleteSharedStorageEntry";
    type Response = crate::EmptyReturns;
}

/// Clears all entries for a given origin's shared storage.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearSharedStorageEntriesParams<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> ClearSharedStorageEntriesParams<'a> {
    pub fn builder() -> ClearSharedStorageEntriesParamsBuilder<'a> { ClearSharedStorageEntriesParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}

#[derive(Default)]
pub struct ClearSharedStorageEntriesParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
}

impl<'a> ClearSharedStorageEntriesParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn build(self) -> ClearSharedStorageEntriesParams<'a> {
        ClearSharedStorageEntriesParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
        }
    }
}

impl<'a> ClearSharedStorageEntriesParams<'a> { pub const METHOD: &'static str = "Storage.clearSharedStorageEntries"; }

impl<'a> crate::CdpCommand<'a> for ClearSharedStorageEntriesParams<'a> {
    const METHOD: &'static str = "Storage.clearSharedStorageEntries";
    type Response = crate::EmptyReturns;
}

/// Resets the budget for 'ownerOrigin' by clearing all budget withdrawals.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetSharedStorageBudgetParams<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> ResetSharedStorageBudgetParams<'a> {
    pub fn builder() -> ResetSharedStorageBudgetParamsBuilder<'a> { ResetSharedStorageBudgetParamsBuilder::default() }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}

#[derive(Default)]
pub struct ResetSharedStorageBudgetParamsBuilder<'a> {
    ownerOrigin: Option<Cow<'a, str>>,
}

impl<'a> ResetSharedStorageBudgetParamsBuilder<'a> {
    pub fn ownerOrigin(mut self, ownerOrigin: impl Into<Cow<'a, str>>) -> Self { self.ownerOrigin = Some(ownerOrigin.into()); self }
    pub fn build(self) -> ResetSharedStorageBudgetParams<'a> {
        ResetSharedStorageBudgetParams {
            ownerOrigin: self.ownerOrigin.unwrap_or_default(),
        }
    }
}

impl<'a> ResetSharedStorageBudgetParams<'a> { pub const METHOD: &'static str = "Storage.resetSharedStorageBudget"; }

impl<'a> crate::CdpCommand<'a> for ResetSharedStorageBudgetParams<'a> {
    const METHOD: &'static str = "Storage.resetSharedStorageBudget";
    type Response = crate::EmptyReturns;
}

/// Enables/disables issuing of sharedStorageAccessed events.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageTrackingParams {
    enable: bool,
}

impl SetSharedStorageTrackingParams {
    pub fn builder() -> SetSharedStorageTrackingParamsBuilder { SetSharedStorageTrackingParamsBuilder::default() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct SetSharedStorageTrackingParamsBuilder {
    enable: Option<bool>,
}

impl SetSharedStorageTrackingParamsBuilder {
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> SetSharedStorageTrackingParams {
        SetSharedStorageTrackingParams {
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl SetSharedStorageTrackingParams { pub const METHOD: &'static str = "Storage.setSharedStorageTracking"; }

impl<'a> crate::CdpCommand<'a> for SetSharedStorageTrackingParams {
    const METHOD: &'static str = "Storage.setSharedStorageTracking";
    type Response = crate::EmptyReturns;
}

/// Set tracking for a storage key's buckets.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageBucketTrackingParams<'a> {
    storageKey: Cow<'a, str>,
    enable: bool,
}

impl<'a> SetStorageBucketTrackingParams<'a> {
    pub fn builder() -> SetStorageBucketTrackingParamsBuilder<'a> { SetStorageBucketTrackingParamsBuilder::default() }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn enable(&self) -> bool { self.enable }
}

#[derive(Default)]
pub struct SetStorageBucketTrackingParamsBuilder<'a> {
    storageKey: Option<Cow<'a, str>>,
    enable: Option<bool>,
}

impl<'a> SetStorageBucketTrackingParamsBuilder<'a> {
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    pub fn enable(mut self, enable: bool) -> Self { self.enable = Some(enable); self }
    pub fn build(self) -> SetStorageBucketTrackingParams<'a> {
        SetStorageBucketTrackingParams {
            storageKey: self.storageKey.unwrap_or_default(),
            enable: self.enable.unwrap_or_default(),
        }
    }
}

impl<'a> SetStorageBucketTrackingParams<'a> { pub const METHOD: &'static str = "Storage.setStorageBucketTracking"; }

impl<'a> crate::CdpCommand<'a> for SetStorageBucketTrackingParams<'a> {
    const METHOD: &'static str = "Storage.setStorageBucketTracking";
    type Response = crate::EmptyReturns;
}

/// Deletes the Storage Bucket with the given storage key and bucket name.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStorageBucketParams<'a> {
    bucket: StorageBucket<'a>,
}

impl<'a> DeleteStorageBucketParams<'a> {
    pub fn builder() -> DeleteStorageBucketParamsBuilder<'a> { DeleteStorageBucketParamsBuilder::default() }
    pub fn bucket(&self) -> &StorageBucket<'a> { &self.bucket }
}

#[derive(Default)]
pub struct DeleteStorageBucketParamsBuilder<'a> {
    bucket: Option<StorageBucket<'a>>,
}

impl<'a> DeleteStorageBucketParamsBuilder<'a> {
    pub fn bucket(mut self, bucket: StorageBucket<'a>) -> Self { self.bucket = Some(bucket); self }
    pub fn build(self) -> DeleteStorageBucketParams<'a> {
        DeleteStorageBucketParams {
            bucket: self.bucket.unwrap_or_default(),
        }
    }
}

impl<'a> DeleteStorageBucketParams<'a> { pub const METHOD: &'static str = "Storage.deleteStorageBucket"; }

impl<'a> crate::CdpCommand<'a> for DeleteStorageBucketParams<'a> {
    const METHOD: &'static str = "Storage.deleteStorageBucket";
    type Response = crate::EmptyReturns;
}

/// Deletes state for sites identified as potential bounce trackers, immediately.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunBounceTrackingMitigationsReturns<'a> {
    deletedSites: Vec<Cow<'a, str>>,
}

impl<'a> RunBounceTrackingMitigationsReturns<'a> {
    pub fn builder() -> RunBounceTrackingMitigationsReturnsBuilder<'a> { RunBounceTrackingMitigationsReturnsBuilder::default() }
    pub fn deletedSites(&self) -> &[Cow<'a, str>] { &self.deletedSites }
}

#[derive(Default)]
pub struct RunBounceTrackingMitigationsReturnsBuilder<'a> {
    deletedSites: Option<Vec<Cow<'a, str>>>,
}

impl<'a> RunBounceTrackingMitigationsReturnsBuilder<'a> {
    pub fn deletedSites(mut self, deletedSites: Vec<Cow<'a, str>>) -> Self { self.deletedSites = Some(deletedSites); self }
    pub fn build(self) -> RunBounceTrackingMitigationsReturns<'a> {
        RunBounceTrackingMitigationsReturns {
            deletedSites: self.deletedSites.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunBounceTrackingMitigationsParams {}

impl RunBounceTrackingMitigationsParams {
    pub fn builder() -> RunBounceTrackingMitigationsParamsBuilder {
        RunBounceTrackingMitigationsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct RunBounceTrackingMitigationsParamsBuilder {}

impl RunBounceTrackingMitigationsParamsBuilder {
    pub fn build(self) -> RunBounceTrackingMitigationsParams {
        RunBounceTrackingMitigationsParams {}
    }
}

impl RunBounceTrackingMitigationsParams { pub const METHOD: &'static str = "Storage.runBounceTrackingMitigations"; }

impl<'a> crate::CdpCommand<'a> for RunBounceTrackingMitigationsParams {
    const METHOD: &'static str = "Storage.runBounceTrackingMitigations";
    type Response = RunBounceTrackingMitigationsReturns<'a>;
}

/// Returns the effective Related Website Sets in use by this profile for the browser
/// session. The effective Related Website Sets will not change during a browser session.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetRelatedWebsiteSetsReturns<'a> {
    sets: Vec<RelatedWebsiteSet<'a>>,
}

impl<'a> GetRelatedWebsiteSetsReturns<'a> {
    pub fn builder() -> GetRelatedWebsiteSetsReturnsBuilder<'a> { GetRelatedWebsiteSetsReturnsBuilder::default() }
    pub fn sets(&self) -> &[RelatedWebsiteSet<'a>] { &self.sets }
}

#[derive(Default)]
pub struct GetRelatedWebsiteSetsReturnsBuilder<'a> {
    sets: Option<Vec<RelatedWebsiteSet<'a>>>,
}

impl<'a> GetRelatedWebsiteSetsReturnsBuilder<'a> {
    pub fn sets(mut self, sets: Vec<RelatedWebsiteSet<'a>>) -> Self { self.sets = Some(sets); self }
    pub fn build(self) -> GetRelatedWebsiteSetsReturns<'a> {
        GetRelatedWebsiteSetsReturns {
            sets: self.sets.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRelatedWebsiteSetsParams {}

impl GetRelatedWebsiteSetsParams {
    pub fn builder() -> GetRelatedWebsiteSetsParamsBuilder {
        GetRelatedWebsiteSetsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct GetRelatedWebsiteSetsParamsBuilder {}

impl GetRelatedWebsiteSetsParamsBuilder {
    pub fn build(self) -> GetRelatedWebsiteSetsParams {
        GetRelatedWebsiteSetsParams {}
    }
}

impl GetRelatedWebsiteSetsParams { pub const METHOD: &'static str = "Storage.getRelatedWebsiteSets"; }

impl<'a> crate::CdpCommand<'a> for GetRelatedWebsiteSetsParams {
    const METHOD: &'static str = "Storage.getRelatedWebsiteSets";
    type Response = GetRelatedWebsiteSetsReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetProtectedAudienceKAnonymityParams<'a> {
    owner: Cow<'a, str>,
    name: Cow<'a, str>,
    hashes: Vec<Cow<'a, str>>,
}

impl<'a> SetProtectedAudienceKAnonymityParams<'a> {
    pub fn builder() -> SetProtectedAudienceKAnonymityParamsBuilder<'a> { SetProtectedAudienceKAnonymityParamsBuilder::default() }
    pub fn owner(&self) -> &str { self.owner.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn hashes(&self) -> &[Cow<'a, str>] { &self.hashes }
}

#[derive(Default)]
pub struct SetProtectedAudienceKAnonymityParamsBuilder<'a> {
    owner: Option<Cow<'a, str>>,
    name: Option<Cow<'a, str>>,
    hashes: Option<Vec<Cow<'a, str>>>,
}

impl<'a> SetProtectedAudienceKAnonymityParamsBuilder<'a> {
    pub fn owner(mut self, owner: impl Into<Cow<'a, str>>) -> Self { self.owner = Some(owner.into()); self }
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn hashes(mut self, hashes: Vec<Cow<'a, str>>) -> Self { self.hashes = Some(hashes); self }
    pub fn build(self) -> SetProtectedAudienceKAnonymityParams<'a> {
        SetProtectedAudienceKAnonymityParams {
            owner: self.owner.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            hashes: self.hashes.unwrap_or_default(),
        }
    }
}

impl<'a> SetProtectedAudienceKAnonymityParams<'a> { pub const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity"; }

impl<'a> crate::CdpCommand<'a> for SetProtectedAudienceKAnonymityParams<'a> {
    const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity";
    type Response = crate::EmptyReturns;
}
