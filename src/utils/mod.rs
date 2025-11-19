/// Returns the backend origin provided at build time via the `BACKEND_URL` env var.
/// Falls back to `http://127.0.0.1:8080` for local development.
pub fn backend_origin() -> &'static str {
    option_env!("BACKEND_URL").unwrap_or("http://127.0.0.1:8080")
}

/// Concatenates the backend origin with the given path (without duplicating slashes).
pub fn api_url(path: &str) -> String {
    let origin = backend_origin().trim_end_matches('/');
    let normalized_path = path.trim_start_matches('/');
    format!("{}/{}", origin, normalized_path)
}
