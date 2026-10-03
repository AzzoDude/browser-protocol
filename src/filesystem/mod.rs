use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};


#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct File<'a> {
    pub name: Cow<'a, str>,
    /// Timestamp
    #[serde(rename = "lastModified")]
    pub last_modified: crate::network::TimeSinceEpoch,
    /// Size in bytes
    pub size: f64,
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Directory<'a> {
    pub name: Cow<'a, str>,
    #[serde(rename = "nestedDirectories")]
    pub nested_directories: Vec<Cow<'a, str>>,
    /// Files that are directly nested under this directory.
    #[serde(rename = "nestedFiles")]
    pub nested_files: Vec<File<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BucketFileSystemLocator<'a> {
    /// Storage key
    #[serde(rename = "storageKey")]
    pub storage_key: crate::storage::SerializedStorageKey<'a>,
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (<https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets>)
    #[serde(skip_serializing_if = "Option::is_none", rename = "bucketName")]
    pub bucket_name: Option<Cow<'a, str>>,
    /// Path to the directory using each path component as an array item.
    #[serde(rename = "pathComponents")]
    pub path_components: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "FileSystem.getDirectory", response = "GetDirectoryReturns<'a>")]
pub struct GetDirectoryParams<'a> {
    #[serde(rename = "bucketFileSystemLocator")]
    pub bucket_file_system_locator: BucketFileSystemLocator<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryReturns<'a> {
    /// Returns the directory object at the path.
    pub directory: Directory<'a>,
}