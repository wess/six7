mod auth;
mod config;
mod handlers;
mod storage;

use axum::{
    routing::{delete, get, head, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use crate::config::Config;
use crate::handlers::AppState;
use crate::storage::Storage;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::load("six7.yaml").expect("failed to load config");
    let storage = Storage::new(&config.storage.path).expect("failed to initialize storage");

    for bucket in &config.buckets {
        let _ = storage.create_bucket(&bucket.name).await;
    }

    let state = Arc::new(AppState { storage });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(handlers::list_buckets))
        .route("/{bucket}",
            put(handlers::create_bucket)
                .head(handlers::head_bucket)
        )
        .route("/{bucket}/{*key}",
            put(handlers::put_object)
                .get(handlers::get_object)
                .delete(handlers::delete_object)
                .head(handlers::head_object)
        )
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");

    tracing::info!("six7 listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}
