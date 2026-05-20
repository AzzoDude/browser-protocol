use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct File<'a> {
    name: Cow<'a, str>,
    /// Timestamp
    lastModified: crate::network::TimeSinceEpoch,
    /// Size in bytes
    size: f64,
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> File<'a> {
    pub fn builder() -> FileBuilder<'a> { FileBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn lastModified(&self) -> &crate::network::TimeSinceEpoch { &self.lastModified }
    pub fn size(&self) -> f64 { self.size }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}

#[derive(Default)]
pub struct FileBuilder<'a> {
    name: Option<Cow<'a, str>>,
    lastModified: Option<crate::network::TimeSinceEpoch>,
    size: Option<f64>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> FileBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Timestamp
    pub fn lastModified(mut self, lastModified: crate::network::TimeSinceEpoch) -> Self { self.lastModified = Some(lastModified); self }
    /// Size in bytes
    pub fn size(mut self, size: f64) -> Self { self.size = Some(size); self }
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> File<'a> {
        File {
            name: self.name.unwrap_or_default(),
            lastModified: self.lastModified.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            type_: self.type_.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Directory<'a> {
    name: Cow<'a, str>,
    nestedDirectories: Vec<Cow<'a, str>>,
    /// Files that are directly nested under this directory.
    nestedFiles: Vec<File<'a>>,
}

impl<'a> Directory<'a> {
    pub fn builder() -> DirectoryBuilder<'a> { DirectoryBuilder::default() }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn nestedDirectories(&self) -> &[Cow<'a, str>] { &self.nestedDirectories }
    pub fn nestedFiles(&self) -> &[File<'a>] { &self.nestedFiles }
}

#[derive(Default)]
pub struct DirectoryBuilder<'a> {
    name: Option<Cow<'a, str>>,
    nestedDirectories: Option<Vec<Cow<'a, str>>>,
    nestedFiles: Option<Vec<File<'a>>>,
}

impl<'a> DirectoryBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    pub fn nestedDirectories(mut self, nestedDirectories: Vec<Cow<'a, str>>) -> Self { self.nestedDirectories = Some(nestedDirectories); self }
    /// Files that are directly nested under this directory.
    pub fn nestedFiles(mut self, nestedFiles: Vec<File<'a>>) -> Self { self.nestedFiles = Some(nestedFiles); self }
    pub fn build(self) -> Directory<'a> {
        Directory {
            name: self.name.unwrap_or_default(),
            nestedDirectories: self.nestedDirectories.unwrap_or_default(),
            nestedFiles: self.nestedFiles.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BucketFileSystemLocator<'a> {
    /// Storage key
    storageKey: crate::storage::SerializedStorageKey<'a>,
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)
    #[serde(skip_serializing_if = "Option::is_none")]
    bucketName: Option<Cow<'a, str>>,
    /// Path to the directory using each path component as an array item.
    pathComponents: Vec<Cow<'a, str>>,
}

impl<'a> BucketFileSystemLocator<'a> {
    pub fn builder() -> BucketFileSystemLocatorBuilder<'a> { BucketFileSystemLocatorBuilder::default() }
    pub fn storageKey(&self) -> &crate::storage::SerializedStorageKey<'a> { &self.storageKey }
    pub fn bucketName(&self) -> Option<&str> { self.bucketName.as_deref() }
    pub fn pathComponents(&self) -> &[Cow<'a, str>] { &self.pathComponents }
}

#[derive(Default)]
pub struct BucketFileSystemLocatorBuilder<'a> {
    storageKey: Option<crate::storage::SerializedStorageKey<'a>>,
    bucketName: Option<Cow<'a, str>>,
    pathComponents: Option<Vec<Cow<'a, str>>>,
}

impl<'a> BucketFileSystemLocatorBuilder<'a> {
    /// Storage key
    pub fn storageKey(mut self, storageKey: crate::storage::SerializedStorageKey<'a>) -> Self { self.storageKey = Some(storageKey); self }
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)
    pub fn bucketName(mut self, bucketName: impl Into<Cow<'a, str>>) -> Self { self.bucketName = Some(bucketName.into()); self }
    /// Path to the directory using each path component as an array item.
    pub fn pathComponents(mut self, pathComponents: Vec<Cow<'a, str>>) -> Self { self.pathComponents = Some(pathComponents); self }
    pub fn build(self) -> BucketFileSystemLocator<'a> {
        BucketFileSystemLocator {
            storageKey: self.storageKey.unwrap_or_default(),
            bucketName: self.bucketName,
            pathComponents: self.pathComponents.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryParams<'a> {
    bucketFileSystemLocator: BucketFileSystemLocator<'a>,
}

impl<'a> GetDirectoryParams<'a> {
    pub fn builder() -> GetDirectoryParamsBuilder<'a> { GetDirectoryParamsBuilder::default() }
    pub fn bucketFileSystemLocator(&self) -> &BucketFileSystemLocator<'a> { &self.bucketFileSystemLocator }
}

#[derive(Default)]
pub struct GetDirectoryParamsBuilder<'a> {
    bucketFileSystemLocator: Option<BucketFileSystemLocator<'a>>,
}

impl<'a> GetDirectoryParamsBuilder<'a> {
    pub fn bucketFileSystemLocator(mut self, bucketFileSystemLocator: BucketFileSystemLocator<'a>) -> Self { self.bucketFileSystemLocator = Some(bucketFileSystemLocator); self }
    pub fn build(self) -> GetDirectoryParams<'a> {
        GetDirectoryParams {
            bucketFileSystemLocator: self.bucketFileSystemLocator.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryReturns<'a> {
    /// Returns the directory object at the path.
    directory: Directory<'a>,
}

impl<'a> GetDirectoryReturns<'a> {
    pub fn builder() -> GetDirectoryReturnsBuilder<'a> { GetDirectoryReturnsBuilder::default() }
    pub fn directory(&self) -> &Directory<'a> { &self.directory }
}

#[derive(Default)]
pub struct GetDirectoryReturnsBuilder<'a> {
    directory: Option<Directory<'a>>,
}

impl<'a> GetDirectoryReturnsBuilder<'a> {
    /// Returns the directory object at the path.
    pub fn directory(mut self, directory: Directory<'a>) -> Self { self.directory = Some(directory); self }
    pub fn build(self) -> GetDirectoryReturns<'a> {
        GetDirectoryReturns {
            directory: self.directory.unwrap_or_default(),
        }
    }
}

impl<'a> GetDirectoryParams<'a> { pub const METHOD: &'static str = "FileSystem.getDirectory"; }

impl<'a> crate::CdpCommand<'a> for GetDirectoryParams<'a> {
    const METHOD: &'static str = "FileSystem.getDirectory";
    type Response = GetDirectoryReturns<'a>;
}
