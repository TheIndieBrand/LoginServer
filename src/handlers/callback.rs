use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;

use crate::{
    cookies::{session_cookie, REDIRECT_COOKIE, STATE_COOKIE},
    discord,
    error::{internal, AppError},
    jwt,
    redirect::validate_redirect,
    state::Shared,
    util::now,
};

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

pub async fn callback(
    State(s): State<Shared>,
    Query(q): Query<CallbackQuery>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    if q.error.is_some() {
        return Err(AppError(StatusCode::BAD_REQUEST, "autorización cancelada"));
    }
    let (Some(code), Some(state)) = (q.code, q.state) else {
        return Err(AppError(StatusCode::BAD_REQUEST, "faltan parámetros"));
    };
    let expected = jar.get(STATE_COOKIE).map(|c| c.value().to_owned());
    if expected.as_deref() != Some(state.as_str()) {
        return Err(AppError(StatusCode::BAD_REQUEST, "state inválido"));
    }
    let target = jar
        .get(REDIRECT_COOKIE)
        .and_then(|c| validate_redirect(&s.cfg, c.value()))
        .ok_or(AppError(StatusCode::BAD_REQUEST, "redirect no válido"))?;

    let access_token = discord::exchange_code(&s.http, &s.cfg, &code).await?;
    let user = discord::fetch_user(&s.http, &access_token).await?;

    s.db
        .upsert_user(&user.id, &user.username, user.avatar.as_deref(), now())
        .map_err(internal)?;

    let token = jwt::sign(&s.cfg.jwt_secret, &user)?;
    let jar = jar
        .remove(Cookie::build(STATE_COOKIE).path("/"))
        .remove(Cookie::build(REDIRECT_COOKIE).path("/"))
        .add(session_cookie(&s.cfg, token));
    Ok((jar, Redirect::to(&target)))
}
