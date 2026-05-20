use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct File<'a> {
    name: Cow<'a, str>,
    /// Timestamp
    #[serde(rename = "lastModified")]
    last_modified: crate::network::TimeSinceEpoch,
    /// Size in bytes
    size: f64,
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
}

impl<'a> File<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `last_modified`: Timestamp
    /// * `size`: Size in bytes
    /// * `type_`: 
    pub fn builder(name: impl Into<Cow<'a, str>>, last_modified: crate::network::TimeSinceEpoch, size: f64, type_: impl Into<Cow<'a, str>>) -> FileBuilder<'a> {
        FileBuilder {
            name: name.into(),
            last_modified: last_modified,
            size: size,
            type_: type_.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Timestamp
    pub fn last_modified(&self) -> &crate::network::TimeSinceEpoch { &self.last_modified }
    /// Size in bytes
    pub fn size(&self) -> f64 { self.size }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
}


pub struct FileBuilder<'a> {
    name: Cow<'a, str>,
    last_modified: crate::network::TimeSinceEpoch,
    size: f64,
    type_: Cow<'a, str>,
}

impl<'a> FileBuilder<'a> {
    pub fn build(self) -> File<'a> {
        File {
            name: self.name,
            last_modified: self.last_modified,
            size: self.size,
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Directory<'a> {
    name: Cow<'a, str>,
    #[serde(rename = "nestedDirectories")]
    nested_directories: Vec<Cow<'a, str>>,
    /// Files that are directly nested under this directory.
    #[serde(rename = "nestedFiles")]
    nested_files: Vec<File<'a>>,
}

impl<'a> Directory<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    /// * `nested_directories`: 
    /// * `nested_files`: Files that are directly nested under this directory.
    pub fn builder(name: impl Into<Cow<'a, str>>, nested_directories: Vec<Cow<'a, str>>, nested_files: Vec<File<'a>>) -> DirectoryBuilder<'a> {
        DirectoryBuilder {
            name: name.into(),
            nested_directories: nested_directories,
            nested_files: nested_files,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    pub fn nested_directories(&self) -> &[Cow<'a, str>] { &self.nested_directories }
    /// Files that are directly nested under this directory.
    pub fn nested_files(&self) -> &[File<'a>] { &self.nested_files }
}


pub struct DirectoryBuilder<'a> {
    name: Cow<'a, str>,
    nested_directories: Vec<Cow<'a, str>>,
    nested_files: Vec<File<'a>>,
}

impl<'a> DirectoryBuilder<'a> {
    pub fn build(self) -> Directory<'a> {
        Directory {
            name: self.name,
            nested_directories: self.nested_directories,
            nested_files: self.nested_files,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BucketFileSystemLocator<'a> {
    /// Storage key
    #[serde(rename = "storageKey")]
    storage_key: crate::storage::SerializedStorageKey<'a>,
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (<https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets>)
    #[serde(skip_serializing_if = "Option::is_none", rename = "bucketName")]
    bucket_name: Option<Cow<'a, str>>,
    /// Path to the directory using each path component as an array item.
    #[serde(rename = "pathComponents")]
    path_components: Vec<Cow<'a, str>>,
}

impl<'a> BucketFileSystemLocator<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `storage_key`: Storage key
    /// * `path_components`: Path to the directory using each path component as an array item.
    pub fn builder(storage_key: crate::storage::SerializedStorageKey<'a>, path_components: Vec<Cow<'a, str>>) -> BucketFileSystemLocatorBuilder<'a> {
        BucketFileSystemLocatorBuilder {
            storage_key: storage_key,
            bucket_name: None,
            path_components: path_components,
        }
    }
    /// Storage key
    pub fn storage_key(&self) -> &crate::storage::SerializedStorageKey<'a> { &self.storage_key }
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (<https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets>)
    pub fn bucket_name(&self) -> Option<&str> { self.bucket_name.as_deref() }
    /// Path to the directory using each path component as an array item.
    pub fn path_components(&self) -> &[Cow<'a, str>] { &self.path_components }
}


pub struct BucketFileSystemLocatorBuilder<'a> {
    storage_key: crate::storage::SerializedStorageKey<'a>,
    bucket_name: Option<Cow<'a, str>>,
    path_components: Vec<Cow<'a, str>>,
}

impl<'a> BucketFileSystemLocatorBuilder<'a> {
    /// Bucket name. Not passing a 'bucketName' will retrieve the default Bucket. (<https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets>)
    pub fn bucket_name(mut self, bucket_name: impl Into<Cow<'a, str>>) -> Self { self.bucket_name = Some(bucket_name.into()); self }
    pub fn build(self) -> BucketFileSystemLocator<'a> {
        BucketFileSystemLocator {
            storage_key: self.storage_key,
            bucket_name: self.bucket_name,
            path_components: self.path_components,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDirectoryParams<'a> {
    #[serde(rename = "bucketFileSystemLocator")]
    bucket_file_system_locator: BucketFileSystemLocator<'a>,
}

impl<'a> GetDirectoryParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `bucket_file_system_locator`: 
    pub fn builder(bucket_file_system_locator: BucketFileSystemLocator<'a>) -> GetDirectoryParamsBuilder<'a> {
        GetDirectoryParamsBuilder {
            bucket_file_system_locator: bucket_file_system_locator,
        }
    }
    pub fn bucket_file_system_locator(&self) -> &BucketFileSystemLocator<'a> { &self.bucket_file_system_locator }
}


pub struct GetDirectoryParamsBuilder<'a> {
    bucket_file_system_locator: BucketFileSystemLocator<'a>,
}

impl<'a> GetDirectoryParamsBuilder<'a> {
    pub fn build(self) -> GetDirectoryParams<'a> {
        GetDirectoryParams {
            bucket_file_system_locator: self.bucket_file_system_locator,
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
    /// Creates a builder for this type with the required parameters:
    /// * `directory`: Returns the directory object at the path.
    pub fn builder(directory: Directory<'a>) -> GetDirectoryReturnsBuilder<'a> {
        GetDirectoryReturnsBuilder {
            directory: directory,
        }
    }
    /// Returns the directory object at the path.
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
