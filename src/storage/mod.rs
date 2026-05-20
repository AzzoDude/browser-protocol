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
    #[serde(rename = "storageType")]
    storage_type: StorageType,
    /// Storage usage (bytes).
    usage: f64,
}

impl UsageForType {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_type`: Name of storage type.
    /// * `usage`: Storage usage (bytes).
    pub fn builder(storage_type: impl Into<StorageType>, usage: f64) -> UsageForTypeBuilder {
        UsageForTypeBuilder {
            storage_type: storage_type.into(),
            usage: usage,
        }
    }
    /// Name of storage type.
    pub fn storage_type(&self) -> &StorageType { &self.storage_type }
    /// Storage usage (bytes).
    pub fn usage(&self) -> f64 { self.usage }
}


pub struct UsageForTypeBuilder {
    storage_type: StorageType,
    usage: f64,
}

impl UsageForTypeBuilder {
    pub fn build(self) -> UsageForType {
        UsageForType {
            storage_type: self.storage_type,
            usage: self.usage,
        }
    }
}

/// Pair of issuer origin and number of available (signed, but not used) Trust
/// Tokens from that issuer.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrustTokens<'a> {
    #[serde(rename = "issuerOrigin")]
    issuer_origin: Cow<'a, str>,
    count: f64,
}

impl<'a> TrustTokens<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `issuer_origin`: 
    /// * `count`: 
    pub fn builder(issuer_origin: impl Into<Cow<'a, str>>, count: f64) -> TrustTokensBuilder<'a> {
        TrustTokensBuilder {
            issuer_origin: issuer_origin.into(),
            count: count,
        }
    }
    pub fn issuer_origin(&self) -> &str { self.issuer_origin.as_ref() }
    pub fn count(&self) -> f64 { self.count }
}


pub struct TrustTokensBuilder<'a> {
    issuer_origin: Cow<'a, str>,
    count: f64,
}

impl<'a> TrustTokensBuilder<'a> {
    pub fn build(self) -> TrustTokens<'a> {
        TrustTokens {
            issuer_origin: self.issuer_origin,
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
    /// Creates a builder for this type with the required parameters:
    /// * `key`: 
    /// * `value`: 
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
    #[serde(rename = "creationTime")]
    creation_time: crate::network::TimeSinceEpoch,
    /// Number of key-value pairs stored in origin's shared storage.
    length: u64,
    /// Current amount of bits of entropy remaining in the navigation budget.
    #[serde(rename = "remainingBudget")]
    remaining_budget: f64,
    /// Total number of bytes stored as key-value pairs in origin's shared
    /// storage.
    #[serde(rename = "bytesUsed")]
    bytes_used: i64,
}

impl SharedStorageMetadata {
    /// Creates a builder for this type with the required parameters:
    /// * `creation_time`: Time when the origin's shared storage was last created.
    /// * `length`: Number of key-value pairs stored in origin's shared storage.
    /// * `remaining_budget`: Current amount of bits of entropy remaining in the navigation budget.
    /// * `bytes_used`: Total number of bytes stored as key-value pairs in origin's shared storage.
    pub fn builder(creation_time: crate::network::TimeSinceEpoch, length: u64, remaining_budget: f64, bytes_used: i64) -> SharedStorageMetadataBuilder {
        SharedStorageMetadataBuilder {
            creation_time: creation_time,
            length: length,
            remaining_budget: remaining_budget,
            bytes_used: bytes_used,
        }
    }
    /// Time when the origin's shared storage was last created.
    pub fn creation_time(&self) -> &crate::network::TimeSinceEpoch { &self.creation_time }
    /// Number of key-value pairs stored in origin's shared storage.
    pub fn length(&self) -> u64 { self.length }
    /// Current amount of bits of entropy remaining in the navigation budget.
    pub fn remaining_budget(&self) -> f64 { self.remaining_budget }
    /// Total number of bytes stored as key-value pairs in origin's shared
    /// storage.
    pub fn bytes_used(&self) -> i64 { self.bytes_used }
}


pub struct SharedStorageMetadataBuilder {
    creation_time: crate::network::TimeSinceEpoch,
    length: u64,
    remaining_budget: f64,
    bytes_used: i64,
}

impl SharedStorageMetadataBuilder {
    pub fn build(self) -> SharedStorageMetadata {
        SharedStorageMetadata {
            creation_time: self.creation_time,
            length: self.length,
            remaining_budget: self.remaining_budget,
            bytes_used: self.bytes_used,
        }
    }
}

/// Represents a dictionary object passed in as privateAggregationConfig to
/// run or selectURL.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStoragePrivateAggregationConfig<'a> {
    /// The chosen aggregation service deployment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "aggregationCoordinatorOrigin")]
    aggregation_coordinator_origin: Option<Cow<'a, str>>,
    /// The context ID provided.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contextId")]
    context_id: Option<Cow<'a, str>>,
    /// Configures the maximum size allowed for filtering IDs.
    #[serde(rename = "filteringIdMaxBytes")]
    filtering_id_max_bytes: u64,
    /// The limit on the number of contributions in the final report.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxContributions")]
    max_contributions: Option<i64>,
}

impl<'a> SharedStoragePrivateAggregationConfig<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `filtering_id_max_bytes`: Configures the maximum size allowed for filtering IDs.
    pub fn builder(filtering_id_max_bytes: u64) -> SharedStoragePrivateAggregationConfigBuilder<'a> {
        SharedStoragePrivateAggregationConfigBuilder {
            aggregation_coordinator_origin: None,
            context_id: None,
            filtering_id_max_bytes: filtering_id_max_bytes,
            max_contributions: None,
        }
    }
    /// The chosen aggregation service deployment.
    pub fn aggregation_coordinator_origin(&self) -> Option<&str> { self.aggregation_coordinator_origin.as_deref() }
    /// The context ID provided.
    pub fn context_id(&self) -> Option<&str> { self.context_id.as_deref() }
    /// Configures the maximum size allowed for filtering IDs.
    pub fn filtering_id_max_bytes(&self) -> u64 { self.filtering_id_max_bytes }
    /// The limit on the number of contributions in the final report.
    pub fn max_contributions(&self) -> Option<i64> { self.max_contributions }
}


pub struct SharedStoragePrivateAggregationConfigBuilder<'a> {
    aggregation_coordinator_origin: Option<Cow<'a, str>>,
    context_id: Option<Cow<'a, str>>,
    filtering_id_max_bytes: u64,
    max_contributions: Option<i64>,
}

impl<'a> SharedStoragePrivateAggregationConfigBuilder<'a> {
    /// The chosen aggregation service deployment.
    pub fn aggregation_coordinator_origin(mut self, aggregation_coordinator_origin: impl Into<Cow<'a, str>>) -> Self { self.aggregation_coordinator_origin = Some(aggregation_coordinator_origin.into()); self }
    /// The context ID provided.
    pub fn context_id(mut self, context_id: impl Into<Cow<'a, str>>) -> Self { self.context_id = Some(context_id.into()); self }
    /// The limit on the number of contributions in the final report.
    pub fn max_contributions(mut self, max_contributions: i64) -> Self { self.max_contributions = Some(max_contributions); self }
    pub fn build(self) -> SharedStoragePrivateAggregationConfig<'a> {
        SharedStoragePrivateAggregationConfig {
            aggregation_coordinator_origin: self.aggregation_coordinator_origin,
            context_id: self.context_id,
            filtering_id_max_bytes: self.filtering_id_max_bytes,
            max_contributions: self.max_contributions,
        }
    }
}

/// Pair of reporting metadata details for a candidate URL for 'selectURL()'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SharedStorageReportingMetadata<'a> {
    #[serde(rename = "eventType")]
    event_type: Cow<'a, str>,
    #[serde(rename = "reportingUrl")]
    reporting_url: Cow<'a, str>,
}

impl<'a> SharedStorageReportingMetadata<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `event_type`: 
    /// * `reporting_url`: 
    pub fn builder(event_type: impl Into<Cow<'a, str>>, reporting_url: impl Into<Cow<'a, str>>) -> SharedStorageReportingMetadataBuilder<'a> {
        SharedStorageReportingMetadataBuilder {
            event_type: event_type.into(),
            reporting_url: reporting_url.into(),
        }
    }
    pub fn event_type(&self) -> &str { self.event_type.as_ref() }
    pub fn reporting_url(&self) -> &str { self.reporting_url.as_ref() }
}


pub struct SharedStorageReportingMetadataBuilder<'a> {
    event_type: Cow<'a, str>,
    reporting_url: Cow<'a, str>,
}

impl<'a> SharedStorageReportingMetadataBuilder<'a> {
    pub fn build(self) -> SharedStorageReportingMetadata<'a> {
        SharedStorageReportingMetadata {
            event_type: self.event_type,
            reporting_url: self.reporting_url,
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
    #[serde(rename = "reportingMetadata")]
    reporting_metadata: Vec<SharedStorageReportingMetadata<'a>>,
}

impl<'a> SharedStorageUrlWithMetadata<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `url`: Spec of candidate URL.
    /// * `reporting_metadata`: Any associated reporting metadata.
    pub fn builder(url: impl Into<Cow<'a, str>>, reporting_metadata: Vec<SharedStorageReportingMetadata<'a>>) -> SharedStorageUrlWithMetadataBuilder<'a> {
        SharedStorageUrlWithMetadataBuilder {
            url: url.into(),
            reporting_metadata: reporting_metadata,
        }
    }
    /// Spec of candidate URL.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Any associated reporting metadata.
    pub fn reporting_metadata(&self) -> &[SharedStorageReportingMetadata<'a>] { &self.reporting_metadata }
}


pub struct SharedStorageUrlWithMetadataBuilder<'a> {
    url: Cow<'a, str>,
    reporting_metadata: Vec<SharedStorageReportingMetadata<'a>>,
}

impl<'a> SharedStorageUrlWithMetadataBuilder<'a> {
    pub fn build(self) -> SharedStorageUrlWithMetadata<'a> {
        SharedStorageUrlWithMetadata {
            url: self.url,
            reporting_metadata: self.reporting_metadata,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptSourceUrl")]
    script_source_url: Option<Cow<'a, str>>,
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataOrigin")]
    data_origin: Option<Cow<'a, str>>,
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationName")]
    operation_name: Option<Cow<'a, str>>,
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "operationId")]
    operation_id: Option<Cow<'a, str>>,
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keepAlive")]
    keep_alive: Option<bool>,
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "privateAggregationConfig")]
    private_aggregation_config: Option<SharedStoragePrivateAggregationConfig<'a>>,
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serializedData")]
    serialized_data: Option<Cow<'a, str>>,
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlsWithMetadata")]
    urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata<'a>>>,
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urnUuid")]
    urn_uuid: Option<Cow<'a, str>>,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoreIfPresent")]
    ignore_if_present: Option<bool>,
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workletOrdinal")]
    worklet_ordinal: Option<i64>,
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.
    #[serde(skip_serializing_if = "Option::is_none", rename = "workletTargetId")]
    worklet_target_id: Option<crate::target::TargetID<'a>>,
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.
    #[serde(skip_serializing_if = "Option::is_none", rename = "withLock")]
    with_lock: Option<Cow<'a, str>>,
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.
    #[serde(skip_serializing_if = "Option::is_none", rename = "batchUpdateId")]
    batch_update_id: Option<Cow<'a, str>>,
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.
    #[serde(skip_serializing_if = "Option::is_none", rename = "batchSize")]
    batch_size: Option<u64>,
}

impl<'a> SharedStorageAccessParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> SharedStorageAccessParamsBuilder<'a> {
        SharedStorageAccessParamsBuilder {
            script_source_url: None,
            data_origin: None,
            operation_name: None,
            operation_id: None,
            keep_alive: None,
            private_aggregation_config: None,
            serialized_data: None,
            urls_with_metadata: None,
            urn_uuid: None,
            key: None,
            value: None,
            ignore_if_present: None,
            worklet_ordinal: None,
            worklet_target_id: None,
            with_lock: None,
            batch_update_id: None,
            batch_size: None,
        }
    }
    /// Spec of the module script URL.
    /// Present only for SharedStorageAccessMethods: addModule and
    /// createWorklet.
    pub fn script_source_url(&self) -> Option<&str> { self.script_source_url.as_deref() }
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.
    pub fn data_origin(&self) -> Option<&str> { self.data_origin.as_deref() }
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operation_name(&self) -> Option<&str> { self.operation_name.as_deref() }
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operation_id(&self) -> Option<&str> { self.operation_id.as_deref() }
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn keep_alive(&self) -> Option<bool> { self.keep_alive }
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn private_aggregation_config(&self) -> Option<&SharedStoragePrivateAggregationConfig<'a>> { self.private_aggregation_config.as_ref() }
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.
    pub fn serialized_data(&self) -> Option<&str> { self.serialized_data.as_deref() }
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urls_with_metadata(&self) -> Option<&[SharedStorageUrlWithMetadata<'a>]> { self.urls_with_metadata.as_deref() }
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urn_uuid(&self) -> Option<&str> { self.urn_uuid.as_deref() }
    /// Key for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set, append, delete, and
    /// get.
    pub fn key(&self) -> Option<&str> { self.key.as_deref() }
    /// Value for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set and append.
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }
    /// Whether or not to set an entry for a key if that key is already present.
    /// Present only for SharedStorageAccessMethod: set.
    pub fn ignore_if_present(&self) -> Option<bool> { self.ignore_if_present }
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.
    pub fn worklet_ordinal(&self) -> Option<i64> { self.worklet_ordinal }
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.
    pub fn worklet_target_id(&self) -> Option<&crate::target::TargetID<'a>> { self.worklet_target_id.as_ref() }
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.
    pub fn with_lock(&self) -> Option<&str> { self.with_lock.as_deref() }
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.
    pub fn batch_update_id(&self) -> Option<&str> { self.batch_update_id.as_deref() }
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.
    pub fn batch_size(&self) -> Option<u64> { self.batch_size }
}

#[derive(Default)]
pub struct SharedStorageAccessParamsBuilder<'a> {
    script_source_url: Option<Cow<'a, str>>,
    data_origin: Option<Cow<'a, str>>,
    operation_name: Option<Cow<'a, str>>,
    operation_id: Option<Cow<'a, str>>,
    keep_alive: Option<bool>,
    private_aggregation_config: Option<SharedStoragePrivateAggregationConfig<'a>>,
    serialized_data: Option<Cow<'a, str>>,
    urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata<'a>>>,
    urn_uuid: Option<Cow<'a, str>>,
    key: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    ignore_if_present: Option<bool>,
    worklet_ordinal: Option<i64>,
    worklet_target_id: Option<crate::target::TargetID<'a>>,
    with_lock: Option<Cow<'a, str>>,
    batch_update_id: Option<Cow<'a, str>>,
    batch_size: Option<u64>,
}

impl<'a> SharedStorageAccessParamsBuilder<'a> {
    /// Spec of the module script URL.
    /// Present only for SharedStorageAccessMethods: addModule and
    /// createWorklet.
    pub fn script_source_url(mut self, script_source_url: impl Into<Cow<'a, str>>) -> Self { self.script_source_url = Some(script_source_url.into()); self }
    /// String denoting "context-origin", "script-origin", or a custom
    /// origin to be used as the worklet's data origin.
    /// Present only for SharedStorageAccessMethod: createWorklet.
    pub fn data_origin(mut self, data_origin: impl Into<Cow<'a, str>>) -> Self { self.data_origin = Some(data_origin.into()); self }
    /// Name of the registered operation to be run.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operation_name(mut self, operation_name: impl Into<Cow<'a, str>>) -> Self { self.operation_name = Some(operation_name.into()); self }
    /// ID of the operation call.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn operation_id(mut self, operation_id: impl Into<Cow<'a, str>>) -> Self { self.operation_id = Some(operation_id.into()); self }
    /// Whether or not to keep the worket alive for future run or selectURL
    /// calls.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn keep_alive(mut self, keep_alive: bool) -> Self { self.keep_alive = Some(keep_alive); self }
    /// Configures the private aggregation options.
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    pub fn private_aggregation_config(mut self, private_aggregation_config: SharedStoragePrivateAggregationConfig<'a>) -> Self { self.private_aggregation_config = Some(private_aggregation_config); self }
    /// The operation's serialized data in bytes (converted to a string).
    /// Present only for SharedStorageAccessMethods: run and selectURL.
    /// TODO(crbug.com/401011862): Consider updating this parameter to binary.
    pub fn serialized_data(mut self, serialized_data: impl Into<Cow<'a, str>>) -> Self { self.serialized_data = Some(serialized_data.into()); self }
    /// Array of candidate URLs' specs, along with any associated metadata.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urls_with_metadata(mut self, urls_with_metadata: Vec<SharedStorageUrlWithMetadata<'a>>) -> Self { self.urls_with_metadata = Some(urls_with_metadata); self }
    /// Spec of the URN:UUID generated for a selectURL call.
    /// Present only for SharedStorageAccessMethod: selectURL.
    pub fn urn_uuid(mut self, urn_uuid: impl Into<Cow<'a, str>>) -> Self { self.urn_uuid = Some(urn_uuid.into()); self }
    /// Key for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set, append, delete, and
    /// get.
    pub fn key(mut self, key: impl Into<Cow<'a, str>>) -> Self { self.key = Some(key.into()); self }
    /// Value for a specific entry in an origin's shared storage.
    /// Present only for SharedStorageAccessMethods: set and append.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Whether or not to set an entry for a key if that key is already present.
    /// Present only for SharedStorageAccessMethod: set.
    pub fn ignore_if_present(mut self, ignore_if_present: bool) -> Self { self.ignore_if_present = Some(ignore_if_present); self }
    /// A number denoting the (0-based) order of the worklet's
    /// creation relative to all other shared storage worklets created by
    /// documents using the current storage partition.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet.
    pub fn worklet_ordinal(mut self, worklet_ordinal: i64) -> Self { self.worklet_ordinal = Some(worklet_ordinal); self }
    /// Hex representation of the DevTools token used as the TargetID for the
    /// associated shared storage worklet.
    /// Present only for SharedStorageAccessMethods: addModule, createWorklet,
    /// run, selectURL, and any other SharedStorageAccessMethod when the
    /// SharedStorageAccessScope is sharedStorageWorklet.
    pub fn worklet_target_id(mut self, worklet_target_id: crate::target::TargetID<'a>) -> Self { self.worklet_target_id = Some(worklet_target_id); self }
    /// Name of the lock to be acquired, if present.
    /// Optionally present only for SharedStorageAccessMethods: batchUpdate,
    /// set, append, delete, and clear.
    pub fn with_lock(mut self, with_lock: impl Into<Cow<'a, str>>) -> Self { self.with_lock = Some(with_lock.into()); self }
    /// If the method has been called as part of a batchUpdate, then this
    /// number identifies the batch to which it belongs.
    /// Optionally present only for SharedStorageAccessMethods:
    /// batchUpdate (required), set, append, delete, and clear.
    pub fn batch_update_id(mut self, batch_update_id: impl Into<Cow<'a, str>>) -> Self { self.batch_update_id = Some(batch_update_id.into()); self }
    /// Number of modifier methods sent in batch.
    /// Present only for SharedStorageAccessMethod: batchUpdate.
    pub fn batch_size(mut self, batch_size: u64) -> Self { self.batch_size = Some(batch_size); self }
    pub fn build(self) -> SharedStorageAccessParams<'a> {
        SharedStorageAccessParams {
            script_source_url: self.script_source_url,
            data_origin: self.data_origin,
            operation_name: self.operation_name,
            operation_id: self.operation_id,
            keep_alive: self.keep_alive,
            private_aggregation_config: self.private_aggregation_config,
            serialized_data: self.serialized_data,
            urls_with_metadata: self.urls_with_metadata,
            urn_uuid: self.urn_uuid,
            key: self.key,
            value: self.value,
            ignore_if_present: self.ignore_if_present,
            worklet_ordinal: self.worklet_ordinal,
            worklet_target_id: self.worklet_target_id,
            with_lock: self.with_lock,
            batch_update_id: self.batch_update_id,
            batch_size: self.batch_size,
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
    #[serde(rename = "storageKey")]
    storage_key: SerializedStorageKey<'a>,
    /// If not specified, it is the default bucket of the storageKey.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
}

impl<'a> StorageBucket<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: 
    pub fn builder(storage_key: impl Into<SerializedStorageKey<'a>>) -> StorageBucketBuilder<'a> {
        StorageBucketBuilder {
            storage_key: storage_key.into(),
            name: None,
        }
    }
    pub fn storage_key(&self) -> &SerializedStorageKey<'a> { &self.storage_key }
    /// If not specified, it is the default bucket of the storageKey.
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
}


pub struct StorageBucketBuilder<'a> {
    storage_key: SerializedStorageKey<'a>,
    name: Option<Cow<'a, str>>,
}

impl<'a> StorageBucketBuilder<'a> {
    /// If not specified, it is the default bucket of the storageKey.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn build(self) -> StorageBucket<'a> {
        StorageBucket {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `bucket`: 
    /// * `id`: 
    /// * `expiration`: 
    /// * `quota`: Storage quota (bytes).
    /// * `persistent`: 
    /// * `durability`: 
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
    /// Storage quota (bytes).
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
    #[serde(rename = "primarySites")]
    primary_sites: Vec<Cow<'a, str>>,
    /// The associated sites of this set, along with the ccTLDs if there is any.
    #[serde(rename = "associatedSites")]
    associated_sites: Vec<Cow<'a, str>>,
    /// The service sites of this set, along with the ccTLDs if there is any.
    #[serde(rename = "serviceSites")]
    service_sites: Vec<Cow<'a, str>>,
}

impl<'a> RelatedWebsiteSet<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `primary_sites`: The primary site of this set, along with the ccTLDs if there is any.
    /// * `associated_sites`: The associated sites of this set, along with the ccTLDs if there is any.
    /// * `service_sites`: The service sites of this set, along with the ccTLDs if there is any.
    pub fn builder(primary_sites: Vec<Cow<'a, str>>, associated_sites: Vec<Cow<'a, str>>, service_sites: Vec<Cow<'a, str>>) -> RelatedWebsiteSetBuilder<'a> {
        RelatedWebsiteSetBuilder {
            primary_sites: primary_sites,
            associated_sites: associated_sites,
            service_sites: service_sites,
        }
    }
    /// The primary site of this set, along with the ccTLDs if there is any.
    pub fn primary_sites(&self) -> &[Cow<'a, str>] { &self.primary_sites }
    /// The associated sites of this set, along with the ccTLDs if there is any.
    pub fn associated_sites(&self) -> &[Cow<'a, str>] { &self.associated_sites }
    /// The service sites of this set, along with the ccTLDs if there is any.
    pub fn service_sites(&self) -> &[Cow<'a, str>] { &self.service_sites }
}


pub struct RelatedWebsiteSetBuilder<'a> {
    primary_sites: Vec<Cow<'a, str>>,
    associated_sites: Vec<Cow<'a, str>>,
    service_sites: Vec<Cow<'a, str>>,
}

impl<'a> RelatedWebsiteSetBuilder<'a> {
    pub fn build(self) -> RelatedWebsiteSet<'a> {
        RelatedWebsiteSet {
            primary_sites: self.primary_sites,
            associated_sites: self.associated_sites,
            service_sites: self.service_sites,
        }
    }
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameParams<'a> {
    #[serde(rename = "frameId")]
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> GetStorageKeyForFrameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `frame_id`: 
    pub fn builder(frame_id: crate::page::FrameId<'a>) -> GetStorageKeyForFrameParamsBuilder<'a> {
        GetStorageKeyForFrameParamsBuilder {
            frame_id: frame_id,
        }
    }
    pub fn frame_id(&self) -> &crate::page::FrameId<'a> { &self.frame_id }
}


pub struct GetStorageKeyForFrameParamsBuilder<'a> {
    frame_id: crate::page::FrameId<'a>,
}

impl<'a> GetStorageKeyForFrameParamsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyForFrameParams<'a> {
        GetStorageKeyForFrameParams {
            frame_id: self.frame_id,
        }
    }
}

/// Returns a storage key given a frame id.
/// Deprecated. Please use Storage.getStorageKey instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyForFrameReturns<'a> {
    #[serde(rename = "storageKey")]
    storage_key: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyForFrameReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: 
    pub fn builder(storage_key: impl Into<SerializedStorageKey<'a>>) -> GetStorageKeyForFrameReturnsBuilder<'a> {
        GetStorageKeyForFrameReturnsBuilder {
            storage_key: storage_key.into(),
        }
    }
    pub fn storage_key(&self) -> &SerializedStorageKey<'a> { &self.storage_key }
}


pub struct GetStorageKeyForFrameReturnsBuilder<'a> {
    storage_key: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyForFrameReturnsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyForFrameReturns<'a> {
        GetStorageKeyForFrameReturns {
            storage_key: self.storage_key,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "frameId")]
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetStorageKeyParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetStorageKeyParamsBuilder<'a> {
        GetStorageKeyParamsBuilder {
            frame_id: None,
        }
    }
    pub fn frame_id(&self) -> Option<&crate::page::FrameId<'a>> { self.frame_id.as_ref() }
}

#[derive(Default)]
pub struct GetStorageKeyParamsBuilder<'a> {
    frame_id: Option<crate::page::FrameId<'a>>,
}

impl<'a> GetStorageKeyParamsBuilder<'a> {
    pub fn frame_id(mut self, frame_id: crate::page::FrameId<'a>) -> Self { self.frame_id = Some(frame_id); self }
    pub fn build(self) -> GetStorageKeyParams<'a> {
        GetStorageKeyParams {
            frame_id: self.frame_id,
        }
    }
}

/// Returns storage key for the given frame. If no frame ID is provided,
/// the storage key of the target executing this command is returned.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStorageKeyReturns<'a> {
    #[serde(rename = "storageKey")]
    storage_key: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: 
    pub fn builder(storage_key: impl Into<SerializedStorageKey<'a>>) -> GetStorageKeyReturnsBuilder<'a> {
        GetStorageKeyReturnsBuilder {
            storage_key: storage_key.into(),
        }
    }
    pub fn storage_key(&self) -> &SerializedStorageKey<'a> { &self.storage_key }
}


pub struct GetStorageKeyReturnsBuilder<'a> {
    storage_key: SerializedStorageKey<'a>,
}

impl<'a> GetStorageKeyReturnsBuilder<'a> {
    pub fn build(self) -> GetStorageKeyReturns<'a> {
        GetStorageKeyReturns {
            storage_key: self.storage_key,
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
    #[serde(rename = "storageTypes")]
    storage_types: Cow<'a, str>,
}

impl<'a> ClearDataForOriginParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    /// * `storage_types`: Comma separated list of StorageType to clear.
    pub fn builder(origin: impl Into<Cow<'a, str>>, storage_types: impl Into<Cow<'a, str>>) -> ClearDataForOriginParamsBuilder<'a> {
        ClearDataForOriginParamsBuilder {
            origin: origin.into(),
            storage_types: storage_types.into(),
        }
    }
    /// Security origin.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// Comma separated list of StorageType to clear.
    pub fn storage_types(&self) -> &str { self.storage_types.as_ref() }
}


pub struct ClearDataForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
    storage_types: Cow<'a, str>,
}

impl<'a> ClearDataForOriginParamsBuilder<'a> {
    pub fn build(self) -> ClearDataForOriginParams<'a> {
        ClearDataForOriginParams {
            origin: self.origin,
            storage_types: self.storage_types,
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
    /// Comma separated list of StorageType to clear.
    #[serde(rename = "storageTypes")]
    storage_types: Cow<'a, str>,
}

impl<'a> ClearDataForStorageKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key.
    /// * `storage_types`: Comma separated list of StorageType to clear.
    pub fn builder(storage_key: impl Into<Cow<'a, str>>, storage_types: impl Into<Cow<'a, str>>) -> ClearDataForStorageKeyParamsBuilder<'a> {
        ClearDataForStorageKeyParamsBuilder {
            storage_key: storage_key.into(),
            storage_types: storage_types.into(),
        }
    }
    /// Storage key.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
    /// Comma separated list of StorageType to clear.
    pub fn storage_types(&self) -> &str { self.storage_types.as_ref() }
}


pub struct ClearDataForStorageKeyParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
    storage_types: Cow<'a, str>,
}

impl<'a> ClearDataForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> ClearDataForStorageKeyParams<'a> {
        ClearDataForStorageKeyParams {
            storage_key: self.storage_key,
            storage_types: self.storage_types,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetCookiesParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetCookiesParamsBuilder<'a> {
        GetCookiesParamsBuilder {
            browser_context_id: None,
        }
    }
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}

#[derive(Default)]
pub struct GetCookiesParamsBuilder<'a> {
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> GetCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(mut self, browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.browser_context_id = Some(browser_context_id); self }
    pub fn build(self) -> GetCookiesParams<'a> {
        GetCookiesParams {
            browser_context_id: self.browser_context_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `cookies`: Array of cookie objects.
    pub fn builder(cookies: Vec<crate::network::Cookie<'a>>) -> GetCookiesReturnsBuilder<'a> {
        GetCookiesReturnsBuilder {
            cookies: cookies,
        }
    }
    /// Array of cookie objects.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> SetCookiesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `cookies`: Cookies to be set.
    pub fn builder(cookies: Vec<crate::network::CookieParam<'a>>) -> SetCookiesParamsBuilder<'a> {
        SetCookiesParamsBuilder {
            cookies: cookies,
            browser_context_id: None,
        }
    }
    /// Cookies to be set.
    pub fn cookies(&self) -> &[crate::network::CookieParam<'a>] { &self.cookies }
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}


pub struct SetCookiesParamsBuilder<'a> {
    cookies: Vec<crate::network::CookieParam<'a>>,
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> SetCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(mut self, browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.browser_context_id = Some(browser_context_id); self }
    pub fn build(self) -> SetCookiesParams<'a> {
        SetCookiesParams {
            cookies: self.cookies,
            browser_context_id: self.browser_context_id,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "browserContextId")]
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> ClearCookiesParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> ClearCookiesParamsBuilder<'a> {
        ClearCookiesParamsBuilder {
            browser_context_id: None,
        }
    }
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(&self) -> Option<&crate::browser::BrowserContextID<'a>> { self.browser_context_id.as_ref() }
}

#[derive(Default)]
pub struct ClearCookiesParamsBuilder<'a> {
    browser_context_id: Option<crate::browser::BrowserContextID<'a>>,
}

impl<'a> ClearCookiesParamsBuilder<'a> {
    /// Browser context to use when called on the browser endpoint.
    pub fn browser_context_id(mut self, browser_context_id: crate::browser::BrowserContextID<'a>) -> Self { self.browser_context_id = Some(browser_context_id); self }
    pub fn build(self) -> ClearCookiesParams<'a> {
        ClearCookiesParams {
            browser_context_id: self.browser_context_id,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> GetUsageAndQuotaParamsBuilder<'a> {
        GetUsageAndQuotaParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Security origin.
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
    #[serde(rename = "overrideActive")]
    override_active: bool,
    /// Storage usage per type (bytes).
    #[serde(rename = "usageBreakdown")]
    usage_breakdown: Vec<UsageForType>,
}

impl GetUsageAndQuotaReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `usage`: Storage usage (bytes).
    /// * `quota`: Storage quota (bytes).
    /// * `override_active`: Whether or not the origin has an active storage quota override
    /// * `usage_breakdown`: Storage usage per type (bytes).
    pub fn builder(usage: f64, quota: f64, override_active: bool, usage_breakdown: Vec<UsageForType>) -> GetUsageAndQuotaReturnsBuilder {
        GetUsageAndQuotaReturnsBuilder {
            usage: usage,
            quota: quota,
            override_active: override_active,
            usage_breakdown: usage_breakdown,
        }
    }
    /// Storage usage (bytes).
    pub fn usage(&self) -> f64 { self.usage }
    /// Storage quota (bytes).
    pub fn quota(&self) -> f64 { self.quota }
    /// Whether or not the origin has an active storage quota override
    pub fn override_active(&self) -> bool { self.override_active }
    /// Storage usage per type (bytes).
    pub fn usage_breakdown(&self) -> &[UsageForType] { &self.usage_breakdown }
}


pub struct GetUsageAndQuotaReturnsBuilder {
    usage: f64,
    quota: f64,
    override_active: bool,
    usage_breakdown: Vec<UsageForType>,
}

impl GetUsageAndQuotaReturnsBuilder {
    pub fn build(self) -> GetUsageAndQuotaReturns {
        GetUsageAndQuotaReturns {
            usage: self.usage,
            quota: self.quota,
            override_active: self.override_active,
            usage_breakdown: self.usage_breakdown,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "quotaSize")]
    quota_size: Option<f64>,
}

impl<'a> OverrideQuotaForOriginParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> OverrideQuotaForOriginParamsBuilder<'a> {
        OverrideQuotaForOriginParamsBuilder {
            origin: origin.into(),
            quota_size: None,
        }
    }
    /// Security origin.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).
    pub fn quota_size(&self) -> Option<f64> { self.quota_size }
}


pub struct OverrideQuotaForOriginParamsBuilder<'a> {
    origin: Cow<'a, str>,
    quota_size: Option<f64>,
}

impl<'a> OverrideQuotaForOriginParamsBuilder<'a> {
    /// The quota size (in bytes) to override the original quota with.
    /// If this is called multiple times, the overridden quota will be equal to
    /// the quotaSize provided in the final call. If this is called without
    /// specifying a quotaSize, the quota will be reset to the default value for
    /// the specified origin. If this is called multiple times with different
    /// origins, the override will be maintained for each origin until it is
    /// disabled (called without a quotaSize).
    pub fn quota_size(mut self, quota_size: f64) -> Self { self.quota_size = Some(quota_size); self }
    pub fn build(self) -> OverrideQuotaForOriginParams<'a> {
        OverrideQuotaForOriginParams {
            origin: self.origin,
            quota_size: self.quota_size,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> TrackCacheStorageForOriginParamsBuilder<'a> {
        TrackCacheStorageForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Security origin.
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForStorageKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key.
    pub fn builder(storage_key: impl Into<Cow<'a, str>>) -> TrackCacheStorageForStorageKeyParamsBuilder<'a> {
        TrackCacheStorageForStorageKeyParamsBuilder {
            storage_key: storage_key.into(),
        }
    }
    /// Storage key.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
}


pub struct TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
}

impl<'a> TrackCacheStorageForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> TrackCacheStorageForStorageKeyParams<'a> {
        TrackCacheStorageForStorageKeyParams {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> TrackIndexedDBForOriginParamsBuilder<'a> {
        TrackIndexedDBForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Security origin.
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForStorageKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key.
    pub fn builder(storage_key: impl Into<Cow<'a, str>>) -> TrackIndexedDBForStorageKeyParamsBuilder<'a> {
        TrackIndexedDBForStorageKeyParamsBuilder {
            storage_key: storage_key.into(),
        }
    }
    /// Storage key.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
}


pub struct TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
}

impl<'a> TrackIndexedDBForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> TrackIndexedDBForStorageKeyParams<'a> {
        TrackIndexedDBForStorageKeyParams {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> UntrackCacheStorageForOriginParamsBuilder<'a> {
        UntrackCacheStorageForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Security origin.
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForStorageKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key.
    pub fn builder(storage_key: impl Into<Cow<'a, str>>) -> UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
        UntrackCacheStorageForStorageKeyParamsBuilder {
            storage_key: storage_key.into(),
        }
    }
    /// Storage key.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
}


pub struct UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
}

impl<'a> UntrackCacheStorageForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> UntrackCacheStorageForStorageKeyParams<'a> {
        UntrackCacheStorageForStorageKeyParams {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `origin`: Security origin.
    pub fn builder(origin: impl Into<Cow<'a, str>>) -> UntrackIndexedDBForOriginParamsBuilder<'a> {
        UntrackIndexedDBForOriginParamsBuilder {
            origin: origin.into(),
        }
    }
    /// Security origin.
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForStorageKeyParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key.
    pub fn builder(storage_key: impl Into<Cow<'a, str>>) -> UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
        UntrackIndexedDBForStorageKeyParamsBuilder {
            storage_key: storage_key.into(),
        }
    }
    /// Storage key.
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
}


pub struct UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
}

impl<'a> UntrackIndexedDBForStorageKeyParamsBuilder<'a> {
    pub fn build(self) -> UntrackIndexedDBForStorageKeyParams<'a> {
        UntrackIndexedDBForStorageKeyParams {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `tokens`: 
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
    #[serde(rename = "issuerOrigin")]
    issuer_origin: Cow<'a, str>,
}

impl<'a> ClearTrustTokensParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `issuer_origin`: 
    pub fn builder(issuer_origin: impl Into<Cow<'a, str>>) -> ClearTrustTokensParamsBuilder<'a> {
        ClearTrustTokensParamsBuilder {
            issuer_origin: issuer_origin.into(),
        }
    }
    pub fn issuer_origin(&self) -> &str { self.issuer_origin.as_ref() }
}


pub struct ClearTrustTokensParamsBuilder<'a> {
    issuer_origin: Cow<'a, str>,
}

impl<'a> ClearTrustTokensParamsBuilder<'a> {
    pub fn build(self) -> ClearTrustTokensParams<'a> {
        ClearTrustTokensParams {
            issuer_origin: self.issuer_origin,
        }
    }
}

/// Removes all Trust Tokens issued by the provided issuerOrigin.
/// Leaves other stored data, including the issuer's Redemption Records, intact.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearTrustTokensReturns {
    /// True if any tokens were deleted, false otherwise.
    #[serde(rename = "didDeleteTokens")]
    did_delete_tokens: bool,
}

impl ClearTrustTokensReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `did_delete_tokens`: True if any tokens were deleted, false otherwise.
    pub fn builder(did_delete_tokens: bool) -> ClearTrustTokensReturnsBuilder {
        ClearTrustTokensReturnsBuilder {
            did_delete_tokens: did_delete_tokens,
        }
    }
    /// True if any tokens were deleted, false otherwise.
    pub fn did_delete_tokens(&self) -> bool { self.did_delete_tokens }
}


pub struct ClearTrustTokensReturnsBuilder {
    did_delete_tokens: bool,
}

impl ClearTrustTokensReturnsBuilder {
    pub fn build(self) -> ClearTrustTokensReturns {
        ClearTrustTokensReturns {
            did_delete_tokens: self.did_delete_tokens,
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> GetInterestGroupDetailsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    /// * `name`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>) -> GetInterestGroupDetailsParamsBuilder<'a> {
        GetInterestGroupDetailsParamsBuilder {
            owner_origin: owner_origin.into(),
            name: name.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct GetInterestGroupDetailsParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
    name: Cow<'a, str>,
}

impl<'a> GetInterestGroupDetailsParamsBuilder<'a> {
    pub fn build(self) -> GetInterestGroupDetailsParams<'a> {
        GetInterestGroupDetailsParams {
            owner_origin: self.owner_origin,
            name: self.name,
        }
    }
}

/// Gets details for a named interest group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestGroupDetailsReturns {
    /// This largely corresponds to:
    /// <https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup>
    /// but has absolute expirationTime instead of relative lifetimeMs and
    /// also adds joiningOrigin.
    details: serde_json::Map<String, JsonValue>,
}

impl GetInterestGroupDetailsReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `details`: This largely corresponds to: <https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup> but has absolute expirationTime instead of relative lifetimeMs and also adds joiningOrigin.
    pub fn builder(details: serde_json::Map<String, JsonValue>) -> GetInterestGroupDetailsReturnsBuilder {
        GetInterestGroupDetailsReturnsBuilder {
            details: details,
        }
    }
    /// This largely corresponds to:
    /// <https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup>
    /// but has absolute expirationTime instead of relative lifetimeMs and
    /// also adds joiningOrigin.
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: 
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
}

impl<'a> GetSharedStorageMetadataParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>) -> GetSharedStorageMetadataParamsBuilder<'a> {
        GetSharedStorageMetadataParamsBuilder {
            owner_origin: owner_origin.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
}


pub struct GetSharedStorageMetadataParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
}

impl<'a> GetSharedStorageMetadataParamsBuilder<'a> {
    pub fn build(self) -> GetSharedStorageMetadataParams<'a> {
        GetSharedStorageMetadataParams {
            owner_origin: self.owner_origin,
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
    /// Creates a builder for this type with the required parameters:
    /// * `metadata`: 
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
}

impl<'a> GetSharedStorageEntriesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>) -> GetSharedStorageEntriesParamsBuilder<'a> {
        GetSharedStorageEntriesParamsBuilder {
            owner_origin: owner_origin.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
}


pub struct GetSharedStorageEntriesParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
}

impl<'a> GetSharedStorageEntriesParamsBuilder<'a> {
    pub fn build(self) -> GetSharedStorageEntriesParams<'a> {
        GetSharedStorageEntriesParams {
            owner_origin: self.owner_origin,
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
    /// Creates a builder for this type with the required parameters:
    /// * `entries`: 
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ignoreIfPresent")]
    ignore_if_present: Option<bool>,
}

impl<'a> SetSharedStorageEntryParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    /// * `key`: 
    /// * `value`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> SetSharedStorageEntryParamsBuilder<'a> {
        SetSharedStorageEntryParamsBuilder {
            owner_origin: owner_origin.into(),
            key: key.into(),
            value: value.into(),
            ignore_if_present: None,
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
    pub fn value(&self) -> &str { self.value.as_ref() }
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    pub fn ignore_if_present(&self) -> Option<bool> { self.ignore_if_present }
}


pub struct SetSharedStorageEntryParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
    key: Cow<'a, str>,
    value: Cow<'a, str>,
    ignore_if_present: Option<bool>,
}

impl<'a> SetSharedStorageEntryParamsBuilder<'a> {
    /// If 'ignoreIfPresent' is included and true, then only sets the entry if
    /// 'key' doesn't already exist.
    pub fn ignore_if_present(mut self, ignore_if_present: bool) -> Self { self.ignore_if_present = Some(ignore_if_present); self }
    pub fn build(self) -> SetSharedStorageEntryParams<'a> {
        SetSharedStorageEntryParams {
            owner_origin: self.owner_origin,
            key: self.key,
            value: self.value,
            ignore_if_present: self.ignore_if_present,
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
    key: Cow<'a, str>,
}

impl<'a> DeleteSharedStorageEntryParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    /// * `key`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>, key: impl Into<Cow<'a, str>>) -> DeleteSharedStorageEntryParamsBuilder<'a> {
        DeleteSharedStorageEntryParamsBuilder {
            owner_origin: owner_origin.into(),
            key: key.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
    pub fn key(&self) -> &str { self.key.as_ref() }
}


pub struct DeleteSharedStorageEntryParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
    key: Cow<'a, str>,
}

impl<'a> DeleteSharedStorageEntryParamsBuilder<'a> {
    pub fn build(self) -> DeleteSharedStorageEntryParams<'a> {
        DeleteSharedStorageEntryParams {
            owner_origin: self.owner_origin,
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
}

impl<'a> ClearSharedStorageEntriesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>) -> ClearSharedStorageEntriesParamsBuilder<'a> {
        ClearSharedStorageEntriesParamsBuilder {
            owner_origin: owner_origin.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
}


pub struct ClearSharedStorageEntriesParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
}

impl<'a> ClearSharedStorageEntriesParamsBuilder<'a> {
    pub fn build(self) -> ClearSharedStorageEntriesParams<'a> {
        ClearSharedStorageEntriesParams {
            owner_origin: self.owner_origin,
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
    #[serde(rename = "ownerOrigin")]
    owner_origin: Cow<'a, str>,
}

impl<'a> ResetSharedStorageBudgetParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `owner_origin`: 
    pub fn builder(owner_origin: impl Into<Cow<'a, str>>) -> ResetSharedStorageBudgetParamsBuilder<'a> {
        ResetSharedStorageBudgetParamsBuilder {
            owner_origin: owner_origin.into(),
        }
    }
    pub fn owner_origin(&self) -> &str { self.owner_origin.as_ref() }
}


pub struct ResetSharedStorageBudgetParamsBuilder<'a> {
    owner_origin: Cow<'a, str>,
}

impl<'a> ResetSharedStorageBudgetParamsBuilder<'a> {
    pub fn build(self) -> ResetSharedStorageBudgetParams<'a> {
        ResetSharedStorageBudgetParams {
            owner_origin: self.owner_origin,
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
    /// Creates a builder for this type with the required parameters:
    /// * `enable`: 
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
    #[serde(rename = "storageKey")]
    storage_key: Cow<'a, str>,
    enable: bool,
}

impl<'a> SetStorageBucketTrackingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: 
    /// * `enable`: 
    pub fn builder(storage_key: impl Into<Cow<'a, str>>, enable: bool) -> SetStorageBucketTrackingParamsBuilder<'a> {
        SetStorageBucketTrackingParamsBuilder {
            storage_key: storage_key.into(),
            enable: enable,
        }
    }
    pub fn storage_key(&self) -> &str { self.storage_key.as_ref() }
    pub fn enable(&self) -> bool { self.enable }
}


pub struct SetStorageBucketTrackingParamsBuilder<'a> {
    storage_key: Cow<'a, str>,
    enable: bool,
}

impl<'a> SetStorageBucketTrackingParamsBuilder<'a> {
    pub fn build(self) -> SetStorageBucketTrackingParams<'a> {
        SetStorageBucketTrackingParams {
            storage_key: self.storage_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `bucket`: 
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
    #[serde(rename = "deletedSites")]
    deleted_sites: Vec<Cow<'a, str>>,
}

impl<'a> RunBounceTrackingMitigationsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `deleted_sites`: 
    pub fn builder(deleted_sites: Vec<Cow<'a, str>>) -> RunBounceTrackingMitigationsReturnsBuilder<'a> {
        RunBounceTrackingMitigationsReturnsBuilder {
            deleted_sites: deleted_sites,
        }
    }
    pub fn deleted_sites(&self) -> &[Cow<'a, str>] { &self.deleted_sites }
}


pub struct RunBounceTrackingMitigationsReturnsBuilder<'a> {
    deleted_sites: Vec<Cow<'a, str>>,
}

impl<'a> RunBounceTrackingMitigationsReturnsBuilder<'a> {
    pub fn build(self) -> RunBounceTrackingMitigationsReturns<'a> {
        RunBounceTrackingMitigationsReturns {
            deleted_sites: self.deleted_sites,
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
    /// Creates a builder for this type with the required parameters:
    /// * `sets`: 
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
    /// Creates a builder for this type with the required parameters:
    /// * `owner`: 
    /// * `name`: 
    /// * `hashes`: 
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
