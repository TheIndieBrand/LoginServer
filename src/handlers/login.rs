use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::cookie::CookieJar;
use url::Url;

use crate::{
    cookies::{temp_cookie, REDIRECT_COOKIE, STATE_COOKIE},
    error::{internal, AppError},
    redirect::{resolve_target, RedirectQuery},
    state::Shared,
    util::random_token,
};

pub async fn login(
    State(s): State<Shared>,
    Query(q): Query<RedirectQuery>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    let target = resolve_target(&s.cfg, q.redirect)?;
    let state = random_token();
    let authorize = Url::parse_with_params(
        "https://discord.com/oauth2/authorize",
        &[
            ("client_id", s.cfg.client_id.as_str()),
            ("response_type", "code"),
            ("redirect_uri", s.cfg.redirect_uri.as_str()),
            ("scope", "identify"),
            ("state", state.as_str()),
            ("prompt", "none"),
        ],
    )
    .map_err(internal)?;

    let jar = jar
        .add(temp_cookie(&s.cfg, STATE_COOKIE, state))
        .add(temp_cookie(&s.cfg, REDIRECT_COOKIE, target));
    Ok((jar, Redirect::to(authorize.as_str())))
}
