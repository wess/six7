use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::future::Future;
use tokio::fs as async_fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: u64,
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub content_type: Option<String>,
}

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self, std::io::Error> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)?;
        Ok(Storage { base_path })
    }

    pub fn bucket_path(&self, bucket: &str) -> PathBuf {
        self.base_path.join(bucket)
    }

    pub fn object_path(&self, bucket: &str, key: &str) -> PathBuf {
        self.bucket_path(bucket).join(key)
    }

    pub async fn create_bucket(&self, bucket: &str) -> Result<(), std::io::Error> {
        let bucket_path = self.bucket_path(bucket);
        async_fs::create_dir_all(bucket_path).await
    }

    #[allow(dead_code)]
    pub async fn bucket_exists(&self, bucket: &str) -> bool {
        self.bucket_path(bucket).exists()
    }

    pub async fn put_object(
        &self,
        bucket: &str,
        key: &str,
        data: Bytes,
        content_type: Option<String>,
    ) -> Result<ObjectMetadata, std::io::Error> {
        let object_path = self.object_path(bucket, key);

        if let Some(parent) = object_path.parent() {
            async_fs::create_dir_all(parent).await?;
        }

        let mut file = async_fs::File::create(&object_path).await?;
        file.write_all(&data).await?;
        file.flush().await?;

        let metadata = async_fs::metadata(&object_path).await?;
        let etag = format!("{:x}", md5::compute(&data));

        Ok(ObjectMetadata {
            key: key.to_string(),
            size: metadata.len(),
            last_modified: Utc::now(),
            etag,
            content_type,
        })
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Bytes, std::io::Error> {
        let object_path = self.object_path(bucket, key);
        let data = async_fs::read(object_path).await?;
        Ok(Bytes::from(data))
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), std::io::Error> {
        let object_path = self.object_path(bucket, key);
        async_fs::remove_file(object_path).await
    }

    #[allow(dead_code)]
    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<ObjectMetadata>, std::io::Error> {
        let bucket_path = self.bucket_path(bucket);
        let mut objects = Vec::new();

        self.scan_directory(&bucket_path, &bucket_path, prefix, &mut objects)
            .await?;

        Ok(objects)
    }

    #[allow(dead_code)]
    fn scan_directory<'a>(
        &'a self,
        dir: &'a Path,
        base: &'a Path,
        prefix: Option<&'a str>,
        objects: &'a mut Vec<ObjectMetadata>,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + 'a>> {
        Box::pin(async move {
            let mut entries = async_fs::read_dir(dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_file() {
                    let relative_path = path.strip_prefix(base).unwrap();
                    let key = relative_path.to_string_lossy().to_string();

                    if let Some(p) = prefix {
                        if !key.starts_with(p) {
                            continue;
                        }
                    }

                    let data = async_fs::read(&path).await?;
                    let etag = format!("{:x}", md5::compute(&data));

                    objects.push(ObjectMetadata {
                        key,
                        size: metadata.len(),
                        last_modified: metadata
                            .modified()
                            .ok()
                            .and_then(|t| DateTime::from_timestamp(
                                t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                                0
                            ))
                            .unwrap_or_else(Utc::now),
                        etag,
                        content_type: None,
                    });
                } else if metadata.is_dir() {
                    self.scan_directory(&path, base, prefix, objects).await?;
                }
            }

            Ok(())
        })
    }

    #[allow(dead_code)]
    pub async fn head_object(&self, bucket: &str, key: &str) -> Result<ObjectMetadata, std::io::Error> {
        let object_path = self.object_path(bucket, key);
        let metadata = async_fs::metadata(&object_path).await?;
        let data = async_fs::read(&object_path).await?;
        let etag = format!("{:x}", md5::compute(&data));

        Ok(ObjectMetadata {
            key: key.to_string(),
            size: metadata.len(),
            last_modified: metadata
                .modified()
                .ok()
                .and_then(|t| DateTime::from_timestamp(
                    t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0
                ))
                .unwrap_or_else(Utc::now),
            etag,
            content_type: None,
        })
    }
}
