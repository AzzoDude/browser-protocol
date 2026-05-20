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
    #[serde(rename = "objectStores")]
    object_stores: Vec<ObjectStore<'a>>,
}

impl<'a> DatabaseWithObjectStores<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Database name.
    /// * `version`: Database version (type is not 'integer', as the standard requires the version number to be 'unsigned long long')
    /// * `object_stores`: Object stores in this database.
    pub fn builder(name: impl Into<Cow<'a, str>>, version: f64, object_stores: Vec<ObjectStore<'a>>) -> DatabaseWithObjectStoresBuilder<'a> {
        DatabaseWithObjectStoresBuilder {
            name: name.into(),
            version: version,
            object_stores: object_stores,
        }
    }
    /// Database name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Database version (type is not 'integer', as the standard
    /// requires the version number to be 'unsigned long long')
    pub fn version(&self) -> f64 { self.version }
    /// Object stores in this database.
    pub fn object_stores(&self) -> &[ObjectStore<'a>] { &self.object_stores }
}


pub struct DatabaseWithObjectStoresBuilder<'a> {
    name: Cow<'a, str>,
    version: f64,
    object_stores: Vec<ObjectStore<'a>>,
}

impl<'a> DatabaseWithObjectStoresBuilder<'a> {
    pub fn build(self) -> DatabaseWithObjectStores<'a> {
        DatabaseWithObjectStores {
            name: self.name,
            version: self.version,
            object_stores: self.object_stores,
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
    #[serde(rename = "keyPath")]
    key_path: KeyPath<'a>,
    /// If true, object store has auto increment flag set.
    #[serde(rename = "autoIncrement")]
    auto_increment: bool,
    /// Indexes in this object store.
    indexes: Vec<ObjectStoreIndex<'a>>,
}

impl<'a> ObjectStore<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Object store name.
    /// * `key_path`: Object store key path.
    /// * `auto_increment`: If true, object store has auto increment flag set.
    /// * `indexes`: Indexes in this object store.
    pub fn builder(name: impl Into<Cow<'a, str>>, key_path: KeyPath<'a>, auto_increment: bool, indexes: Vec<ObjectStoreIndex<'a>>) -> ObjectStoreBuilder<'a> {
        ObjectStoreBuilder {
            name: name.into(),
            key_path: key_path,
            auto_increment: auto_increment,
            indexes: indexes,
        }
    }
    /// Object store name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Object store key path.
    pub fn key_path(&self) -> &KeyPath<'a> { &self.key_path }
    /// If true, object store has auto increment flag set.
    pub fn auto_increment(&self) -> bool { self.auto_increment }
    /// Indexes in this object store.
    pub fn indexes(&self) -> &[ObjectStoreIndex<'a>] { &self.indexes }
}


pub struct ObjectStoreBuilder<'a> {
    name: Cow<'a, str>,
    key_path: KeyPath<'a>,
    auto_increment: bool,
    indexes: Vec<ObjectStoreIndex<'a>>,
}

impl<'a> ObjectStoreBuilder<'a> {
    pub fn build(self) -> ObjectStore<'a> {
        ObjectStore {
            name: self.name,
            key_path: self.key_path,
            auto_increment: self.auto_increment,
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
    #[serde(rename = "keyPath")]
    key_path: KeyPath<'a>,
    /// If true, index is unique.
    unique: bool,
    /// If true, index allows multiple entries for a key.
    #[serde(rename = "multiEntry")]
    multi_entry: bool,
}

impl<'a> ObjectStoreIndex<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Index name.
    /// * `key_path`: Index key path.
    /// * `unique`: If true, index is unique.
    /// * `multi_entry`: If true, index allows multiple entries for a key.
    pub fn builder(name: impl Into<Cow<'a, str>>, key_path: KeyPath<'a>, unique: bool, multi_entry: bool) -> ObjectStoreIndexBuilder<'a> {
        ObjectStoreIndexBuilder {
            name: name.into(),
            key_path: key_path,
            unique: unique,
            multi_entry: multi_entry,
        }
    }
    /// Index name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Index key path.
    pub fn key_path(&self) -> &KeyPath<'a> { &self.key_path }
    /// If true, index is unique.
    pub fn unique(&self) -> bool { self.unique }
    /// If true, index allows multiple entries for a key.
    pub fn multi_entry(&self) -> bool { self.multi_entry }
}


pub struct ObjectStoreIndexBuilder<'a> {
    name: Cow<'a, str>,
    key_path: KeyPath<'a>,
    unique: bool,
    multi_entry: bool,
}

impl<'a> ObjectStoreIndexBuilder<'a> {
    pub fn build(self) -> ObjectStoreIndex<'a> {
        ObjectStoreIndex {
            name: self.name,
            key_path: self.key_path,
            unique: self.unique,
            multi_entry: self.multi_entry,
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Key type.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> KeyBuilder<'a> {
        KeyBuilder {
            type_: type_.into(),
            number: None,
            string: None,
            date: None,
            array: None,
        }
    }
    /// Key type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Number value.
    pub fn number(&self) -> Option<f64> { self.number }
    /// String value.
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    /// Date value.
    pub fn date(&self) -> Option<f64> { self.date }
    /// Array value.
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
    #[serde(rename = "lowerOpen")]
    lower_open: bool,
    /// If true upper bound is open.
    #[serde(rename = "upperOpen")]
    upper_open: bool,
}

impl<'a> KeyRange<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `lower_open`: If true lower bound is open.
    /// * `upper_open`: If true upper bound is open.
    pub fn builder(lower_open: bool, upper_open: bool) -> KeyRangeBuilder<'a> {
        KeyRangeBuilder {
            lower: None,
            upper: None,
            lower_open: lower_open,
            upper_open: upper_open,
        }
    }
    /// Lower bound.
    pub fn lower(&self) -> Option<&Key<'a>> { self.lower.as_ref() }
    /// Upper bound.
    pub fn upper(&self) -> Option<&Key<'a>> { self.upper.as_ref() }
    /// If true lower bound is open.
    pub fn lower_open(&self) -> bool { self.lower_open }
    /// If true upper bound is open.
    pub fn upper_open(&self) -> bool { self.upper_open }
}


pub struct KeyRangeBuilder<'a> {
    lower: Option<Key<'a>>,
    upper: Option<Key<'a>>,
    lower_open: bool,
    upper_open: bool,
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
            lower_open: self.lower_open,
            upper_open: self.upper_open,
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
    #[serde(rename = "primaryKey")]
    primary_key: crate::runtime::RemoteObject,
    /// Value object.
    value: crate::runtime::RemoteObject,
}

impl DataEntry {
    /// Creates a builder for this type with the required parameters:
    /// * `key`: Key object.
    /// * `primary_key`: Primary key object.
    /// * `value`: Value object.
    pub fn builder(key: crate::runtime::RemoteObject, primary_key: crate::runtime::RemoteObject, value: crate::runtime::RemoteObject) -> DataEntryBuilder {
        DataEntryBuilder {
            key: key,
            primary_key: primary_key,
            value: value,
        }
    }
    /// Key object.
    pub fn key(&self) -> &crate::runtime::RemoteObject { &self.key }
    /// Primary key object.
    pub fn primary_key(&self) -> &crate::runtime::RemoteObject { &self.primary_key }
    /// Value object.
    pub fn value(&self) -> &crate::runtime::RemoteObject { &self.value }
}


pub struct DataEntryBuilder {
    key: crate::runtime::RemoteObject,
    primary_key: crate::runtime::RemoteObject,
    value: crate::runtime::RemoteObject,
}

impl DataEntryBuilder {
    pub fn build(self) -> DataEntry {
        DataEntry {
            key: self.key,
            primary_key: self.primary_key,
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
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Key path type.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> KeyPathBuilder<'a> {
        KeyPathBuilder {
            type_: type_.into(),
            string: None,
            array: None,
        }
    }
    /// Key path type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// String value.
    pub fn string(&self) -> Option<&str> { self.string.as_deref() }
    /// Array value.
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    object_store_name: Cow<'a, str>,
}

impl<'a> ClearObjectStoreParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: Database name.
    /// * `object_store_name`: Object store name.
    pub fn builder(database_name: impl Into<Cow<'a, str>>, object_store_name: impl Into<Cow<'a, str>>) -> ClearObjectStoreParamsBuilder<'a> {
        ClearObjectStoreParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// Database name.
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
    /// Object store name.
    pub fn object_store_name(&self) -> &str { self.object_store_name.as_ref() }
}


pub struct ClearObjectStoreParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
    object_store_name: Cow<'a, str>,
}

impl<'a> ClearObjectStoreParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> ClearObjectStoreParams<'a> {
        ClearObjectStoreParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
            object_store_name: self.object_store_name,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
}

impl<'a> DeleteDatabaseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: Database name.
    pub fn builder(database_name: impl Into<Cow<'a, str>>) -> DeleteDatabaseParamsBuilder<'a> {
        DeleteDatabaseParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// Database name.
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
}


pub struct DeleteDatabaseParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
}

impl<'a> DeleteDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> DeleteDatabaseParams<'a> {
        DeleteDatabaseParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
    #[serde(rename = "objectStoreName")]
    object_store_name: Cow<'a, str>,
    /// Range of entry keys to delete
    #[serde(rename = "keyRange")]
    key_range: KeyRange<'a>,
}

impl<'a> DeleteObjectStoreEntriesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: 
    /// * `object_store_name`: 
    /// * `key_range`: Range of entry keys to delete
    pub fn builder(database_name: impl Into<Cow<'a, str>>, object_store_name: impl Into<Cow<'a, str>>, key_range: KeyRange<'a>) -> DeleteObjectStoreEntriesParamsBuilder<'a> {
        DeleteObjectStoreEntriesParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            key_range: key_range,
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
    pub fn object_store_name(&self) -> &str { self.object_store_name.as_ref() }
    /// Range of entry keys to delete
    pub fn key_range(&self) -> &KeyRange<'a> { &self.key_range }
}


pub struct DeleteObjectStoreEntriesParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
    object_store_name: Cow<'a, str>,
    key_range: KeyRange<'a>,
}

impl<'a> DeleteObjectStoreEntriesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> DeleteObjectStoreEntriesParams<'a> {
        DeleteObjectStoreEntriesParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
            object_store_name: self.object_store_name,
            key_range: self.key_range,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    object_store_name: Cow<'a, str>,
    /// Index name. If not specified, it performs an object store data request.
    #[serde(skip_serializing_if = "Option::is_none", rename = "indexName")]
    index_name: Option<Cow<'a, str>>,
    /// Number of records to skip.
    #[serde(rename = "skipCount")]
    skip_count: u64,
    /// Number of records to fetch.
    #[serde(rename = "pageSize")]
    page_size: u64,
    /// Key range.
    #[serde(skip_serializing_if = "Option::is_none", rename = "keyRange")]
    key_range: Option<KeyRange<'a>>,
}

impl<'a> RequestDataParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: Database name.
    /// * `object_store_name`: Object store name.
    /// * `skip_count`: Number of records to skip.
    /// * `page_size`: Number of records to fetch.
    pub fn builder(database_name: impl Into<Cow<'a, str>>, object_store_name: impl Into<Cow<'a, str>>, skip_count: u64, page_size: u64) -> RequestDataParamsBuilder<'a> {
        RequestDataParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            index_name: None,
            skip_count: skip_count,
            page_size: page_size,
            key_range: None,
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// Database name.
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
    /// Object store name.
    pub fn object_store_name(&self) -> &str { self.object_store_name.as_ref() }
    /// Index name. If not specified, it performs an object store data request.
    pub fn index_name(&self) -> Option<&str> { self.index_name.as_deref() }
    /// Number of records to skip.
    pub fn skip_count(&self) -> u64 { self.skip_count }
    /// Number of records to fetch.
    pub fn page_size(&self) -> u64 { self.page_size }
    /// Key range.
    pub fn key_range(&self) -> Option<&KeyRange<'a>> { self.key_range.as_ref() }
}


pub struct RequestDataParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
    object_store_name: Cow<'a, str>,
    index_name: Option<Cow<'a, str>>,
    skip_count: u64,
    page_size: u64,
    key_range: Option<KeyRange<'a>>,
}

impl<'a> RequestDataParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    /// Index name. If not specified, it performs an object store data request.
    pub fn index_name(mut self, index_name: impl Into<Cow<'a, str>>) -> Self { self.index_name = Some(index_name.into()); self }
    /// Key range.
    pub fn key_range(mut self, key_range: KeyRange<'a>) -> Self { self.key_range = Some(key_range); self }
    pub fn build(self) -> RequestDataParams<'a> {
        RequestDataParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
            object_store_name: self.object_store_name,
            index_name: self.index_name,
            skip_count: self.skip_count,
            page_size: self.page_size,
            key_range: self.key_range,
        }
    }
}

/// Requests data from object store or index.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDataReturns {
    /// Array of object store data entries.
    #[serde(rename = "objectStoreDataEntries")]
    object_store_data_entries: Vec<DataEntry>,
    /// If true, there are more entries to fetch in the given range.
    #[serde(rename = "hasMore")]
    has_more: bool,
}

impl RequestDataReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `object_store_data_entries`: Array of object store data entries.
    /// * `has_more`: If true, there are more entries to fetch in the given range.
    pub fn builder(object_store_data_entries: Vec<DataEntry>, has_more: bool) -> RequestDataReturnsBuilder {
        RequestDataReturnsBuilder {
            object_store_data_entries: object_store_data_entries,
            has_more: has_more,
        }
    }
    /// Array of object store data entries.
    pub fn object_store_data_entries(&self) -> &[DataEntry] { &self.object_store_data_entries }
    /// If true, there are more entries to fetch in the given range.
    pub fn has_more(&self) -> bool { self.has_more }
}


pub struct RequestDataReturnsBuilder {
    object_store_data_entries: Vec<DataEntry>,
    has_more: bool,
}

impl RequestDataReturnsBuilder {
    pub fn build(self) -> RequestDataReturns {
        RequestDataReturns {
            object_store_data_entries: self.object_store_data_entries,
            has_more: self.has_more,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
    /// Object store name.
    #[serde(rename = "objectStoreName")]
    object_store_name: Cow<'a, str>,
}

impl<'a> GetMetadataParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: Database name.
    /// * `object_store_name`: Object store name.
    pub fn builder(database_name: impl Into<Cow<'a, str>>, object_store_name: impl Into<Cow<'a, str>>) -> GetMetadataParamsBuilder<'a> {
        GetMetadataParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// Database name.
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
    /// Object store name.
    pub fn object_store_name(&self) -> &str { self.object_store_name.as_ref() }
}


pub struct GetMetadataParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
    object_store_name: Cow<'a, str>,
}

impl<'a> GetMetadataParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> GetMetadataParams<'a> {
        GetMetadataParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
            object_store_name: self.object_store_name,
        }
    }
}

/// Gets metadata of an object store.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMetadataReturns {
    /// the entries count
    #[serde(rename = "entriesCount")]
    entries_count: f64,
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.
    #[serde(rename = "keyGeneratorValue")]
    key_generator_value: f64,
}

impl GetMetadataReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `entries_count`: the entries count
    /// * `key_generator_value`: the current value of key generator, to become the next inserted key into the object store. Valid if objectStore.autoIncrement is true.
    pub fn builder(entries_count: f64, key_generator_value: f64) -> GetMetadataReturnsBuilder {
        GetMetadataReturnsBuilder {
            entries_count: entries_count,
            key_generator_value: key_generator_value,
        }
    }
    /// the entries count
    pub fn entries_count(&self) -> f64 { self.entries_count }
    /// the current value of key generator, to become the next inserted
    /// key into the object store. Valid if objectStore.autoIncrement
    /// is true.
    pub fn key_generator_value(&self) -> f64 { self.key_generator_value }
}


pub struct GetMetadataReturnsBuilder {
    entries_count: f64,
    key_generator_value: f64,
}

impl GetMetadataReturnsBuilder {
    pub fn build(self) -> GetMetadataReturns {
        GetMetadataReturns {
            entries_count: self.entries_count,
            key_generator_value: self.key_generator_value,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    /// Database name.
    #[serde(rename = "databaseName")]
    database_name: Cow<'a, str>,
}

impl<'a> RequestDatabaseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_name`: Database name.
    pub fn builder(database_name: impl Into<Cow<'a, str>>) -> RequestDatabaseParamsBuilder<'a> {
        RequestDatabaseParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            database_name: database_name.into(),
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
    /// Database name.
    pub fn database_name(&self) -> &str { self.database_name.as_ref() }
}


pub struct RequestDatabaseParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
    database_name: Cow<'a, str>,
}

impl<'a> RequestDatabaseParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> RequestDatabaseParams<'a> {
        RequestDatabaseParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
            database_name: self.database_name,
        }
    }
}

/// Requests database with given name in given frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseReturns<'a> {
    /// Database with an array of object stores.
    #[serde(rename = "databaseWithObjectStores")]
    database_with_object_stores: DatabaseWithObjectStores<'a>,
}

impl<'a> RequestDatabaseReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_with_object_stores`: Database with an array of object stores.
    pub fn builder(database_with_object_stores: DatabaseWithObjectStores<'a>) -> RequestDatabaseReturnsBuilder<'a> {
        RequestDatabaseReturnsBuilder {
            database_with_object_stores: database_with_object_stores,
        }
    }
    /// Database with an array of object stores.
    pub fn database_with_object_stores(&self) -> &DatabaseWithObjectStores<'a> { &self.database_with_object_stores }
}


pub struct RequestDatabaseReturnsBuilder<'a> {
    database_with_object_stores: DatabaseWithObjectStores<'a>,
}

impl<'a> RequestDatabaseReturnsBuilder<'a> {
    pub fn build(self) -> RequestDatabaseReturns<'a> {
        RequestDatabaseReturns {
            database_with_object_stores: self.database_with_object_stores,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "securityOrigin")]
    security_origin: Option<Cow<'a, str>>,
    /// Storage key.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageKey")]
    storage_key: Option<Cow<'a, str>>,
    /// Storage bucket. If not specified, it uses the default bucket.
    #[serde(skip_serializing_if = "Option::is_none", rename = "storageBucket")]
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestDatabaseNamesParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> RequestDatabaseNamesParamsBuilder<'a> {
        RequestDatabaseNamesParamsBuilder {
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(&self) -> Option<&str> { self.security_origin.as_deref() }
    /// Storage key.
    pub fn storage_key(&self) -> Option<&str> { self.storage_key.as_deref() }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(&self) -> Option<&crate::storage::StorageBucket<'a>> { self.storage_bucket.as_ref() }
}

#[derive(Default)]
pub struct RequestDatabaseNamesParamsBuilder<'a> {
    security_origin: Option<Cow<'a, str>>,
    storage_key: Option<Cow<'a, str>>,
    storage_bucket: Option<crate::storage::StorageBucket<'a>>,
}

impl<'a> RequestDatabaseNamesParamsBuilder<'a> {
    /// At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    /// Security origin.
    pub fn security_origin(mut self, security_origin: impl Into<Cow<'a, str>>) -> Self { self.security_origin = Some(security_origin.into()); self }
    /// Storage key.
    pub fn storage_key(mut self, storage_key: impl Into<Cow<'a, str>>) -> Self { self.storage_key = Some(storage_key.into()); self }
    /// Storage bucket. If not specified, it uses the default bucket.
    pub fn storage_bucket(mut self, storage_bucket: crate::storage::StorageBucket<'a>) -> Self { self.storage_bucket = Some(storage_bucket); self }
    pub fn build(self) -> RequestDatabaseNamesParams<'a> {
        RequestDatabaseNamesParams {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            storage_bucket: self.storage_bucket,
        }
    }
}

/// Requests database names for given security origin.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestDatabaseNamesReturns<'a> {
    /// Database names for origin.
    #[serde(rename = "databaseNames")]
    database_names: Vec<Cow<'a, str>>,
}

impl<'a> RequestDatabaseNamesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `database_names`: Database names for origin.
    pub fn builder(database_names: Vec<Cow<'a, str>>) -> RequestDatabaseNamesReturnsBuilder<'a> {
        RequestDatabaseNamesReturnsBuilder {
            database_names: database_names,
        }
    }
    /// Database names for origin.
    pub fn database_names(&self) -> &[Cow<'a, str>] { &self.database_names }
}


pub struct RequestDatabaseNamesReturnsBuilder<'a> {
    database_names: Vec<Cow<'a, str>>,
}

impl<'a> RequestDatabaseNamesReturnsBuilder<'a> {
    pub fn build(self) -> RequestDatabaseNamesReturns<'a> {
        RequestDatabaseNamesReturns {
            database_names: self.database_names,
        }
    }
}

impl<'a> RequestDatabaseNamesParams<'a> { pub const METHOD: &'static str = "IndexedDB.requestDatabaseNames"; }

impl<'a> crate::CdpCommand<'a> for RequestDatabaseNamesParams<'a> {
    const METHOD: &'static str = "IndexedDB.requestDatabaseNames";
    type Response = RequestDatabaseNamesReturns<'a>;
}
