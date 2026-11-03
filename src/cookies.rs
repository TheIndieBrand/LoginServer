use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::{config::Config, jwt::SESSION_DAYS};

pub const SESSION_COOKIE: &str = "session";
pub const STATE_COOKIE: &str = "oauth_state";
pub const REDIRECT_COOKIE: &str = "oauth_redirect";

pub fn session_cookie(cfg: &Config, value: String) -> Cookie<'static> {
    let mut b = Cookie::build((SESSION_COOKIE, value))
        .path("/")
        .http_only(true)
        .secure(cfg.cookie_secure)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(SESSION_DAYS));
    if let Some(d) = &cfg.cookie_domain {
        b = b.domain(d.clone());
    }
    b.build()
}

pub fn session_removal(cfg: &Config) -> Cookie<'static> {
    let mut b = Cookie::build(SESSION_COOKIE).path("/");
    if let Some(d) = &cfg.cookie_domain {
        b = b.domain(d.clone());
    }
    b.build()
}

pub fn temp_cookie(cfg: &Config, name: &'static str, value: String) -> Cookie<'static> {
    Cookie::build((name, value))
        .path("/")
        .http_only(true)
        .secure(cfg.cookie_secure)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::minutes(10))
        .build()
}
