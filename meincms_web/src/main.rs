mod auth;
mod db;
mod handlers;
mod models;
mod tenant;
mod views;

use std::net::SocketAddr;
use axum::{
    http::{header, HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::DbStore;
use handlers::{
    get_all_articles, get_article, get_edit_article, get_history, get_index, get_login,
    get_logout, get_search, get_version, post_login, post_save_article,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "meincms_web=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_store = DbStore::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/", get(get_index))
        .route("/login", get(get_login).post(post_login))
        .route("/logout", get(get_logout))
        .route("/alle", get(get_all_articles))
        .route("/suche", get(get_search))
        .route("/edit/*slug", get(get_edit_article))
        .route("/save/*slug", post(post_save_article))
        .route("/wiki/*slug", get(get_article))
        .route("/history/*slug", get(get_history))
        .route("/version/:version_num", get(get_version))
        .layer(axum::middleware::map_response(|mut response: axum::response::Response| async move {
            let headers = response.headers_mut();
            headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store, no-cache, must-revalidate"));
            headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));
            headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
            headers.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            response
        }))
        .layer(axum::middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| async move {
            let path = req.uri().path();
            if path.contains("/.") || path.starts_with("/config") || path.contains("gitignore") || path.contains(".env") {
                return (axum::http::StatusCode::FORBIDDEN, "Zugriff verweigert.").into_response();
            }
            next.run(req).await
        }))
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(db_store);

    let port = std::env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");

    tracing::info!("🚀 MeinCMS Rust Web Backend gestartet auf http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
