use axum::http::StatusCode;
use serde::Deserialize;
use url::Url;

use crate::{config::Config, error::AppError};

#[derive(Deserialize)]
pub struct RedirectQuery {
    pub redirect: Option<String>,
}
pub fn validate_redirect(cfg: &Config, raw: &str) -> Option<String> {
    let url = Url::parse(raw).ok()?;
    if !url.username().is_empty() || url.password().is_some() {
        return None;
    }
    let host = url.host_str()?;
    let suffix = &cfg.allowed_suffix;
    let host_ok = host == suffix || host.ends_with(&format!(".{suffix}"));
    let dev_ok = !cfg.cookie_secure && (host == "localhost" || host == "127.0.0.1");
    let scheme_ok = url.scheme() == "https" || (!cfg.cookie_secure && url.scheme() == "http");
    if (host_ok || dev_ok) && scheme_ok {
        Some(url.to_string())
    } else {
        None
    }
}

pub fn resolve_target(cfg: &Config, redirect: Option<String>) -> Result<String, AppError> {
    match redirect {
        Some(r) => validate_redirect(cfg, &r)
            .ok_or(AppError(StatusCode::BAD_REQUEST, "redirect no permitido")),
        None => Ok(format!("https://{}/", cfg.allowed_suffix)),
    }
}
