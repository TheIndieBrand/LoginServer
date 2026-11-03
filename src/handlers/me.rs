use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;

use crate::{cookies::SESSION_COOKIE, error::AppError, jwt, state::Shared};

#[derive(Serialize)]
pub struct Me {
    id: String,
    name: String,
    avatar_url: Option<String>,
}

fn extract_token(jar: &CookieJar, headers: &HeaderMap) -> Option<String> {
    if let Some(c) = jar.get(SESSION_COOKIE) {
        return Some(c.value().to_owned());
    }
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::to_owned)
}

pub async fn me(
    State(s): State<Shared>,
    jar: CookieJar,
    headers: HeaderMap,
) -> Result<Json<Me>, AppError> {
    let claims = extract_token(&jar, &headers)
        .and_then(|t| jwt::verify(&s.cfg.jwt_secret, &t))
        .ok_or(AppError(StatusCode::UNAUTHORIZED, "no autenticado"))?;
    let avatar_url = claims
        .avatar
        .map(|h| format!("https://cdn.discordapp.com/avatars/{}/{}.png", claims.sub, h));
    Ok(Json(Me {
        id: claims.sub,
        name: claims.name,
        avatar_url,
    }))
}
