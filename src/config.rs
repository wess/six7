use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub buckets: Vec<BucketConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketConfig {
    pub name: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn get_bucket(&self, name: &str) -> Option<&BucketConfig> {
        self.buckets.iter().find(|b| b.name == name)
    }

    #[allow(dead_code)]
    pub fn find_bucket_by_access_key(&self, access_key: &str) -> Option<&BucketConfig> {
        self.buckets.iter().find(|b| b.access_key == access_key)
    }
}
