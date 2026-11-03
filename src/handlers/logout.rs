use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum_extra::extract::cookie::CookieJar;

use crate::{
    cookies::session_removal,
    error::AppError,
    redirect::{resolve_target, RedirectQuery},
    state::Shared,
};

pub async fn logout(
    State(s): State<Shared>,
    Query(q): Query<RedirectQuery>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    let target = resolve_target(&s.cfg, q.redirect)?;
    Ok((jar.remove(session_removal(&s.cfg)), Redirect::to(&target)))
}
