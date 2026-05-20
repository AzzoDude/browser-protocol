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
    pub fn builder() -> DatabaseWithObjectStoresBuilder<'a> { DatabaseWithObjectStoresBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn version(&self) -> f64 { self.version }
    pub fn objectStores(&self) -> &[ObjectStore<'a>] { &self.objectStores }
}

#[derive(Default)]
pub struct DatabaseWithObjectStoresBuilder<'a> {
    name: Option<Cow<'a, str>>,
    version: Option<f64>,
    objectStores: Option<Vec<ObjectStore<'a>>>,
}

impl<'a> DatabaseWithObjectStoresBuilder<'a> {
    /// Database name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Database version (type is not 'integer', as the standard
    /// requires the version number to be 'unsigned long long')
    pub fn version(mut self, version: f64) -> Self { self.version = Some(version); self }
    /// Object stores in this database.
    pub fn objectStores(mut self, objectStores: Vec<ObjectStore<'a>>) -> Self { self.objectStores = Some(objectStores); self }
    pub fn build(self) -> DatabaseWithObjectStores<'a> {
        DatabaseWithObjectStores {
            name: self.name.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
            objectStores: self.objectStores.unwrap_or_default(),
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
    pub fn builder() -> ObjectStoreBuilder<'a> { ObjectStoreBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn keyPath(&self) -> &KeyPath<'a> { &self.keyPath }
    pub fn autoIncrement(&self) -> bool { self.autoIncrement }
    pub fn indexes(&self) -> &[ObjectStoreIndex<'a>] { &self.indexes }
}

#[derive(Default)]
pub struct ObjectStoreBuilder<'a> {
    name: Option<Cow<'a, str>>,
    keyPath: Option<KeyPath<'a>>,
    autoIncrement: Option<bool>,
    indexes: Option<Vec<ObjectStoreIndex<'a>>>,
}

impl<'a> ObjectStoreBuilder<'a> {
    /// Object store name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Object store key path.
    pub fn keyPath(mut self, keyPath: KeyPath<'a>) -> Self { self.keyPath = Some(keyPath); self }
    /// If true, object store has auto increment flag set.
    pub fn autoIncrement(mut self, autoIncrement: bool) -> Self { self.autoIncrement = Some(autoIncrement); self }
    /// Indexes in this object store.
    pub fn indexes(mut self, indexes: Vec<ObjectStoreIndex<'a>>) -> Self { self.indexes = Some(indexes); self }
    pub fn build(self) -> ObjectStore<'a> {
        ObjectStore {
            name: self.name.unwrap_or_default(),
            keyPath: self.keyPath.unwrap_or_default(),
            autoIncrement: self.autoIncrement.unwrap_or_default(),
            indexes: self.indexes.unwrap_or_default(),
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
    pub fn builder() -> ObjectStoreIndexBuilder<'a> { ObjectStoreIndexBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn keyPath(&self) -> &KeyPath<'a> { &self.keyPath }
    pub fn unique(&self) -> bool { self.unique }
    pub fn multiEntry(&self) -> bool { self.multiEntry }
}

#[derive(Default)]
pub struct ObjectStoreIndexBuilder<'a> {
    name: Option<Cow<'a, str>>,
    keyPath: Option<KeyPath<'a>>,
    unique: Option<bool>,
    multiEntry: Option<bool>,
}

impl<'a> ObjectStoreIndexBuilder<'a> {
    /// Index name.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Index key path.
    pub fn keyPath(mut self, keyPath: KeyPath<'a>) -> Self { self.keyPath = Some(keyPath); self }
    /// If true, index is unique.
    pub fn unique(mut self, unique: bool) -> Self { self.unique = Some(unique); self }
    /// If true, index allows multiple entries for a key.
    pub fn multiEntry(mut self, multiEntry: bool) -> Self { self.multiEntry = Some(multiEntry); self }
    pub fn build(self) -> ObjectStoreIndex<'a> {
        ObjectStoreIndex {
            name: self.name.unwrap_or_default(),
            keyPath: self.keyPath.unwrap_or_default(),
            unique: self.unique.unwrap_or_default(),
            multiEntry: self.multiEntry.unwrap_or_default(),
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
    pub fn builder() -> KeyBuilder<'a> { KeyBuilder::default() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn number(&self) -> Option<f64> { self.number }
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    pub fn date(&self) -> Option<f64> { self.date }
    pub fn array(&self) -> Option<&[Box<Key<'a>>]> { self.array.as_deref() }
}

#[derive(Default)]
pub struct KeyBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    number: Option<f64>,
    string: Option<Cow<'a, str>>,
    date: Option<f64>,
    array: Option<Vec<Box<Key<'a>>>>,
}

impl<'a> KeyBuilder<'a> {
    /// Key type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
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
            type_: self.type_.unwrap_or_default(),
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
    pub fn builder() -> KeyRangeBuilder<'a> { KeyRangeBuilder::default() }
    pub fn lower(&self) -> Option<&Key<'a>> { self.lower.as_ref() }
    pub fn upper(&self) -> Option<&Key<'a>> { self.upper.as_ref() }
    pub fn lowerOpen(&self) -> bool { self.lowerOpen }
    pub fn upperOpen(&self) -> bool { self.upperOpen }
}

#[derive(Default)]
pub struct KeyRangeBuilder<'a> {
    lower: Option<Key<'a>>,
    upper: Option<Key<'a>>,
    lowerOpen: Option<bool>,
    upperOpen: Option<bool>,
}

impl<'a> KeyRangeBuilder<'a> {
    /// Lower bound.
    pub fn lower(mut self, lower: Key<'a>) -> Self { self.lower = Some(lower); self }
    /// Upper bound.
    pub fn upper(mut self, upper: Key<'a>) -> Self { self.upper = Some(upper); self }
    /// If true lower bound is open.
    pub fn lowerOpen(mut self, lowerOpen: bool) -> Self { self.lowerOpen = Some(lowerOpen); self }
    /// If true upper bound is open.
    pub fn upperOpen(mut self, upperOpen: bool) -> Self { self.upperOpen = Some(upperOpen); self }
    pub fn build(self) -> KeyRange<'a> {
        KeyRange {
            lower: self.lower,
            upper: self.upper,
            lowerOpen: self.lowerOpen.unwrap_or_default(),
            upperOpen: self.upperOpen.unwrap_or_default(),
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
    pub fn builder() -> DataEntryBuilder { DataEntryBuilder::default() }
    pub fn key(&self) -> &crate::runtime::RemoteObject { &self.key }
    pub fn primaryKey(&self) -> &crate::runtime::RemoteObject { &self.primaryKey }
    pub fn value(&self) -> &crate::runtime::RemoteObject { &self.value }
}

#[derive(Default)]
pub struct DataEntryBuilder {
    key: Option<crate::runtime::RemoteObject>,
    primaryKey: Option<crate::runtime::RemoteObject>,
    value: Option<crate::runtime::RemoteObject>,
}

impl DataEntryBuilder {
    /// Key object.
    pub fn key(mut self, key: crate::runtime::RemoteObject) -> Self { self.key = Some(key); self }
    /// Primary key object.
    pub fn primaryKey(mut self, primaryKey: crate::runtime::RemoteObject) -> Self { self.primaryKey = Some(primaryKey); self }
    /// Value object.
    pub fn value(mut self, value: crate::runtime::RemoteObject) -> Self { self.value = Some(value); self }
    pub fn build(self) -> DataEntry {
        DataEntry {
            key: self.key.unwrap_or_default(),
            primaryKey: self.primaryKey.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
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
    pub fn builder() -> KeyPathBuilder<'a> { KeyPathBuilder::default() }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    pub fn array(&self) -> Option<&[Cow<'a, str>]> { self.array.as_deref() }
}

#[derive(Default)]
pub struct KeyPathBuilder<'a> {
    type_: Option<Cow<'a, str>>,
    string: Option<Cow<'a, str>>,
    array: Option<Vec<Cow<'a, str>>>,
}

impl<'a> KeyPathBuilder<'a> {
    /// Key path type.
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    /// String value.
    pub fn string(mut self, string: impl Into<Cow<'a, str>>) -> Self { self.string = Some(string.into()); self }
    /// Array value.
    pub fn array(mut self, array: Vec<Cow<'a, str>>) -> Self { self.array = Some(array); self }
    pub fn build(self) -> KeyPath<'a> {
        KeyPath {
            type_: self.type_.unwrap_or_default(),
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
    pub fn builder() -> ClearObjectStoreParamsBuilder<'a> { ClearObjectStoreParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
}

#[derive(Default)]
pub struct ClearObjectStoreParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
    objectStoreName: Option<Cow<'a, str>>,
}

impl<'a> ClearObjectStoreParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// Database name.
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    /// Object store name.
    pub fn objectStoreName(mut self, objectStoreName: impl Into<Cow<'a, str>>) -> Self { self.objectStoreName = Some(objectStoreName.into()); self }
    pub fn build(self) -> ClearObjectStoreParams<'a> {
        ClearObjectStoreParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
            objectStoreName: self.objectStoreName.unwrap_or_default(),
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
    pub fn builder() -> DeleteDatabaseParamsBuilder<'a> { DeleteDatabaseParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
}

#[derive(Default)]
pub struct DeleteDatabaseParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
}

impl<'a> DeleteDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// Database name.
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    pub fn build(self) -> DeleteDatabaseParams<'a> {
        DeleteDatabaseParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
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
    pub fn builder() -> DeleteObjectStoreEntriesParamsBuilder<'a> { DeleteObjectStoreEntriesParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
    pub fn keyRange(&self) -> &KeyRange<'a> { &self.keyRange }
}

#[derive(Default)]
pub struct DeleteObjectStoreEntriesParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
    objectStoreName: Option<Cow<'a, str>>,
    keyRange: Option<KeyRange<'a>>,
}

impl<'a> DeleteObjectStoreEntriesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    pub fn objectStoreName(mut self, objectStoreName: impl Into<Cow<'a, str>>) -> Self { self.objectStoreName = Some(objectStoreName.into()); self }
    /// Range of entry keys to delete
    pub fn keyRange(mut self, keyRange: KeyRange<'a>) -> Self { self.keyRange = Some(keyRange); self }
    pub fn build(self) -> DeleteObjectStoreEntriesParams<'a> {
        DeleteObjectStoreEntriesParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
            objectStoreName: self.objectStoreName.unwrap_or_default(),
            keyRange: self.keyRange.unwrap_or_default(),
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

impl DisableParams {
    pub fn builder() -> DisableParamsBuilder {
        DisableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct DisableParamsBuilder {}

impl DisableParamsBuilder {
    pub fn build(self) -> DisableParams {
        DisableParams {}
    }
}

impl DisableParams { pub const METHOD: &'static str = "IndexedDB.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "IndexedDB.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams {
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct EnableParamsBuilder {}

impl EnableParamsBuilder {
    pub fn build(self) -> EnableParams {
        EnableParams {}
    }
}

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
    pub fn builder() -> RequestDataParamsBuilder<'a> { RequestDataParamsBuilder::default() }
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

#[derive(Default)]
pub struct RequestDataParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
    objectStoreName: Option<Cow<'a, str>>,
    indexName: Option<Cow<'a, str>>,
    skipCount: Option<u64>,
    pageSize: Option<u64>,
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
    /// Database name.
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    /// Object store name.
    pub fn objectStoreName(mut self, objectStoreName: impl Into<Cow<'a, str>>) -> Self { self.objectStoreName = Some(objectStoreName.into()); self }
    /// Index name. If not specified, it performs an object store data request.
    pub fn indexName(mut self, indexName: impl Into<Cow<'a, str>>) -> Self { self.indexName = Some(indexName.into()); self }
    /// Number of records to skip.
    pub fn skipCount(mut self, skipCount: u64) -> Self { self.skipCount = Some(skipCount); self }
    /// Number of records to fetch.
    pub fn pageSize(mut self, pageSize: u64) -> Self { self.pageSize = Some(pageSize); self }
    /// Key range.
    pub fn keyRange(mut self, keyRange: KeyRange<'a>) -> Self { self.keyRange = Some(keyRange); self }
    pub fn build(self) -> RequestDataParams<'a> {
        RequestDataParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
            objectStoreName: self.objectStoreName.unwrap_or_default(),
            indexName: self.indexName,
            skipCount: self.skipCount.unwrap_or_default(),
            pageSize: self.pageSize.unwrap_or_default(),
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
    pub fn builder() -> RequestDataReturnsBuilder { RequestDataReturnsBuilder::default() }
    pub fn objectStoreDataEntries(&self) -> &[DataEntry] { &self.objectStoreDataEntries }
    pub fn hasMore(&self) -> bool { self.hasMore }
}

#[derive(Default)]
pub struct RequestDataReturnsBuilder {
    objectStoreDataEntries: Option<Vec<DataEntry>>,
    hasMore: Option<bool>,
}

impl RequestDataReturnsBuilder {
    /// Array of object store data entries.
    pub fn objectStoreDataEntries(mut self, objectStoreDataEntries: Vec<DataEntry>) -> Self { self.objectStoreDataEntries = Some(objectStoreDataEntries); self }
    /// If true, there are more entries to fetch in the given range.
    pub fn hasMore(mut self, hasMore: bool) -> Self { self.hasMore = Some(hasMore); self }
    pub fn build(self) -> RequestDataReturns {
        RequestDataReturns {
            objectStoreDataEntries: self.objectStoreDataEntries.unwrap_or_default(),
            hasMore: self.hasMore.unwrap_or_default(),
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
    pub fn builder() -> GetMetadataParamsBuilder<'a> { GetMetadataParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
    pub fn objectStoreName(&self) -> &str { self.objectStoreName.as_ref() }
}

#[derive(Default)]
pub struct GetMetadataParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
    objectStoreName: Option<Cow<'a, str>>,
}

impl<'a> GetMetadataParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// Database name.
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    /// Object store name.
    pub fn objectStoreName(mut self, objectStoreName: impl Into<Cow<'a, str>>) -> Self { self.objectStoreName = Some(objectStoreName.into()); self }
    pub fn build(self) -> GetMetadataParams<'a> {
        GetMetadataParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
            objectStoreName: self.objectStoreName.unwrap_or_default(),
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
    pub fn builder() -> GetMetadataReturnsBuilder { GetMetadataReturnsBuilder::default() }
    pub fn entriesCount(&self) -> f64 { self.entriesCount }
    pub fn keyGeneratorValue(&self) -> f64 { self.keyGeneratorValue }
}

#[derive(Default)]
pub struct GetMetadataReturnsBuilder {
    entriesCount: Option<f64>,
    keyGeneratorValue: Option<f64>,
}

impl GetMetadataReturnsBuilder {
    /// the entries count
    pub fn entriesCount(mut self, entriesCount: f64) -> Self { self.entriesCount = Some(entriesCount); self }
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.
    pub fn keyGeneratorValue(mut self, keyGeneratorValue: f64) -> Self { self.keyGeneratorValue = Some(keyGeneratorValue); self }
    pub fn build(self) -> GetMetadataReturns {
        GetMetadataReturns {
            entriesCount: self.entriesCount.unwrap_or_default(),
            keyGeneratorValue: self.keyGeneratorValue.unwrap_or_default(),
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
    pub fn builder() -> RequestDatabaseParamsBuilder<'a> { RequestDatabaseParamsBuilder::default() }
    pub fn securityOrigin(&self) -> Option<&str> { self.securityOrigin.as_deref() }
    pub fn storageKey(&self) -> Option<&str> { self.storageKey.as_deref() }
    pub fn storageBucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storageBucket.as_ref() }
    pub fn databaseName(&self) -> &str { self.databaseName.as_ref() }
}

#[derive(Default)]
pub struct RequestDatabaseParamsBuilder<'a> {
    securityOrigin: Option<Cow<'a, str>>,
    storageKey: Option<Cow<'a, str>>,
    storageBucket: Option<crate::storage::StorageBucket<'a>>,
    databaseName: Option<Cow<'a, str>>,
}

impl<'a> RequestDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn securityOrigin(mut self, securityOrigin: impl Into<Cow<'a, str>>) -> Self { self.securityOrigin = Some(securityOrigin.into()); self }
    /// Storage key.
    pub fn storageKey(mut self, storageKey: impl Into<Cow<'a, str>>) -> Self { self.storageKey = Some(storageKey.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storageBucket(mut self, storageBucket: crate::storage::StorageBucket<'a>) -> Self { self.storageBucket = Some(storageBucket); self }
    /// Database name.
    pub fn databaseName(mut self, databaseName: impl Into<Cow<'a, str>>) -> Self { self.databaseName = Some(databaseName.into()); self }
    pub fn build(self) -> RequestDatabaseParams<'a> {
        RequestDatabaseParams {
            securityOrigin: self.securityOrigin,
            storageKey: self.storageKey,
            storageBucket: self.storageBucket,
            databaseName: self.databaseName.unwrap_or_default(),
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
    pub fn builder() -> RequestDatabaseReturnsBuilder<'a> { RequestDatabaseReturnsBuilder::default() }
    pub fn databaseWithObjectStores(&self) -> &DatabaseWithObjectStores<'a> { &self.databaseWithObjectStores }
}

#[derive(Default)]
pub struct RequestDatabaseReturnsBuilder<'a> {
    databaseWithObjectStores: Option<DatabaseWithObjectStores<'a>>,
}

impl<'a> RequestDatabaseReturnsBuilder<'a> {
    /// Database with an array of object stores.
    pub fn databaseWithObjectStores(mut self, databaseWithObjectStores: DatabaseWithObjectStores<'a>) -> Self { self.databaseWithObjectStores = Some(databaseWithObjectStores); self }
    pub fn build(self) -> RequestDatabaseReturns<'a> {
        RequestDatabaseReturns {
            databaseWithObjectStores: self.databaseWithObjectStores.unwrap_or_default(),
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
    pub fn builder() -> RequestDatabaseNamesParamsBuilder<'a> { RequestDatabaseNamesParamsBuilder::default() }
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
    pub fn builder() -> RequestDatabaseNamesReturnsBuilder<'a> { RequestDatabaseNamesReturnsBuilder::default() }
    pub fn databaseNames(&self) -> &[Cow<'a, str>] { &self.databaseNames }
}

#[derive(Default)]
pub struct RequestDatabaseNamesReturnsBuilder<'a> {
    databaseNames: Option<Vec<Cow<'a, str>>>,
}

impl<'a> RequestDatabaseNamesReturnsBuilder<'a> {
    /// Database names for origin.
    pub fn databaseNames(mut self, databaseNames: Vec<Cow<'a, str>>) -> Self { self.databaseNames = Some(databaseNames); self }
    pub fn build(self) -> RequestDatabaseNamesReturns<'a> {
        RequestDatabaseNamesReturns {
            databaseNames: self.databaseNames.unwrap_or_default(),
        }
    }
}

impl<'a> RequestDatabaseNamesParams<'a> { pub const METHOD: &'static str = "IndexedDB.requestDatabaseNames"; }

impl<'a> crate::CdpCommand<'a> for RequestDatabaseNamesParams<'a> {
    const METHOD: &'static str = "IndexedDB.requestDatabaseNames";
    type Response = RequestDatabaseNamesReturns<'a>;
}
