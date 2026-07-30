//! Post-login redirect resolution.

/// Resolves the post-login redirect against the frontend origin. Only same-origin
/// paths (starting with `/`) are honored to avoid open redirects.
pub(super) fn frontend_target(frontend_url: &str, redirect: Option<&str>) -> String {
    match redirect {
        Some(path) if path.starts_with('/') => {
            format!("{}{}", frontend_url.trim_end_matches('/'), path)
        }
        _ => frontend_url.to_string(),
    }
}
