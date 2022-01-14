use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Extension, Path};
use axum::handler::get;
use axum::http::{HeaderMap, StatusCode};
use axum::Router;
use bytes::Bytes;
use lru::LruCache;
use percent_encoding::percent_decode_str;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::info;

use pb::*;

mod pb;
mod engine;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // 初始化tracing
    tracing_subscriber::fmt::init();
    //构建cache
    let cache = Arc::new(Mutex::new(LruCache::new(16)));
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new()
            .layer(AddExtensionLayer::new(cache))
            .into_inner());
    let addr = "127.0.0.1:8080".parse().unwrap();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>, Extension(cache): Extension<Cache>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = &percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let data = retrieve_image(&url, cache).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    // TODO:处理图片，需要图片引擎
    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, data.to_vec()))
}

async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();
    let lock_cache = &mut cache.lock().await;
    let data = match lock_cache.get(&key) {
        Some(value) => {
            info!("Match cache {}",key);
            value.to_owned()
        }
        None => {
            info!("retrieve image");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            lock_cache.put(key, data.clone());
            data
        }
    };
    Ok(data)
}
