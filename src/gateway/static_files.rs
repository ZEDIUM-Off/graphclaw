//! Static file serving for the embedded web dashboard and future UI surface.
//!
//! Uses `rust-embed` to bundle the frontend build directories into the binary at compile time.

use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/dist/"]
struct WebAssets;

#[derive(Embed)]
#[folder = "ui/dist/"]
struct UiAssets;

/// Serve static files from `/_app/*` path
pub async fn handle_static(uri: Uri) -> impl IntoResponse {
    let path = uri
        .path()
        .strip_prefix("/_app/")
        .unwrap_or(uri.path())
        .trim_start_matches('/');

    serve_embedded_file::<WebAssets>(path)
}

/// SPA fallback: serve index.html for any non-API, non-static GET request
pub async fn handle_spa_fallback() -> impl IntoResponse {
    serve_embedded_file::<WebAssets>("index.html")
}

/// Serve static files from `/_ui/*`.
pub async fn handle_ui_static(uri: Uri) -> impl IntoResponse {
    let path = uri.path().strip_prefix("/_ui/").unwrap_or(uri.path());
    let normalized = if path.is_empty() { "index.html" } else { path };

    match UiAssets::get(normalized) {
        Some(_) => serve_embedded_file::<UiAssets>(normalized),
        None => serve_embedded_file::<UiAssets>("index.html"),
    }
}

/// SPA fallback for the embedded Vue UI.
pub async fn handle_ui_index() -> impl IntoResponse {
    serve_embedded_file::<UiAssets>("index.html")
}

fn serve_embedded_file<E: Embed>(path: &str) -> impl IntoResponse {
    match E::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path)
                .first_or_octet_stream()
                .to_string();

            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime),
                    (
                        header::CACHE_CONTROL,
                        if path.contains("assets/") {
                            // Hashed filenames — immutable cache
                            "public, max-age=31536000, immutable".to_string()
                        } else {
                            // index.html etc — no cache
                            "no-cache".to_string()
                        },
                    ),
                ],
                content.data.to_vec(),
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}
