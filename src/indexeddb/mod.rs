use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Database with an array of object stores.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseWithObjectStores<'a> {
    /// Database name.
    name: Cow<'a, str>,
    /// Database version (type is not 'integer', as the standard
    /// requires the version number to be 'unsigned long long')
    version: f64,
    /// Object stores in this database.
    objectStores: Vec<ObjectStore<'a>>,
}

impl<'a> DatabaseWithObjectStores<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, version: f64, objectStores: Vec<ObjectStore<'a>>) -> DatabaseWithObjectStoresBuilder<'a> {
        DatabaseWithObjectStoresBuilder {
            name: name.into(),
            version: version,
            objectStores: objectStores,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn version(&self) -> f64 { self.version }
    pub fn objectStores(&self) -> &[ObjectStore<'a>] { &self.objectStores }
}


pub struct DatabaseWithObjectStoresBuilder<'a> {
    name: Cow<'a, str>,
    version: f64,
    objectStores: Vec<ObjectStore<'a>>,
}

impl<'a> DatabaseWithObjectStoresBuilder<'a> {
    pub fn build(self) -> DatabaseWithObjectStores<'a> {
        DatabaseWithObjectStores {
            name: self.name,
            version: self.version,
            objectStores: self.objectStores,
        }
    }
}

/// Object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStore<'a> {
    /// Object store name.
    name: Cow<'a, str>,
    /// Object store key path.
    keyPath: KeyPath<'a>,
    /// If true, object store has auto increment flag set.
    autoIncrement: bool,
    /// Indexes in this object store.
    indexes: Vec<ObjectStoreIndex<'a>>,
}

impl<'a> ObjectStore<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, keyPath: KeyPath<'a>, autoIncrement: bool, indexes: Vec<ObjectStoreIndex<'a>>) -> ObjectStoreBuilder<'a> {
        ObjectStoreBuilder {
            name: name.into(),
            keyPath: keyPath,
            autoIncrement: autoIncrement,
            indexes: indexes,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn keyPath(&self) -> &KeyPath<'a> { &self.keyPath }
    pub fn autoIncrement(&self) -> bool { self.autoIncrement }
    pub fn indexes(&self) -> &[ObjectStoreIndex<'a>] { &self.indexes }
}


pub struct ObjectStoreBuilder<'a> {
    name: Cow<'a, str>,
    keyPath: KeyPath<'a>,
    autoIncrement: bool,
    indexes: Vec<ObjectStoreIndex<'a>>,
}

impl<'a> ObjectStoreBuilder<'a> {
    pub fn build(self) -> ObjectStore<'a> {
        ObjectStore {
            name: self.name,
            keyPath: self.keyPath,
            autoIncrement: self.autoIncrement,
            indexes: self.indexes,
        }
    }
}

/// Object store index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectStoreIndex<'a> {
    /// Index name.
    name: Cow<'a, str>,
    /// Index key path.
    keyPath: KeyPath<'a>,
    /// If true, index is unique.
    unique: bool,
    /// If true, index allows multiple entries for a key.
    multiEntry: bool,
}

impl<'a> ObjectStoreIndex<'a> {
    pub fn builder(name: impl Into<Cow<'a, str>>, keyPath: KeyPath<'a>, unique: bool, multiEntry: bool) -> ObjectStoreIndexBuilder<'a> {
        ObjectStoreIndexBuilder {
            name: name.into(),
            keyPath: keyPath,
            unique: unique,
            multiEntry: multiEntry,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn keyPath(&self) -> &KeyPath<'a> { &self.keyPath }
    pub fn unique(&self) -> bool { self.unique }
    pub fn multiEntry(&self) -> bool { self.multiEntry }
}


pub struct ObjectStoreIndexBuilder<'a> {
    name: Cow<'a, str>,
    keyPath: KeyPath<'a>,
    unique: bool,
    multiEntry: bool,
}

impl<'a> ObjectStoreIndexBuilder<'a> {
    pub fn build(self) -> ObjectStoreIndex<'a> {
        ObjectStoreIndex {
            name: self.name,
            keyPath: self.keyPath,
            unique: self.unique,
            multiEntry: self.multiEntry,
        }
    }
}

/// Key.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Key<'a> {
    /// Key type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Number value.
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<f64>,
    /// String value.
    #[serde(skip_serializing_if = "Option::is_none")]
    string: Option<Cow<'a, str>>,
    /// Date value.
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<f64>,
    /// Array value.
    #[serde(skip_serializing_if = "Option::is_none")]
    array: Option<Vec<Box<Key<'a>>>>,
}

impl<'a> Key<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> KeyBuilder<'a> {
        KeyBuilder {
            type_: type_.into(),
            number: None,
            string: None,
            date: None,
            array: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn number(&self) -> Option<f64> { self.number }
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    pub fn date(&self) -> Option<f64> { self.date }
    pub fn array(&self) -> Option<&[Box<Key<'a>>]> { self.array.as_deref() }
}


pub struct KeyBuilder<'a> {
    type_: Cow<'a, str>,
    number: Option<f64>,
    string: Option<Cow<'a, str>>,
    date: Option<f64>,
    array: Option<Vec<Box<Key<'a>>>>,
}

impl<'a> KeyBuilder<'a> {
    /// Number value.
    pub fn number(mut self, number: f64) -> Self { self.number = Some(number); self }
    /// String value.
    pub fn string(mut self, string: impl Into<Cow<'a, str>>) -> Self { self.string = Some(string.into()); self }
    /// Date value.
    pub fn date(mut self, date: f64) -> Self { self.date = Some(date); self }
    /// Array value.
    pub fn array(mut self, array: Vec<Box<Key<'a>>>) -> Self { self.array = Some(array); self }
    pub fn build(self) -> Key<'a> {
        Key {
            type_: self.type_,
            number: self.number,
            string: self.string,
            date: self.date,
            array: self.array,
        }
    }
}

/// Key range.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyRange<'a> {
    /// Lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    lower: Option<Key<'a>>,
    /// Upper bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    upper: Option<Key<'a>>,
    /// If true lower bound is open.
    lowerOpen: bool,
    /// If true upper bound is open.
    upperOpen: bool,
}

impl<'a> KeyRange<'a> {
    pub fn builder(lowerOpen: bool, upperOpen: bool) -> KeyRangeBuilder<'a> {
        KeyRangeBuilder {
            lower: None,
            upper: None,
            lowerOpen: lowerOpen,
            upperOpen: upperOpen,
        }
    }
    pub fn lower(&self) -> Option<&Key<'a>> { self.lower.as_ref() }
    pub fn upper(&self) -> Option<&Key<'a>> { self.upper.as_ref() }
    pub fn lowerOpen(&self) -> bool { self.lowerOpen }
    pub fn upperOpen(&self) -> bool { self.upperOpen }
}


pub struct KeyRangeBuilder<'a> {
    lower: Option<Key<'a>>,
    upper: Option<Key<'a>>,
    lowerOpen: bool,
    upperOpen: bool,
}

impl<'a> KeyRangeBuilder<'a> {
    /// Lower bound.
    pub fn lower(mut self, lower: Key<'a>) -> Self { self.lower = Some(lower); self }
    /// Upper bound.
    pub fn upper(mut self, upper: Key<'a>) -> Self { self.upper = Some(upper); self }
    pub fn build(self) -> KeyRange<'a> {
        KeyRange {
            lower: self.lower,
            upper: self.upper,
            lowerOpen: self.lowerOpen,
            upperOpen: self.upperOpen,
        }
    }
}

/// Data entry.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataEntry {
    /// Key object.
    key: crate::runtime::RemoteObject,
    /// Primary key object.
    primaryKey: crate::runtime::RemoteObject,
    /// Value object.
    value: crate::runtime::RemoteObject,
}

impl DataEntry {
    pub fn builder(key: crate::runtime::RemoteObject, primaryKey: crate::runtime::RemoteObject, value: crate::runtime::RemoteObject) -> DataEntryBuilder {
        DataEntryBuilder {
            key: key,
            primaryKey: primaryKey,
            value: value,
        }
    }
    pub fn key(&self) -> &crate::runtime::RemoteObject { &self.key }
    pub fn primaryKey(&self) -> &crate::runtime::RemoteObject { &self.primaryKey }
    pub fn value(&self) -> &crate::runtime::RemoteObject { &self.value }
}


pub struct DataEntryBuilder {
    key: crate::runtime::RemoteObject,
    primaryKey: crate::runtime::RemoteObject,
    value: crate::runtime::RemoteObject,
}

impl DataEntryBuilder {
    pub fn build(self) -> DataEntry {
        DataEntry {
            key: self.key,
            primaryKey: self.primaryKey,
            value: self.value,
        }
    }
}

/// Key path.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyPath<'a> {
    /// Key path type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// String value.
    #[serde(skip_serializing_if = "Option::is_none")]
    string: Option<Cow<'a, str>>,
    /// Array value.
    #[serde(skip_serializing_if = "Option::is_none")]
    array: Option<Vec<Cow<'a, str>>>,
}

impl<'a> KeyPath<'a> {
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> KeyPathBuilder<'a> {
        KeyPathBuilder {
            type_: type_.into(),
            string: None,
            array: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    pub fn array(&self) -> Option<&[Cow<'a, str>]> { self.array.as_deref() }
}


pub struct KeyPathBuilder<'a> {
    type_: Cow<'a, str>,
    string: Option<Cow<'a, str>>,
    array: Option<Vec<Cow<'a, str>>>,
}

impl<'a> KeyPathBuilder<'a> {
    /// String value.
    pub fn string(mut self, string: impl Into<Cow<'a, str>>) -> Self { self.string = Some(string.into()); self }
    /// Array value.
    pub fn array(mut self, array: Vec<Cow<'a, str>>) -> Self { self.array = Some(array); self }
    pub fn build(self) -> KeyPath<'a> {
        KeyPath {
            type_: self.type_,
            string: self.string,
            array: self.array,
        }
    }
}

/// Clears all entries from an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearObjectStoreParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    databaseName: Cow<'a, str>,
    /// Object store name.
    objectStoreName: Cow<'a, str>,
}

impl<'a> ClearObjectStoreParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>, objectStoreName: impl Into<Cow<'a, str>>) -> ClearObjectStoreParamsBuilder<'a> {
        ClearObjectStoreParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
            objectStoreName: objectStoreName.into(),
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
}


pub struct ClearObjectStoreParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
    objectStoreName: Cow<'a, str>,
}

impl<'a> ClearObjectStoreParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> ClearObjectStoreParams<'a> {
        ClearObjectStoreParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
            objectStoreName: self.objectStoreName,
        }
    }
}

impl<'a> ClearObjectStoreParams<'a> { pub const METHOD: &'static str = "IndexedDB.clearObjectStore"; }

impl<'a> crate::CdpCommand<'a> for ClearObjectStoreParams<'a> {
    const METHOD: &'static str = "IndexedDB.clearObjectStore";
    type Response = crate::EmptyReturns;
}

/// Deletes a database.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatabaseParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    databaseName: Cow<'a, str>,
}

impl<'a> DeleteDatabaseParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>) -> DeleteDatabaseParamsBuilder<'a> {
        DeleteDatabaseParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
}


pub struct DeleteDatabaseParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
}

impl<'a> DeleteDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> DeleteDatabaseParams<'a> {
        DeleteDatabaseParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
        }
    }
}

impl<'a> DeleteDatabaseParams<'a> { pub const METHOD: &'static str = "IndexedDB.deleteDatabase"; }

impl<'a> crate::CdpCommand<'a> for DeleteDatabaseParams<'a> {
    const METHOD: &'static str = "IndexedDB.deleteDatabase";
    type Response = crate::EmptyReturns;
}

/// Delete a range of entries from an object store

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeleteObjectStoreEntriesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
    objectStoreName: Cow<'a, str>,
    /// Range of entry keys to delete
    keyRange: KeyRange<'a>,
}

impl<'a> DeleteObjectStoreEntriesParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>, objectStoreName: impl Into<Cow<'a, str>>, keyRange: KeyRange<'a>) -> DeleteObjectStoreEntriesParamsBuilder<'a> {
        DeleteObjectStoreEntriesParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
            objectStoreName: objectStoreName.into(),
            keyRange: keyRange,
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
    pub fn keyRange(&self) -> &KeyRange<'a> { &self.keyRange }
}


pub struct DeleteObjectStoreEntriesParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
    objectStoreName: Cow<'a, str>,
    keyRange: KeyRange<'a>,
}

impl<'a> DeleteObjectStoreEntriesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> DeleteObjectStoreEntriesParams<'a> {
        DeleteObjectStoreEntriesParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
            objectStoreName: self.objectStoreName,
            keyRange: self.keyRange,
        }
    }
}

impl<'a> DeleteObjectStoreEntriesParams<'a> { pub const METHOD: &'static str = "IndexedDB.deleteObjectStoreEntries"; }

impl<'a> crate::CdpCommand<'a> for DeleteObjectStoreEntriesParams<'a> {
    const METHOD: &'static str = "IndexedDB.deleteObjectStoreEntries";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "IndexedDB.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "IndexedDB.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "IndexedDB.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "IndexedDB.enable";
    type Response = crate::EmptyReturns;
}

/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    databaseName: Cow<'a, str>,
    /// Object store name.
    objectStoreName: Cow<'a, str>,
    /// Index name. If not specified, it performs an object store data request.
    #[serde(skip_serializing_if = "Option::is_none")]
    indexName: Option<Cow<'a, str>>,
    /// Number of records to skip.
    skipCount: u64,
    /// Number of records to fetch.
    pageSize: u64,
    /// Key range.
    #[serde(skip_serializing_if = "Option::is_none")]
    keyRange: Option<KeyRange<'a>>,
}

impl<'a> RequestDataParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>, objectStoreName: impl Into<Cow<'a, str>>, skipCount: u64, pageSize: u64) -> RequestDataParamsBuilder<'a> {
        RequestDataParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
            objectStoreName: objectStoreName.into(),
            indexName: None,
            skipCount: skipCount,
            pageSize: pageSize,
            keyRange: None,
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
    pub fn indexName(&self) -> Option<&str> { self.indexName.as_deref() }
    pub fn skipCount(&self) -> u64 { self.skipCount }
    pub fn pageSize(&self) -> u64 { self.pageSize }
    pub fn keyRange(&self) -> Option<&KeyRange<'a>> { self.keyRange.as_ref() }
}


pub struct RequestDataParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
    objectStoreName: Cow<'a, str>,
    indexName: Option<Cow<'a, str>>,
    skipCount: u64,
    pageSize: u64,
    keyRange: Option<KeyRange<'a>>,
}

impl<'a> RequestDataParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// Index name. If not specified, it performs an object store data request.
    pub fn indexName(mut self, indexName: impl Into<Cow<'a, str>>) -> Self { self.indexName = Some(indexName.into()); self }
    /// Key range.
    pub fn keyRange(mut self, keyRange: KeyRange<'a>) -> Self { self.keyRange = Some(keyRange); self }
    pub fn build(self) -> RequestDataParams<'a> {
        RequestDataParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
            objectStoreName: self.objectStoreName,
            indexName: self.indexName,
            skipCount: self.skipCount,
            pageSize: self.pageSize,
            keyRange: self.keyRange,
        }
    }
}

/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataReturns {
    /// Array of object store data entries.
    objectStoreDataEntries: Vec<DataEntry>,
    /// If true, there are more entries to fetch in the given range.
    hasMore: bool,
}

impl RequestDataReturns {
    pub fn builder(objectStoreDataEntries: Vec<DataEntry>, hasMore: bool) -> RequestDataReturnsBuilder {
        RequestDataReturnsBuilder {
            objectStoreDataEntries: objectStoreDataEntries,
            hasMore: hasMore,
        }
    }
    pub fn objectStoreDataEntries(&self) -> &[DataEntry] { &self.objectStoreDataEntries }
    pub fn hasMore(&self) -> bool { self.hasMore }
}


pub struct RequestDataReturnsBuilder {
    objectStoreDataEntries: Vec<DataEntry>,
    hasMore: bool,
}

impl RequestDataReturnsBuilder {
    pub fn build(self) -> RequestDataReturns {
        RequestDataReturns {
            objectStoreDataEntries: self.objectStoreDataEntries,
            hasMore: self.hasMore,
        }
    }
}

impl<'a> RequestDataParams<'a> { pub const METHOD: &'static str = "IndexedDB.requestData"; }

impl<'a> crate::CdpCommand<'a> for RequestDataParams<'a> {
    const METHOD: &'static str = "IndexedDB.requestData";
    type Response = RequestDataReturns;
}

/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    databaseName: Cow<'a, str>,
    /// Object store name.
    objectStoreName: Cow<'a, str>,
}

impl<'a> GetMetadataParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>, objectStoreName: impl Into<Cow<'a, str>>) -> GetMetadataParamsBuilder<'a> {
        GetMetadataParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
            objectStoreName: objectStoreName.into(),
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
}


pub struct GetMetadataParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
    objectStoreName: Cow<'a, str>,
}

impl<'a> GetMetadataParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> GetMetadataParams<'a> {
        GetMetadataParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
            objectStoreName: self.objectStoreName,
        }
    }
}

/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataReturns {
    /// the entries count
    entriesCount: f64,
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.
    keyGeneratorValue: f64,
}

impl GetMetadataReturns {
    pub fn builder(entriesCount: f64, keyGeneratorValue: f64) -> GetMetadataReturnsBuilder {
        GetMetadataReturnsBuilder {
            entriesCount: entriesCount,
            keyGeneratorValue: keyGeneratorValue,
        }
    }
    pub fn entriesCount(&self) -> f64 { self.entriesCount }
    pub fn keyGeneratorValue(&self) -> f64 { self.keyGeneratorValue }
}


pub struct GetMetadataReturnsBuilder {
    entriesCount: f64,
    keyGeneratorValue: f64,
}

impl GetMetadataReturnsBuilder {
    pub fn build(self) -> GetMetadataReturns {
        GetMetadataReturns {
            entriesCount: self.entriesCount,
            keyGeneratorValue: self.keyGeneratorValue,
        }
    }
}

impl<'a> GetMetadataParams<'a> { pub const METHOD: &'static str = "IndexedDB.getMetadata"; }

impl<'a> crate::CdpCommand<'a> for GetMetadataParams<'a> {
    const METHOD: &'static str = "IndexedDB.getMetadata";
    type Response = GetMetadataReturns;
}

/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    securityOrigin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageKey: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    databaseName: Cow<'a, str>,
}

impl<'a> RequestDatabaseParams<'a> {
    pub fn builder(databaseName: impl Into<Cow<'a, str>>) -> RequestDatabaseParamsBuilder<'a> {
        RequestDatabaseParamsBuilder {
            securityOrigin: None,
            storageKey: None,
            storageBucket: None,
            databaseName: databaseName.into(),
        }
    }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
}


pub struct RequestDatabaseParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Cow<'a, str>,
}

impl<'a> RequestDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> RequestDatabaseParams<'a> {
        RequestDatabaseParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName,
        }
    }
}

/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseReturns<'a> {
    /// Database with an array of object stores.
    databaseWithObjectStores: DatabaseWithObjectStores<'a>,
}

impl<'a> RequestDatabaseReturns<'a> {
    pub fn builder(databaseWithObjectStores: DatabaseWithObjectStores<'a>) -> RequestDatabaseReturnsBuilder<'a> {
        RequestDatabaseReturnsBuilder {
            databaseWithObjectStores: databaseWithObjectStores,
        }
    }
    pub fn databaseWithObjectStores(&self) -> &DatabaseWithObjectStores<'a> { &self.databaseWithObjectStores }
}


pub struct RequestDatabaseReturnsBuilder<'a> {
    databaseWithObjectStores: DatabaseWithObjectStores<'a>,
}

impl<'a> RequestDatabaseReturnsBuilder<'a> {
    pub fn build(self) -> RequestDatabaseReturns<'a> {
        RequestDatabaseReturns {
            databaseWithObjectStores: self.databaseWithObjectStores,
        }
    }
}

impl<'a> RequestDatabaseParams<'a> { pub const METHOD: &'static str = "IndexedDB.requestDatabase"; }

impl<'a> crate::CdpCommand<'a> for RequestDatabaseParams<'a> {
    const METHOD: &'static str = "IndexedDB.requestDatabase";
    type Response = RequestDatabaseReturns<'a>;
}

/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesParams<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
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

impl<'a> RequestDatabaseNamesParams<'a> {
    pub fn builder() -> RequestDatabaseNamesParamsBuilder<'a> {
        RequestDatabaseNamesParamsBuilder {
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
pub struct RequestDatabaseNamesParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestDatabaseNamesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn build(self) -> RequestDatabaseNamesParams<'a> {
        RequestDatabaseNamesParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
        }
    }
}

/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesReturns<'a> {
    /// Database names for origin.
    databaseNames: Vec<Cow<'a, str>>,
}

impl<'a> RequestDatabaseNamesReturns<'a> {
    pub fn builder(databaseNames: Vec<Cow<'a, str>>) -> RequestDatabaseNamesReturnsBuilder<'a> {
        RequestDatabaseNamesReturnsBuilder {
            databaseNames: databaseNames,
        }
    }
    pub fn databaseNames(&self) -> &[Cow<'a, str>] { &self.databaseNames }
}


pub struct RequestDatabaseNamesReturnsBuilder<'a> {
    databaseNames: Vec<Cow<'a, str>>,
}

impl<'a> RequestDatabaseNamesReturnsBuilder<'a> {
    pub fn build(self) -> RequestDatabaseNamesReturns<'a> {
        RequestDatabaseNamesReturns {
            databaseNames: self.databaseNames,
        }
    }
}

impl<'a> RequestDatabaseNamesParams<'a> { pub const METHOD: &'static str = "IndexedDB.requestDatabaseNames"; }

impl<'a> crate::CdpCommand<'a> for RequestDatabaseNamesParams<'a> {
    const METHOD: &'static str = "IndexedDB.requestDatabaseNames";
    type Response = RequestDatabaseNamesReturns<'a>;
}
