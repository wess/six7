use six7::config::Config;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_load_config() {
    let yaml_content = r#"
server:
  host: "127.0.0.1"
  port: 9000

storage:
  path: "./test-data"

buckets:
  - name: "test-bucket"
    access_key: "test-key"
    secret_key: "test-secret"
    region: "us-east-1"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = Config::load(temp_file.path()).unwrap();

    assert_eq!(config.server.host, "127.0.0.1");
    assert_eq!(config.server.port, 9000);
    assert_eq!(config.storage.path, "./test-data");
    assert_eq!(config.buckets.len(), 1);
    assert_eq!(config.buckets[0].name, "test-bucket");
    assert_eq!(config.buckets[0].access_key, "test-key");
    assert_eq!(config.buckets[0].secret_key, "test-secret");
    assert_eq!(config.buckets[0].region, "us-east-1");
}

#[test]
fn test_get_bucket() {
    let yaml_content = r#"
server:
  host: "127.0.0.1"
  port: 9000

storage:
  path: "./test-data"

buckets:
  - name: "bucket-one"
    access_key: "key1"
    secret_key: "secret1"
    region: "us-east-1"
  - name: "bucket-two"
    access_key: "key2"
    secret_key: "secret2"
    region: "us-west-2"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = Config::load(temp_file.path()).unwrap();

    let bucket = config.get_bucket("bucket-one");
    assert!(bucket.is_some());
    assert_eq!(bucket.unwrap().name, "bucket-one");

    let bucket = config.get_bucket("bucket-two");
    assert!(bucket.is_some());
    assert_eq!(bucket.unwrap().name, "bucket-two");

    let bucket = config.get_bucket("nonexistent");
    assert!(bucket.is_none());
}

#[test]
fn test_find_bucket_by_access_key() {
    let yaml_content = r#"
server:
  host: "127.0.0.1"
  port: 9000

storage:
  path: "./test-data"

buckets:
  - name: "bucket-one"
    access_key: "key1"
    secret_key: "secret1"
    region: "us-east-1"
  - name: "bucket-two"
    access_key: "key2"
    secret_key: "secret2"
    region: "us-west-2"
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = Config::load(temp_file.path()).unwrap();

    let bucket = config.find_bucket_by_access_key("key1");
    assert!(bucket.is_some());
    assert_eq!(bucket.unwrap().name, "bucket-one");

    let bucket = config.find_bucket_by_access_key("key2");
    assert!(bucket.is_some());
    assert_eq!(bucket.unwrap().name, "bucket-two");

    let bucket = config.find_bucket_by_access_key("nonexistent");
    assert!(bucket.is_none());
}
