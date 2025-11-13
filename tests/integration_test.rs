use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use bytes::Bytes;
use six7::{config::Config, handlers::AppState, storage::Storage};
use std::sync::Arc;
use tempfile::{NamedTempFile, TempDir};
use tower::ServiceExt;
use std::io::Write;

fn create_test_app() -> Router {
    let temp_dir = TempDir::new().unwrap();
    let yaml_content = format!(
        r#"
server:
  host: "127.0.0.1"
  port: 9000

storage:
  path: "{}"

buckets:
  - name: "test-bucket"
    access_key: "minioadmin"
    secret_key: "minioadmin"
    region: "us-east-1"
"#,
        temp_dir.path().display()
    );

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = Config::load(temp_file.path()).unwrap();
    let storage = Storage::new(&config.storage.path).unwrap();

    let state = Arc::new(AppState { storage });

    use axum::routing::{delete, get, put};
    use tower_http::cors::{Any, CorsLayer};

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(six7::handlers::list_buckets))
        .route("/{bucket}", put(six7::handlers::create_bucket))
        .route("/{bucket}/{*key}", put(six7::handlers::put_object))
        .route("/{bucket}/{*key}", get(six7::handlers::get_object))
        .route("/{bucket}/{*key}", delete(six7::handlers::delete_object))
        .layer(cors)
        .with_state(state)
}

#[tokio::test]
async fn test_list_buckets() {
    let app = create_test_app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_bucket() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/new-bucket")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_put_and_get_object() {
    let app = create_test_app();

    // Create bucket first
    let app_clone = app.clone();
    let response = app_clone
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/test-bucket")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Put object
    let app_clone = app.clone();
    let data = Bytes::from("hello world");
    let response = app_clone
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/test-bucket/test.txt")
                .header("content-type", "text/plain")
                .body(Body::from(data.clone()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Get object
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/test-bucket/test.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_object() {
    let app = create_test_app();

    // Create bucket
    let app_clone = app.clone();
    let _ = app_clone
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/test-bucket")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Put object
    let app_clone = app.clone();
    let data = Bytes::from("test data");
    let _ = app_clone
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/test-bucket/file.txt")
                .body(Body::from(data))
                .unwrap(),
        )
        .await
        .unwrap();

    // Delete object
    let app_clone = app.clone();
    let response = app_clone
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/test-bucket/file.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/test-bucket/file.txt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
