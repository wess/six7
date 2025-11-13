use axum::http::{HeaderMap, HeaderValue};
use six7::auth::{extract_access_key, hash_payload};

#[test]
fn test_extract_access_key_v4() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static(
            "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, \
            SignedHeaders=host;range;x-amz-date, \
            Signature=fe5f80f77d5fa3beca038a248ff027d0445342fe2855ddc963176630326f1024"
        )
    );

    let access_key = extract_access_key(&headers);
    assert!(access_key.is_some());
    assert_eq!(access_key.unwrap(), "AKIAIOSFODNN7EXAMPLE");
}

#[test]
fn test_extract_access_key_missing() {
    let headers = HeaderMap::new();
    let access_key = extract_access_key(&headers);
    assert!(access_key.is_none());
}

#[test]
fn test_hash_payload() {
    let data = b"hello world";
    let hash = hash_payload(data);
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn test_hash_empty_payload() {
    let data = b"";
    let hash = hash_payload(data);
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
