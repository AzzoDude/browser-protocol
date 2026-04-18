use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct File {

    pub name: String,
    /// Timestamp

    pub lastModified: crate::network::TimeSinceEpoch,
    /// Size in bytes

    pub size: f64,

    #[serde(rename = "type")]
    pub type_: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Directory {

    pub name: String,

    pub nestedDirectories: Vec<String>,
    /// Files that are directly nested under this directory.

    pub nestedFiles: Vec<File>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BucketFileSystemLocator {
    /// Storage key

    pub storageKey: crate::storage::SerializedStorageKey,
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (<https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets>)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucketName: Option<String>,
    /// Path to the directory using each path component as an array item.

    pub pathComponents: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryParams {

    pub bucketFileSystemLocator: BucketFileSystemLocator,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryReturns {
    /// Returns the directory object at the path.

    pub directory: Directory,
}
