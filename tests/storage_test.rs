use bytes::Bytes;
use six7::storage::Storage;
use tempfile::TempDir;

#[tokio::test]
async fn test_create_bucket() {
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();

    storage.create_bucket("test-bucket").await.unwrap();
    assert!(storage.bucket_exists("test-bucket").await);
}

#[tokio::test]
async fn test_put_and_get_object() {
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();

    storage.create_bucket("test-bucket").await.unwrap();

    let data = Bytes::from("hello world");
    let metadata = storage
        .put_object("test-bucket", "test.txt", data.clone(), Some("text/plain".to_string()))
        .await
        .unwrap();

    assert_eq!(metadata.key, "test.txt");
    assert_eq!(metadata.size, 11);
    assert!(metadata.content_type.is_some());

    let retrieved = storage.get_object("test-bucket", "test.txt").await.unwrap();
    assert_eq!(retrieved, data);
}

#[tokio::test]
async fn test_delete_object() {
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();

    storage.create_bucket("test-bucket").await.unwrap();

    let data = Bytes::from("hello world");
    storage
        .put_object("test-bucket", "test.txt", data, None)
        .await
        .unwrap();

    assert!(storage.get_object("test-bucket", "test.txt").await.is_ok());

    storage.delete_object("test-bucket", "test.txt").await.unwrap();

    assert!(storage.get_object("test-bucket", "test.txt").await.is_err());
}

#[tokio::test]
async fn test_list_objects() {
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();

    storage.create_bucket("test-bucket").await.unwrap();

    let data = Bytes::from("test");
    storage
        .put_object("test-bucket", "file1.txt", data.clone(), None)
        .await
        .unwrap();
    storage
        .put_object("test-bucket", "file2.txt", data.clone(), None)
        .await
        .unwrap();
    storage
        .put_object("test-bucket", "dir/file3.txt", data.clone(), None)
        .await
        .unwrap();

    let objects = storage.list_objects("test-bucket", None).await.unwrap();
    assert_eq!(objects.len(), 3);

    let objects = storage.list_objects("test-bucket", Some("dir/")).await.unwrap();
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].key, "dir/file3.txt");
}

#[tokio::test]
async fn test_head_object() {
    let temp_dir = TempDir::new().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();

    storage.create_bucket("test-bucket").await.unwrap();

    let data = Bytes::from("hello world");
    storage
        .put_object("test-bucket", "test.txt", data, None)
        .await
        .unwrap();

    let metadata = storage.head_object("test-bucket", "test.txt").await.unwrap();
    assert_eq!(metadata.key, "test.txt");
    assert_eq!(metadata.size, 11);
}
