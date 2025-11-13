use axum::{
    body::Body,
    extract::{Path, RawQuery, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::Deserialize;
use std::sync::Arc;

use crate::storage::Storage;

pub struct AppState {
    pub storage: Storage,
}

#[derive(Deserialize, Default, Debug)]
pub struct ListObjectsQuery {
    #[serde(rename = "list-type")]
    pub list_type: Option<String>,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    #[serde(rename = "max-keys")]
    pub max_keys: Option<usize>,
    #[serde(rename = "continuation-token")]
    pub continuation_token: Option<String>,
}

pub async fn list_buckets(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    <Owner>
        <ID>local</ID>
        <DisplayName>local</DisplayName>
    </Owner>
    <Buckets/>
</ListAllMyBucketsResult>"#;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(Body::from(xml))
        .unwrap()
}


pub async fn create_bucket(
    State(state): State<Arc<AppState>>,
    Path(bucket): Path<String>,
) -> Response {
    create_bucket_impl(&state, &bucket).await
}

pub async fn head_bucket(
    State(state): State<Arc<AppState>>,
    Path(bucket): Path<String>,
) -> Response {
    head_bucket_impl(&state, &bucket).await
}

pub async fn list_bucket_objects(
    State(state): State<Arc<AppState>>,
    Path(bucket): Path<String>,
    RawQuery(query): RawQuery,
) -> Response {
    let params = query
        .and_then(|q| serde_urlencoded::from_str(&q).ok())
        .unwrap_or_default();
    list_objects_impl(&state, &bucket, params).await
}

async fn create_bucket_impl(state: &AppState, bucket: &str) -> Response {
    match state.storage.create_bucket(bucket).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn head_bucket_impl(state: &AppState, bucket: &str) -> Response {
    if state.storage.bucket_exists(bucket).await {
        StatusCode::OK.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn list_objects_impl(state: &AppState, bucket: &str, params: ListObjectsQuery) -> Response {
    let prefix = params.prefix.as_deref();
    let objects = match state.storage.list_objects(&bucket, prefix).await {
        Ok(objs) => objs,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    let max_keys = params.max_keys.unwrap_or(1000).min(1000);
    let delimiter = params.delimiter.as_deref();

    let mut filtered_objects = Vec::new();
    let mut common_prefixes = std::collections::HashSet::new();

    for obj in objects {
        if let Some(delim) = delimiter {
            if let Some(prefix_str) = prefix {
                if let Some(remainder) = obj.key.strip_prefix(prefix_str) {
                    if let Some(delim_pos) = remainder.find(delim) {
                        let common_prefix = format!("{}{}{}", prefix_str, &remainder[..delim_pos], delim);
                        common_prefixes.insert(common_prefix);
                        continue;
                    }
                }
            } else if let Some(delim_pos) = obj.key.find(delim) {
                let common_prefix = format!("{}{}", &obj.key[..delim_pos], delim);
                common_prefixes.insert(common_prefix);
                continue;
            }
        }
        filtered_objects.push(obj);
    }

    let is_truncated = filtered_objects.len() > max_keys;
    let objects_to_return: Vec<_> = filtered_objects.into_iter().take(max_keys).collect();

    let mut contents = String::new();
    for obj in &objects_to_return {
        contents.push_str(&format!(
            r#"<Contents>
            <Key>{}</Key>
            <LastModified>{}</LastModified>
            <ETag>"{}"</ETag>
            <Size>{}</Size>
            <StorageClass>STANDARD</StorageClass>
        </Contents>"#,
            xml_escape(&obj.key),
            obj.last_modified.to_rfc3339(),
            obj.etag,
            obj.size
        ));
    }

    let mut common_prefixes_xml = String::new();
    for cp in common_prefixes {
        common_prefixes_xml.push_str(&format!(
            r#"<CommonPrefixes>
            <Prefix>{}</Prefix>
        </CommonPrefixes>"#,
            xml_escape(&cp)
        ));
    }

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    <Name>{}</Name>
    <Prefix>{}</Prefix>
    <MaxKeys>{}</MaxKeys>
    <Delimiter>{}</Delimiter>
    <IsTruncated>{}</IsTruncated>
    {}
    {}
</ListBucketResult>"#,
        xml_escape(&bucket),
        xml_escape(prefix.unwrap_or("")),
        max_keys,
        xml_escape(delimiter.unwrap_or("")),
        is_truncated,
        contents,
        common_prefixes_xml
    );

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(Body::from(xml))
        .unwrap()
}

pub async fn put_object(
    State(state): State<Arc<AppState>>,
    Path((bucket, key)): Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match state.storage.put_object(&bucket, &key, body, content_type).await {
        Ok(metadata) => Response::builder()
            .status(StatusCode::OK)
            .header("ETag", format!("\"{}\"", metadata.etag))
            .body(Body::empty())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap(),
    }
}

pub async fn get_object(
    State(state): State<Arc<AppState>>,
    Path((bucket, key)): Path<(String, String)>,
) -> Response {
    match state.storage.get_object(&bucket, &key).await {
        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/octet-stream")
            .body(Body::from(data))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}

pub async fn head_object(
    State(state): State<Arc<AppState>>,
    Path((bucket, key)): Path<(String, String)>,
) -> Response {
    match state.storage.head_object(&bucket, &key).await {
        Ok(metadata) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", metadata.content_type.as_deref().unwrap_or("application/octet-stream"))
            .header("Content-Length", metadata.size.to_string())
            .header("ETag", format!("\"{}\"", metadata.etag))
            .header("Last-Modified", metadata.last_modified.to_rfc2822())
            .body(Body::empty())
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}

pub async fn delete_object(
    State(state): State<Arc<AppState>>,
    Path((bucket, key)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.storage.delete_object(&bucket, &key).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
