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
    pub fn builder(name: impl Into<Cow<'a, str>>, lastModified: crate::network::TimeSinceEpoch, size: f64, type_: impl Into<Cow<'a, str>>) -> FileBuilder<'a> {
        FileBuilder {
            name: name.into(),
            lastModified: lastModified,
            size: size,
            type_: type_.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn lastModified(&self) -> &crate::network::TimeSinceEpoch { &self.lastModified }
    pub fn size(&self) -> f64 { self.size }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct FileBuilder<'a> {
    name: Cow<'a, str>,
    lastModified: crate::network::TimeSinceEpoch,
    size: f64,
    type_: Cow<'a, str>,
}

impl<'a> FileBuilder<'a> {
    pub fn build(self) -> File<'a> {
        File {
            name: self.name,
            lastModified: self.lastModified,
            size: self.size,
            type_: self.type_,
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
    pub fn builder(name: impl Into<Cow<'a, str>>, nestedDirectories: Vec<Cow<'a, str>>, nestedFiles: Vec<File<'a>>) -> DirectoryBuilder<'a> {
        DirectoryBuilder {
            name: name.into(),
            nestedDirectories: nestedDirectories,
            nestedFiles: nestedFiles,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn nestedDirectories(&self) -> &[Cow<'a, str>] { &self.nestedDirectories }
    pub fn nestedFiles(&self) -> &[File<'a>] { &self.nestedFiles }
}


pub struct DirectoryBuilder<'a> {
    name: Cow<'a, str>,
    nestedDirectories: Vec<Cow<'a, str>>,
    nestedFiles: Vec<File<'a>>,
}

impl<'a> DirectoryBuilder<'a> {
    pub fn build(self) -> Directory<'a> {
        Directory {
            name: self.name,
            nestedDirectories: self.nestedDirectories,
            nestedFiles: self.nestedFiles,
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
    pub fn builder(storageKey: crate::storage::SerializedStorageKey<'a>, pathComponents: Vec<Cow<'a, str>>) -> BucketFileSystemLocatorBuilder<'a> {
        BucketFileSystemLocatorBuilder {
            storageKey: storageKey,
            bucketName: None,
            pathComponents: pathComponents,
        }
    }
    pub fn storageKey(&self) -> &crate::storage::SerializedStorageKey<'a> { &self.storageKey }
    pub fn bucketName(&self) -> Option<&str> { self.bucketName.as_deref() }
    pub fn pathComponents(&self) -> &[Cow<'a, str>] { &self.pathComponents }
}


pub struct BucketFileSystemLocatorBuilder<'a> {
    storageKey: crate::storage::SerializedStorageKey<'a>,
    bucketName: Option<Cow<'a, str>>,
    pathComponents: Vec<Cow<'a, str>>,
}

impl<'a> BucketFileSystemLocatorBuilder<'a> {
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)
    pub fn bucketName(mut self, bucketName: impl Into<Cow<'a, str>>) -> Self { self.bucketName = Some(bucketName.into()); self }
    pub fn build(self) -> BucketFileSystemLocator<'a> {
        BucketFileSystemLocator {
            storageKey: self.storageKey,
            bucketName: self.bucketName,
            pathComponents: self.pathComponents,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryParams<'a> {
    bucketFileSystemLocator: BucketFileSystemLocator<'a>,
}

impl<'a> GetDirectoryParams<'a> {
    pub fn builder(bucketFileSystemLocator: BucketFileSystemLocator<'a>) -> GetDirectoryParamsBuilder<'a> {
        GetDirectoryParamsBuilder {
            bucketFileSystemLocator: bucketFileSystemLocator,
        }
    }
    pub fn bucketFileSystemLocator(&self) -> &BucketFileSystemLocator<'a> { &self.bucketFileSystemLocator }
}


pub struct GetDirectoryParamsBuilder<'a> {
    bucketFileSystemLocator: BucketFileSystemLocator<'a>,
}

impl<'a> GetDirectoryParamsBuilder<'a> {
    pub fn build(self) -> GetDirectoryParams<'a> {
        GetDirectoryParams {
            bucketFileSystemLocator: self.bucketFileSystemLocator,
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
    pub fn builder(directory: Directory<'a>) -> GetDirectoryReturnsBuilder<'a> {
        GetDirectoryReturnsBuilder {
            directory: directory,
        }
    }
    pub fn directory(&self) -> &Directory<'a> { &self.directory }
}


pub struct GetDirectoryReturnsBuilder<'a> {
    directory: Directory<'a>,
}

impl<'a> GetDirectoryReturnsBuilder<'a> {
    pub fn build(self) -> GetDirectoryReturns<'a> {
        GetDirectoryReturns {
            directory: self.directory,
        }
    }
}

impl<'a> GetDirectoryParams<'a> { pub const METHOD: &'static str = "FileSystem.getDirectory"; }

impl<'a> crate::CdpCommand<'a> for GetDirectoryParams<'a> {
    const METHOD: &'static str = "FileSystem.getDirectory";
    type Response = GetDirectoryReturns<'a>;
}
