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
    pub fn builder(storageType: impl Into<StorageType>, usage: f64) -> UsageForTypeBuilder {
        UsageForTypeBuilder {
            storageType: storageType.into(),
            usage: usage,
        }
    }
    pub fn storageType(&self) -> &StorageType { &self.storageType }
    pub fn usage(&self) -> f64 { self.usage }
}


pub struct UsageForTypeBuilder {
    storageType: StorageType,
    usage: f64,
}

impl UsageForTypeBuilder {
    pub fn build(self) -> UsageForType {
        UsageForType {
            storageType: self.storageType,
            usage: self.usage,
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
    pub fn builder(issuerOrigin: impl Into<Cow<'a, str>>, count: f64) -> TrustTokensBuilder<'a> {
        TrustTokensBuilder {
            issuerOrigin: issuerOrigin.into(),
            count: count,
        }
    }
    pub fn issuerOrigin(&self) -> &str { self.issuerOrigin.as_ref() }
    pub fn count(&self) -> f64 { self.count }
}


pub struct TrustTokensBuilder<'a> {
    issuerOrigin: Cow<'a, str>,
    count: f64,
}

impl<'a> TrustTokensBuilder<'a> {
    pub fn build(self) -> TrustTokens<'a> {
        TrustTokens {
            issuerOrigin: self.issuerOrigin,
            count: self.count,
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
    pub fn builder(key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SharedStorageEntryBuilder<'a> {
        SharedStorageEntryBuilder {
            key: key.into(),
            value: value.into(),
        }
    }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
}


pub struct SharedStorageEntryBuilder<'a> {
    key: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> SharedStorageEntryBuilder<'a> {
    pub fn build(self) -> SharedStorageEntry<'a> {
        SharedStorageEntry {
            key: self.key,
            value: self.value,
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
    pub fn builder(creationTime: crate::network::TimeSinceEpoch, length: u64, remainingBudget: f64, bytesUsed: i64) -> SharedStorageMetadataBuilder {
        SharedStorageMetadataBuilder {
            creationTime: creationTime,
            length: length,
            remainingBudget: remainingBudget,
            bytesUsed: bytesUsed,
        }
    }
    pub fn creationTime(&self) -> &crate::network::TimeSinceEpoch { &self.creationTime }
    pub fn length(&self) -> u64 { self.length }
    pub fn remainingBudget(&self) -> f64 { self.remainingBudget }
    pub fn bytesUsed(&self) -> i64 { self.bytesUsed }
}


pub struct SharedStorageMetadataBuilder {
    creationTime: crate::network::TimeSinceEpoch,
    length: u64,
    remainingBudget: f64,
    bytesUsed: i64,
}

impl SharedStorageMetadataBuilder {
    pub fn build(self) -> SharedStorageMetadata {
        SharedStorageMetadata {
            creationTime: self.creationTime,
            length: self.length,
            remainingBudget: self.remainingBudget,
            bytesUsed: self.bytesUsed,
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
    pub fn builder(filteringIdMaxBytes: u64) -> SharedStoragePrivateAggregationConfigBuilder<'a> {
        SharedStoragePrivateAggregationConfigBuilder {
            aggregationCoordinatorOrigin: None,
            contextId: None,
            filteringIdMaxBytes: filteringIdMaxBytes,
            maxContributions: None,
        }
    }
    pub fn aggregationCoordinatorOrigin(&self) -> Option<&str> { self.aggregationCoordinatorOrigin.as_deref() }
    pub fn contextId(&self) -> Option<&str> { self.contextId.as_deref() }
    pub fn filteringIdMaxBytes(&self) -> u64 { self.filteringIdMaxBytes }
    pub fn maxContributions(&self) -> Option<i64> { self.maxContributions }
}


pub struct SharedStoragePrivateAggregationConfigBuilder<'a> {
    aggregationCoordinatorOrigin: Option<Cow<'a, str>>,
    contextId: Option<Cow<'a, str>>,
    filteringIdMaxBytes: u64,
    maxContributions: Option<i64>,
}

impl<'a> SharedStoragePrivateAggregationConfigBuilder<'a> {
    /// The chosen aggregation service deployment.
    pub fn aggregationCoordinatorOrigin(mut self, aggregationCoordinatorOrigin: impl Into<Cow<'a, str>>) -> Self { self.aggregationCoordinatorOrigin = Some(aggregationCoordinatorOrigin.into()); self }
    /// The context ID provided.
    pub fn contextId(mut self, contextId: impl Into<Cow<'a, str>>) -> Self { self.contextId = Some(contextId.into()); self }
    /// The limit on the number of contributions in the final report.
    pub fn maxContributions(mut self, maxContributions: i64) -> Self { self.maxContributions = Some(maxContributions); self }
    pub fn build(self) -> SharedStoragePrivateAggregationConfig<'a> {
        SharedStoragePrivateAggregationConfig {
            aggregationCoordinatorOrigin: self.aggregationCoordinatorOrigin,
            contextId: self.contextId,
            filteringIdMaxBytes: self.filteringIdMaxBytes,
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
    pub fn builder(eventType: impl Into<Cow<'a, str>>, reportingUrl: impl Into<Cow<'a, str>>) -> SharedStorageReportingMetadataBuilder<'a> {
        SharedStorageReportingMetadataBuilder {
            eventType: eventType.into(),
            reportingUrl: reportingUrl.into(),
        }
    }
    pub fn eventType(&self) -> &str { self.eventType.as_ref() }
    pub fn reportingUrl(&self) -> &str { self.reportingUrl.as_ref() }
}


pub struct SharedStorageReportingMetadataBuilder<'a> {
    eventType: Cow<'a, str>,
    reportingUrl: Cow<'a, str>,
}

impl<'a> SharedStorageReportingMetadataBuilder<'a> {
    pub fn build(self) -> SharedStorageReportingMetadata<'a> {
        SharedStorageReportingMetadata {
            eventType: self.eventType,
            reportingUrl: self.reportingUrl,
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
    pub fn builder(url: impl Into<Cow<'a, str>>, reportingMetadata: Vec<SharedStorageReportingMetadata<'a>>) -> SharedStorageUrlWithMetadataBuilder<'a> {
        SharedStorageUrlWithMetadataBuilder {
            url: url.into(),
            reportingMetadata: reportingMetadata,
        }
    }
    pub fn url(&self) -> &str { self.url.as_ref() }
    pub fn reportingMetadata(&self) -> &[SharedStorageReportingMetadata<'a>] { &self.reportingMetadata }
}


pub struct SharedStorageUrlWithMetadataBuilder<'a> {
    url: Cow<'a, str>,
    reportingMetadata: Vec<SharedStorageReportingMetadata<'a>>,
}

impl<'a> SharedStorageUrlWithMetadataBuilder<'a> {
    pub fn build(self) -> SharedStorageUrlWithMetadata<'a> {
        SharedStorageUrlWithMetadata {
            url: self.url,
            reportingMetadata: self.reportingMetadata,
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
    pub fn builder() -> SharedStorageAccessParamsBuilder<'a> {
        SharedStorageAccessParamsBuilder {
            scriptSourceUrl: None,
            dataOrigin: None,
            operationName: None,
            operationId: None,
            keepAlive: None,
            privateAggregationConfig: None,
            serializedData: None,
            urlsWithMetadata: None,
            urnUuid: None,
            key: None,
            value: None,
            ignoreIfPresent: None,
            workletOrdinal: None,
            workletTargetId: None,
            withLock: None,
            batchUpdateId: None,
            batchSize: None,
        }
    }
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
    pub fn builder(storageKey: impl Into<SerializedStorageKey<'a>>) -> StorageBucketBuilder<'a> {
        StorageBucketBuilder {
            storageKey: storageKey.into(),
            name: None,
        }
    }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}


pub struct StorageBucketBuilder<'a> {
    storageKey: SerializedStorageKey<'a>,
    name: Option<Cow<'a, str>>,
}

impl<'a> StorageBucketBuilder<'a> {
    /// If not specified, it is the default bucket of the storageKey.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> StorageBucket<'a> {
        StorageBucket {
            storageKey: self.storageKey,
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
    pub fn builder(bucket: StorageBucket<'a>, id: impl Into<Cow<'a, str>>, expiration: crate::network::TimeSinceEpoch, quota: f64, persistent: bool, durability: impl Into<StorageBucketsDurability>) -> StorageBucketInfoBuilder<'a> {
        StorageBucketInfoBuilder {
            bucket: bucket,
            id: id.into(),
            expiration: expiration,
            quota: quota,
            persistent: persistent,
            durability: durability.into(),
        }
    }
    pub fn bucket(&self) -> &StorageBucket<'a> { &self.bucket }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn expiration(&self) -> &crate::network::TimeSinceEpoch { &self.expiration }
    pub fn quota(&self) -> f64 { self.quota }
    pub fn persistent(&self) -> bool { self.persistent }
    pub fn durability(&self) -> &StorageBucketsDurability { &self.durability }
}


pub struct StorageBucketInfoBuilder<'a> {
    bucket: StorageBucket<'a>,
    id: Cow<'a, str>,
    expiration: crate::network::TimeSinceEpoch,
    quota: f64,
    persistent: bool,
    durability: StorageBucketsDurability,
}

impl<'a> StorageBucketInfoBuilder<'a> {
    pub fn build(self) -> StorageBucketInfo<'a> {
        StorageBucketInfo {
            bucket: self.bucket,
            id: self.id,
            expiration: self.expiration,
            quota: self.quota,
            persistent: self.persistent,
            durability: self.durability,
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
    pub fn builder(primarySites: Vec<Cow<'a, str>>, associatedSites: Vec<Cow<'a, str>>, serviceSites: Vec<Cow<'a, str>>) -> RelatedWebsiteSetBuilder<'a> {
        RelatedWebsiteSetBuilder {
            primarySites: primarySites,
            associatedSites: associatedSites,
            serviceSites: serviceSites,
        }
    }
    pub fn primarySites(&self) -> &[Cow<'a, str>] { &self.primarySites }
    pub fn associatedSites(&self) -> &[Cow<'a, str>] { &self.associatedSites }
    pub fn serviceSites(&self) -> &[Cow<'a, str>] { &self.serviceSites }
}


pub struct RelatedWebsiteSetBuilder<'a> {
    primarySites: Vec<Cow<'a, str>>,
    associatedSites: Vec<Cow<'a, str>>,
    serviceSites: Vec<Cow<'a, str>>,
}

impl<'a> RelatedWebsiteSetBuilder<'a> {
    pub fn build(self) -> RelatedWebsiteSet<'a> {
        RelatedWebsiteSet {
            primarySites: self.primarySites,
            associatedSites: self.associatedSites,
            serviceSites: self.serviceSites,
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
    pub fn builder(frameId: crate::page::FrameId<'a>) -> GetStorageKeyForFrameParamsBuilder<'a> {
        GetStorageKeyForFrameParamsBuilder {
            frameId: frameId,
        }
    }
    pub fn frameId(&self) -> &crate::page::FrameId<'a> { &self.frameId }
}


pub struct GetStorageKeyForFrameParamsBuilder<'a> {
    frameId: crate::page::FrameId<'a>,
}

impl<'a> GetStorageKeyForFrameParamsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyForFrameParams<'a> {
        GetStorageKeyForFrameParams {
            frameId: self.frameId,
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
    pub fn builder(storageKey: impl Into<SerializedStorageKey<'a>>) -> GetStorageKeyForFrameReturnsBuilder<'a> {
        GetStorageKeyForFrameReturnsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
}


pub struct GetStorageKeyForFrameReturnsBuilder<'a> {
    storageKey: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyForFrameReturnsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyForFrameReturns<'a> {
        GetStorageKeyForFrameReturns {
            storageKey: self.storageKey,
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
    pub fn builder() -> GetStorageKeyParamsBuilder<'a> {
        GetStorageKeyParamsBuilder {
            frameId: None,
        }
    }
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
    pub fn builder(storageKey: impl Into<SerializedStorageKey<'a>>) -> GetStorageKeyReturnsBuilder<'a> {
        GetStorageKeyReturnsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &SerializedStorageKey<'a> { &self.storageKey }
}


pub struct GetStorageKeyReturnsBuilder<'a> {
    storageKey: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyReturnsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyReturns<'a> {
        GetStorageKeyReturns {
            storageKey: self.storageKey,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>, storageTypes: impl Into<Cow<'a, str>>) -> ClearDataForOriginParamsBuilder<'a> {
        ClearDataForOriginParamsBuilder {
            origin: origin.into(),
            storageTypes: storageTypes.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn storageTypes(&self) -> &str { self.storageTypes.as_ref() }
}


pub struct ClearDataForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
    storageTypes: Cow<'a, str>,
}

impl<'a> ClearDataForOriginParamsBuilder<'a> {
    pub fn build(self) -> ClearDataForOriginParams<'a> {
        ClearDataForOriginParams {
            origin: self.origin,
            storageTypes: self.storageTypes,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>, storageTypes: impl Into<Cow<'a, str>>) -> ClearDataForStorageKeyParamsBuilder<'a> {
        ClearDataForStorageKeyParamsBuilder {
            storageKey: storageKey.into(),
            storageTypes: storageTypes.into(),
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn storageTypes(&self) -> &str { self.storageTypes.as_ref() }
}


pub struct ClearDataForStorageKeyParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
    storageTypes: Cow<'a, str>,
}

impl<'a> ClearDataForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> ClearDataForStorageKeyParams<'a> {
        ClearDataForStorageKeyParams {
            storageKey: self.storageKey,
            storageTypes: self.storageTypes,
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
    pub fn builder() -> GetCookiesParamsBuilder<'a> {
        GetCookiesParamsBuilder {
            browserContextId: None,
        }
    }
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
    pub fn builder(cookies: Vec<crate::network::Cookie<'a>>) -> GetCookiesReturnsBuilder<'a> {
        GetCookiesReturnsBuilder {
            cookies: cookies,
        }
    }
    pub fn cookies(&self) -> &[crate::network::Cookie<'a>] { &self.cookies }
}


pub struct GetCookiesReturnsBuilder<'a> {
    cookies: Vec<crate::network::Cookie<'a>>,
}

impl<'a> GetCookiesReturnsBuilder<'a> {
    pub fn build(self) -> GetCookiesReturns<'a> {
        GetCookiesReturns {
            cookies: self.cookies,
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
    pub fn builder(cookies: Vec<crate::network::CookieParam<'a>>) -> SetCookiesParamsBuilder<'a> {
        SetCookiesParamsBuilder {
            cookies: cookies,
            browserContextId: None,
        }
    }
    pub fn cookies(&self) -> &[crate::network::CookieParam<'a>] { &self.cookies }
    pub fn browserContextId(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browserContextId.as_ref() }
}


pub struct SetCookiesParamsBuilder<'a> {
    cookies: Vec<crate::network::CookieParam<'a>>,
    browserContextId: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> SetCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browserContextId(mut self, browserContextId: crate::browser::BrowserContextID<'a>) -> Self { self.browserContextId = Some(browserContextId); self }
    pub fn build(self) -> SetCookiesParams<'a> {
        SetCookiesParams {
            cookies: self.cookies,
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
    pub fn builder() -> ClearCookiesParamsBuilder<'a> {
        ClearCookiesParamsBuilder {
            browserContextId: None,
        }
    }
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> GetUsageAndQuotaParamsBuilder<'a> {
        GetUsageAndQuotaParamsBuilder {
            origin: origin.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct GetUsageAndQuotaParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> GetUsageAndQuotaParamsBuilder<'a> {
    pub fn build(self) -> GetUsageAndQuotaParams<'a> {
        GetUsageAndQuotaParams {
            origin: self.origin,
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
    pub fn builder(usage: f64, quota: f64, overrideActive: bool, usageBreakdown: Vec<UsageForType>) -> GetUsageAndQuotaReturnsBuilder {
        GetUsageAndQuotaReturnsBuilder {
            usage: usage,
            quota: quota,
            overrideActive: overrideActive,
            usageBreakdown: usageBreakdown,
        }
    }
    pub fn usage(&self) -> f64 { self.usage }
    pub fn quota(&self) -> f64 { self.quota }
    pub fn overrideActive(&self) -> bool { self.overrideActive }
    pub fn usageBreakdown(&self) -> &[UsageForType] { &self.usageBreakdown }
}


pub struct GetUsageAndQuotaReturnsBuilder {
    usage: f64,
    quota: f64,
    overrideActive: bool,
    usageBreakdown: Vec<UsageForType>,
}

impl GetUsageAndQuotaReturnsBuilder {
    pub fn build(self) -> GetUsageAndQuotaReturns {
        GetUsageAndQuotaReturns {
            usage: self.usage,
            quota: self.quota,
            overrideActive: self.overrideActive,
            usageBreakdown: self.usageBreakdown,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> OverrideQuotaForOriginParamsBuilder<'a> {
        OverrideQuotaForOriginParamsBuilder {
            origin: origin.into(),
            quotaSize: None,
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    pub fn quotaSize(&self) -> Option<f64> { self.quotaSize }
}


pub struct OverrideQuotaForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
    quotaSize: Option<f64>,
}

impl<'a> OverrideQuotaForOriginParamsBuilder<'a> {
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
            origin: self.origin,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> TrackCacheStorageForOriginParamsBuilder<'a> {
        TrackCacheStorageForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct TrackCacheStorageForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForOriginParamsBuilder<'a> {
    pub fn build(self) -> TrackCacheStorageForOriginParams<'a> {
        TrackCacheStorageForOriginParams {
            origin: self.origin,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>) -> TrackCacheStorageForStorageKeyParamsBuilder<'a> {
        TrackCacheStorageForStorageKeyParamsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}


pub struct TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> TrackCacheStorageForStorageKeyParams<'a> {
        TrackCacheStorageForStorageKeyParams {
            storageKey: self.storageKey,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> TrackIndexedDBForOriginParamsBuilder<'a> {
        TrackIndexedDBForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct TrackIndexedDBForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForOriginParamsBuilder<'a> {
    pub fn build(self) -> TrackIndexedDBForOriginParams<'a> {
        TrackIndexedDBForOriginParams {
            origin: self.origin,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>) -> TrackIndexedDBForStorageKeyParamsBuilder<'a> {
        TrackIndexedDBForStorageKeyParamsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}


pub struct TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> TrackIndexedDBForStorageKeyParams<'a> {
        TrackIndexedDBForStorageKeyParams {
            storageKey: self.storageKey,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> UntrackCacheStorageForOriginParamsBuilder<'a> {
        UntrackCacheStorageForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct UntrackCacheStorageForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForOriginParamsBuilder<'a> {
    pub fn build(self) -> UntrackCacheStorageForOriginParams<'a> {
        UntrackCacheStorageForOriginParams {
            origin: self.origin,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>) -> UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
        UntrackCacheStorageForStorageKeyParamsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}


pub struct UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> UntrackCacheStorageForStorageKeyParams<'a> {
        UntrackCacheStorageForStorageKeyParams {
            storageKey: self.storageKey,
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
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> UntrackIndexedDBForOriginParamsBuilder<'a> {
        UntrackIndexedDBForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    pub fn origin(&self) -> &str { self.origin.as_ref() }
}


pub struct UntrackIndexedDBForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForOriginParamsBuilder<'a> {
    pub fn build(self) -> UntrackIndexedDBForOriginParams<'a> {
        UntrackIndexedDBForOriginParams {
            origin: self.origin,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>) -> UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
        UntrackIndexedDBForStorageKeyParamsBuilder {
            storageKey: storageKey.into(),
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
}


pub struct UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> UntrackIndexedDBForStorageKeyParams<'a> {
        UntrackIndexedDBForStorageKeyParams {
            storageKey: self.storageKey,
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
    pub fn builder(tokens: Vec<TrustTokens<'a>>) -> GetTrustTokensReturnsBuilder<'a> {
        GetTrustTokensReturnsBuilder {
            tokens: tokens,
        }
    }
    pub fn tokens(&self) -> &[TrustTokens<'a>] { &self.tokens }
}


pub struct GetTrustTokensReturnsBuilder<'a> {
    tokens: Vec<TrustTokens<'a>>,
}

impl<'a> GetTrustTokensReturnsBuilder<'a> {
    pub fn build(self) -> GetTrustTokensReturns<'a> {
        GetTrustTokensReturns {
            tokens: self.tokens,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTrustTokensParams {}

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
    pub fn builder(issuerOrigin: impl Into<Cow<'a, str>>) -> ClearTrustTokensParamsBuilder<'a> {
        ClearTrustTokensParamsBuilder {
            issuerOrigin: issuerOrigin.into(),
        }
    }
    pub fn issuerOrigin(&self) -> &str { self.issuerOrigin.as_ref() }
}


pub struct ClearTrustTokensParamsBuilder<'a> {
    issuerOrigin: Cow<'a, str>,
}

impl<'a> ClearTrustTokensParamsBuilder<'a> {
    pub fn build(self) -> ClearTrustTokensParams<'a> {
        ClearTrustTokensParams {
            issuerOrigin: self.issuerOrigin,
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
    pub fn builder(didDeleteTokens: bool) -> ClearTrustTokensReturnsBuilder {
        ClearTrustTokensReturnsBuilder {
            didDeleteTokens: didDeleteTokens,
        }
    }
    pub fn didDeleteTokens(&self) -> bool { self.didDeleteTokens }
}


pub struct ClearTrustTokensReturnsBuilder {
    didDeleteTokens: bool,
}

impl ClearTrustTokensReturnsBuilder {
    pub fn build(self) -> ClearTrustTokensReturns {
        ClearTrustTokensReturns {
            didDeleteTokens: self.didDeleteTokens,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>) -> GetInterestGroupDetailsParamsBuilder<'a> {
        GetInterestGroupDetailsParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
            name: name.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct GetInterestGroupDetailsParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> GetInterestGroupDetailsParamsBuilder<'a> {
    pub fn build(self) -> GetInterestGroupDetailsParams<'a> {
        GetInterestGroupDetailsParams {
            ownerOrigin: self.ownerOrigin,
            name: self.name,
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
    pub fn builder(details: serde_json::Map<String, JsonValue>) -> GetInterestGroupDetailsReturnsBuilder {
        GetInterestGroupDetailsReturnsBuilder {
            details: details,
        }
    }
    pub fn details(&self) -> &serde_json::Map<String, JsonValue> { &self.details }
}


pub struct GetInterestGroupDetailsReturnsBuilder {
    details: serde_json::Map<String, JsonValue>,
}

impl GetInterestGroupDetailsReturnsBuilder {
    pub fn build(self) -> GetInterestGroupDetailsReturns {
        GetInterestGroupDetailsReturns {
            details: self.details,
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
    pub fn builder(enable: bool) -> SetInterestGroupTrackingParamsBuilder {
        SetInterestGroupTrackingParamsBuilder {
            enable: enable,
        }
    }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetInterestGroupTrackingParamsBuilder {
    enable: bool,
}

impl SetInterestGroupTrackingParamsBuilder {
    pub fn build(self) -> SetInterestGroupTrackingParams {
        SetInterestGroupTrackingParams {
            enable: self.enable,
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
    pub fn builder(enable: bool) -> SetInterestGroupAuctionTrackingParamsBuilder {
        SetInterestGroupAuctionTrackingParamsBuilder {
            enable: enable,
        }
    }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetInterestGroupAuctionTrackingParamsBuilder {
    enable: bool,
}

impl SetInterestGroupAuctionTrackingParamsBuilder {
    pub fn build(self) -> SetInterestGroupAuctionTrackingParams {
        SetInterestGroupAuctionTrackingParams {
            enable: self.enable,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>) -> GetSharedStorageMetadataParamsBuilder<'a> {
        GetSharedStorageMetadataParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}


pub struct GetSharedStorageMetadataParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> GetSharedStorageMetadataParamsBuilder<'a> {
    pub fn build(self) -> GetSharedStorageMetadataParams<'a> {
        GetSharedStorageMetadataParams {
            ownerOrigin: self.ownerOrigin,
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
    pub fn builder(metadata: SharedStorageMetadata) -> GetSharedStorageMetadataReturnsBuilder {
        GetSharedStorageMetadataReturnsBuilder {
            metadata: metadata,
        }
    }
    pub fn metadata(&self) -> &SharedStorageMetadata { &self.metadata }
}


pub struct GetSharedStorageMetadataReturnsBuilder {
    metadata: SharedStorageMetadata,
}

impl GetSharedStorageMetadataReturnsBuilder {
    pub fn build(self) -> GetSharedStorageMetadataReturns {
        GetSharedStorageMetadataReturns {
            metadata: self.metadata,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>) -> GetSharedStorageEntriesParamsBuilder<'a> {
        GetSharedStorageEntriesParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}


pub struct GetSharedStorageEntriesParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> GetSharedStorageEntriesParamsBuilder<'a> {
    pub fn build(self) -> GetSharedStorageEntriesParams<'a> {
        GetSharedStorageEntriesParams {
            ownerOrigin: self.ownerOrigin,
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
    pub fn builder(entries: Vec<SharedStorageEntry<'a>>) -> GetSharedStorageEntriesReturnsBuilder<'a> {
        GetSharedStorageEntriesReturnsBuilder {
            entries: entries,
        }
    }
    pub fn entries(&self) -> &[SharedStorageEntry<'a>] { &self.entries }
}


pub struct GetSharedStorageEntriesReturnsBuilder<'a> {
    entries: Vec<SharedStorageEntry<'a>>,
}

impl<'a> GetSharedStorageEntriesReturnsBuilder<'a> {
    pub fn build(self) -> GetSharedStorageEntriesReturns<'a> {
        GetSharedStorageEntriesReturns {
            entries: self.entries,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetSharedStorageEntryParamsBuilder<'a> {
        SetSharedStorageEntryParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
            key: key.into(),
            value: value.into(),
            ignoreIfPresent: None,
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    pub fn ignoreIfPresent(&self) -> Option<bool> { self.ignoreIfPresent }
}


pub struct SetSharedStorageEntryParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    ignoreIfPresent: Option<bool>,
}

impl<'a> SetSharedStorageEntryParamsBuilder<'a> {
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    pub fn ignoreIfPresent(mut self, ignoreIfPresent: bool) -> Self { self.ignoreIfPresent = Some(ignoreIfPresent); self }
    pub fn build(self) -> SetSharedStorageEntryParams<'a> {
        SetSharedStorageEntryParams {
            ownerOrigin: self.ownerOrigin,
            key: self.key,
            value: self.value,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>, key: impl Into<Cow<'a, str>>) -> DeleteSharedStorageEntryParamsBuilder<'a> {
        DeleteSharedStorageEntryParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
            key: key.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
}


pub struct DeleteSharedStorageEntryParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
    key: Cow<'a, str>,
}

impl<'a> DeleteSharedStorageEntryParamsBuilder<'a> {
    pub fn build(self) -> DeleteSharedStorageEntryParams<'a> {
        DeleteSharedStorageEntryParams {
            ownerOrigin: self.ownerOrigin,
            key: self.key,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>) -> ClearSharedStorageEntriesParamsBuilder<'a> {
        ClearSharedStorageEntriesParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}


pub struct ClearSharedStorageEntriesParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> ClearSharedStorageEntriesParamsBuilder<'a> {
    pub fn build(self) -> ClearSharedStorageEntriesParams<'a> {
        ClearSharedStorageEntriesParams {
            ownerOrigin: self.ownerOrigin,
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
    pub fn builder(ownerOrigin: impl Into<Cow<'a, str>>) -> ResetSharedStorageBudgetParamsBuilder<'a> {
        ResetSharedStorageBudgetParamsBuilder {
            ownerOrigin: ownerOrigin.into(),
        }
    }
    pub fn ownerOrigin(&self) -> &str { self.ownerOrigin.as_ref() }
}


pub struct ResetSharedStorageBudgetParamsBuilder<'a> {
    ownerOrigin: Cow<'a, str>,
}

impl<'a> ResetSharedStorageBudgetParamsBuilder<'a> {
    pub fn build(self) -> ResetSharedStorageBudgetParams<'a> {
        ResetSharedStorageBudgetParams {
            ownerOrigin: self.ownerOrigin,
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
    pub fn builder(enable: bool) -> SetSharedStorageTrackingParamsBuilder {
        SetSharedStorageTrackingParamsBuilder {
            enable: enable,
        }
    }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetSharedStorageTrackingParamsBuilder {
    enable: bool,
}

impl SetSharedStorageTrackingParamsBuilder {
    pub fn build(self) -> SetSharedStorageTrackingParams {
        SetSharedStorageTrackingParams {
            enable: self.enable,
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
    pub fn builder(storageKey: impl Into<Cow<'a, str>>, enable: bool) -> SetStorageBucketTrackingParamsBuilder<'a> {
        SetStorageBucketTrackingParamsBuilder {
            storageKey: storageKey.into(),
            enable: enable,
        }
    }
    pub fn storageKey(&self) -> &str { self.storageKey.as_ref() }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetStorageBucketTrackingParamsBuilder<'a> {
    storageKey: Cow<'a, str>,
    enable: bool,
}

impl<'a> SetStorageBucketTrackingParamsBuilder<'a> {
    pub fn build(self) -> SetStorageBucketTrackingParams<'a> {
        SetStorageBucketTrackingParams {
            storageKey: self.storageKey,
            enable: self.enable,
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
    pub fn builder(bucket: StorageBucket<'a>) -> DeleteStorageBucketParamsBuilder<'a> {
        DeleteStorageBucketParamsBuilder {
            bucket: bucket,
        }
    }
    pub fn bucket(&self) -> &StorageBucket<'a> { &self.bucket }
}


pub struct DeleteStorageBucketParamsBuilder<'a> {
    bucket: StorageBucket<'a>,
}

impl<'a> DeleteStorageBucketParamsBuilder<'a> {
    pub fn build(self) -> DeleteStorageBucketParams<'a> {
        DeleteStorageBucketParams {
            bucket: self.bucket,
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
    pub fn builder(deletedSites: Vec<Cow<'a, str>>) -> RunBounceTrackingMitigationsReturnsBuilder<'a> {
        RunBounceTrackingMitigationsReturnsBuilder {
            deletedSites: deletedSites,
        }
    }
    pub fn deletedSites(&self) -> &[Cow<'a, str>] { &self.deletedSites }
}


pub struct RunBounceTrackingMitigationsReturnsBuilder<'a> {
    deletedSites: Vec<Cow<'a, str>>,
}

impl<'a> RunBounceTrackingMitigationsReturnsBuilder<'a> {
    pub fn build(self) -> RunBounceTrackingMitigationsReturns<'a> {
        RunBounceTrackingMitigationsReturns {
            deletedSites: self.deletedSites,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunBounceTrackingMitigationsParams {}

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
    pub fn builder(sets: Vec<RelatedWebsiteSet<'a>>) -> GetRelatedWebsiteSetsReturnsBuilder<'a> {
        GetRelatedWebsiteSetsReturnsBuilder {
            sets: sets,
        }
    }
    pub fn sets(&self) -> &[RelatedWebsiteSet<'a>] { &self.sets }
}


pub struct GetRelatedWebsiteSetsReturnsBuilder<'a> {
    sets: Vec<RelatedWebsiteSet<'a>>,
}

impl<'a> GetRelatedWebsiteSetsReturnsBuilder<'a> {
    pub fn build(self) -> GetRelatedWebsiteSetsReturns<'a> {
        GetRelatedWebsiteSetsReturns {
            sets: self.sets,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRelatedWebsiteSetsParams {}

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
    pub fn builder(owner: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, hashes: Vec<Cow<'a, str>>) -> SetProtectedAudienceKAnonymityParamsBuilder<'a> {
        SetProtectedAudienceKAnonymityParamsBuilder {
            owner: owner.into(),
            name: name.into(),
            hashes: hashes,
        }
    }
    pub fn owner(&self) -> &str { self.owner.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn hashes(&self) -> &[Cow<'a, str>] { &self.hashes }
}


pub struct SetProtectedAudienceKAnonymityParamsBuilder<'a> {
    owner: Cow<'a, str>,
    name: Cow<'a, str>,
    hashes: Vec<Cow<'a, str>>,
}

impl<'a> SetProtectedAudienceKAnonymityParamsBuilder<'a> {
    pub fn build(self) -> SetProtectedAudienceKAnonymityParams<'a> {
        SetProtectedAudienceKAnonymityParams {
            owner: self.owner,
            name: self.name,
            hashes: self.hashes,
        }
    }
}

impl<'a> SetProtectedAudienceKAnonymityParams<'a> { pub const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity"; }

impl<'a> crate::CdpCommand<'a> for SetProtectedAudienceKAnonymityParams<'a> {
    const METHOD: &'static str = "Storage.setProtectedAudienceKAnonymity";
    type Response = crate::EmptyReturns;
}
