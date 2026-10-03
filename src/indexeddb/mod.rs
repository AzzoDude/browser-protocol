use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Database with an array of object stores.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseWithObjectStores<'a> {
    /// Database name.
    pub name: Cow<'a, str>,
    /// Database version (type is not 'integer', as the standard
    /// requires the version number to be 'unsigned long long')
    pub version: f64,
    /// Object stores in this database.
    #[serde(rename = "objectStores")]
    pub object_stores: Vec<ObjectStore<'a>>,
}
/// Object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStore<'a> {
    /// Object store name.
    pub name: Cow<'a, str>,
    /// Object store key path.
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath<'a>,
    /// If true, object store has auto increment flag set.
    #[serde(rename = "autoIncrement")]
    pub auto_increment: bool,
    /// Indexes in this object store.
    pub indexes: Vec<ObjectStoreIndex<'a>>,
}
/// Object store index.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStoreIndex<'a> {
    /// Index name.
    pub name: Cow<'a, str>,
    /// Index key path.
    #[serde(rename = "keyPath")]
    pub key_path: KeyPath<'a>,
    /// If true, index is unique.
    pub unique: bool,
    /// If true, index allows multiple entries for a key.
    #[serde(rename = "multiEntry")]
    pub multi_entry: bool,
}
/// Key.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Key<'a> {
    /// Key type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Number value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    /// String value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<Cow<'a, str>>,
    /// Date value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// Array value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Box<Key<'a>>>>,
}
/// Key range.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct KeyRange<'a> {
    /// Lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower: Option<Key<'a>>,
    /// Upper bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper: Option<Key<'a>>,
    /// If true lower bound is open.
    #[serde(rename = "lowerOpen")]
    pub lower_open: bool,
    /// If true upper bound is open.
    #[serde(rename = "upperOpen")]
    pub upper_open: bool,
}
/// Data entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry {
    /// Key object.
    pub key: crate::runtime::RemoteObject,
    /// Primary key object.
    #[serde(rename = "primaryKey")]
    pub primary_key: crate::runtime::RemoteObject,
    /// Value object.
    pub value: crate::runtime::RemoteObject,
}
/// Key path.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct KeyPath<'a> {
    /// Key path type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// String value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<Cow<'a, str>>,
    /// Array value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Cow<'a, str>>>,
}
/// Clears all entries from an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.clearObjectStore")]
pub struct ClearObjectStoreParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    pub object_store_name: Cow<'a, str>,
}
/// Deletes a database.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.deleteDatabase")]
pub struct DeleteDatabaseParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
}
/// Delete a range of entries from an object store

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.deleteObjectStoreEntries")]
pub struct DeleteObjectStoreEntriesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
    #[serde(rename = "objectStoreName")]
    pub object_store_name: Cow<'a, str>,
    /// Range of entry keys to delete
    #[serde(rename = "keyRange")]
    pub key_range: KeyRange<'a>,
}
/// Disables events from backend.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.disable")]
pub struct DisableParams {

}
/// Enables events from backend.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.enable")]
pub struct EnableParams {

}
/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.requestData", response = "RequestDataReturns")]
pub struct RequestDataParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    pub object_store_name: Cow<'a, str>,
    /// Index name. If not specified, it performs an object store data request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "indexName")]
    pub index_name: Option<Cow<'a, str>>,
    /// Number of records to skip.
    #[serde(rename = "skipCount")]
    pub skip_count: u64,
    /// Number of records to fetch.
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    /// Key range.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyRange")]
    pub key_range: Option<KeyRange<'a>>,
}
/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataReturns {
    /// Array of object store data entries.
    #[serde(rename = "objectStoreDataEntries")]
    pub object_store_data_entries: Vec<DataEntry>,
    /// If true, there are more entries to fetch in the given range.
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.getMetadata", response = "GetMetadataReturns")]
pub struct GetMetadataParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    pub object_store_name: Cow<'a, str>,
}
/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataReturns {
    /// the entries count
    #[serde(rename = "entriesCount")]
    pub entries_count: f64,
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.
    #[serde(rename = "keyGeneratorValue")]
    pub key_generator_value: f64,
}
/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.requestDatabase", response = "RequestDatabaseReturns<'a>")]
pub struct RequestDatabaseParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    pub database_name: Cow<'a, str>,
}
/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseReturns<'a> {
    /// Database with an array of object stores.
    #[serde(rename = "databaseWithObjectStores")]
    pub database_with_object_stores: DatabaseWithObjectStores<'a>,
}
/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "IndexedDB.requestDatabaseNames", response = "RequestDatabaseNamesReturns<'a>")]
pub struct RequestDatabaseNamesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    pub security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    pub storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    pub storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}
/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesReturns<'a> {
    /// Database names for origin.
    #[serde(rename = "databaseNames")]
    pub database_names: Vec<Cow<'a, str>>,
}