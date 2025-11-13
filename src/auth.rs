use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
type HmacSha256 = Hmac<Sha256>;

#[allow(dead_code)]
pub fn verify_authorization(
    headers: &HeaderMap,
    method: &str,
    path: &str,
    query: &str,
    body: &[u8],
    secret_key: &str,
) -> bool {
    let auth_header = match headers.get("authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return false,
    };

    if auth_header.starts_with("AWS4-HMAC-SHA256") {
        verify_v4_signature(headers, method, path, query, body, secret_key)
    } else {
        false
    }
}

#[allow(dead_code)]
fn verify_v4_signature(
    headers: &HeaderMap,
    _method: &str,
    _path: &str,
    _query: &str,
    _body: &[u8],
    _secret_key: &str,
) -> bool {
    let auth_header = match headers.get("authorization") {
        Some(h) => match h.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        },
        None => return false,
    };

    // basic validation - just check format for now
    // full signature validation would require canonical request building
    auth_header.starts_with("AWS4-HMAC-SHA256")
}

#[allow(dead_code)]
pub fn extract_access_key(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("authorization")?.to_str().ok()?;

    if auth_header.starts_with("AWS4-HMAC-SHA256") {
        extract_access_key_v4(auth_header)
    } else {
        None
    }
}

#[allow(dead_code)]
fn extract_access_key_v4(auth_header: &str) -> Option<String> {
    // AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request...
    for part in auth_header.split_whitespace() {
        if part.starts_with("Credential=") {
            let credential = part.strip_prefix("Credential=")?;
            let access_key = credential.split('/').next()?;
            return Some(access_key.to_string());
        }
    }
    None
}

#[allow(dead_code)]
pub fn hash_payload(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

#[allow(dead_code)]
pub fn sign(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
