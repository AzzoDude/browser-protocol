use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Database with an array of object stores.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseWithObjectStores {
    /// Database name.

    pub name: String,
    /// Database version (type is not 'integer', as the standard
    /// requires the version number to be 'unsigned long long')

    pub version: f64,
    /// Object stores in this database.

    pub objectStores: Vec<ObjectStore>,
}

/// Object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStore {
    /// Object store name.

    pub name: String,
    /// Object store key path.

    pub keyPath: KeyPath,
    /// If true, object store has auto increment flag set.

    pub autoIncrement: bool,
    /// Indexes in this object store.

    pub indexes: Vec<ObjectStoreIndex>,
}

/// Object store index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStoreIndex {
    /// Index name.

    pub name: String,
    /// Index key path.

    pub keyPath: KeyPath,
    /// If true, index is unique.

    pub unique: bool,
    /// If true, index allows multiple entries for a key.

    pub multiEntry: bool,
}

/// Key.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    /// Key type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Number value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    /// String value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// Date value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// Array value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Key>>,
}

/// Key range.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyRange {
    /// Lower bound.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower: Option<Key>,
    /// Upper bound.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper: Option<Key>,
    /// If true lower bound is open.

    pub lowerOpen: bool,
    /// If true upper bound is open.

    pub upperOpen: bool,
}

/// Data entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry {
    /// Key object.

    pub key: crate::runtime::RemoteObject,
    /// Primary key object.

    pub primaryKey: crate::runtime::RemoteObject,
    /// Value object.

    pub value: crate::runtime::RemoteObject,
}

/// Key path.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyPath {
    /// Key path type.

    #[serde(rename = "type")]
    pub type_: String,
    /// String value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// Array value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<String>>,
}

/// Clears all entries from an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearObjectStoreParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// Database name.

    pub databaseName: String,
    /// Object store name.

    pub objectStoreName: String,
}

/// Deletes a database.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatabaseParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// Database name.

    pub databaseName: String,
}

/// Delete a range of entries from an object store

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectStoreEntriesParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,

    pub databaseName: String,

    pub objectStoreName: String,
    /// Range of entry keys to delete

    pub keyRange: KeyRange,
}

/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// Database name.

    pub databaseName: String,
    /// Object store name.

    pub objectStoreName: String,
    /// Index name. If not specified, it performs an object store data request.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexName: Option<String>,
    /// Number of records to skip.

    pub skipCount: u64,
    /// Number of records to fetch.

    pub pageSize: u64,
    /// Key range.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyRange: Option<KeyRange>,
}

/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataReturns {
    /// Array of object store data entries.

    pub objectStoreDataEntries: Vec<DataEntry>,
    /// If true, there are more entries to fetch in the given range.

    pub hasMore: bool,
}

/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// Database name.

    pub databaseName: String,
    /// Object store name.

    pub objectStoreName: String,
}

/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataReturns {
    /// the entries count

    pub entriesCount: f64,
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.

    pub keyGeneratorValue: f64,
}

/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityOrigin: Option<String>,
    /// Storage key.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageKey: Option<String>,
    /// Storage bucket. If not specified, it uses the default bucket.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageBucket: Option<crate::storage::StorageBucket>,
    /// Database name.

    pub databaseName: String,
}

/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseReturns {
    /// Database with an array of object stores.

    pub databaseWithObjectStores: DatabaseWithObjectStores,
}

/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesParams {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
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

/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesReturns {
    /// Database names for origin.

    pub databaseNames: Vec<String>,
}
