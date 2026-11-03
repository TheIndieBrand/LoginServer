use serde::Deserialize;

use crate::{
    config::Config,
    error::{upstream, AppError},
};

#[derive(Deserialize)]
struct TokenResp {
    access_token: String,
}

#[derive(Deserialize)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

pub async fn exchange_code(
    http: &reqwest::Client,
    cfg: &Config,
    code: &str,
) -> Result<String, AppError> {
    let token: TokenResp = http
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", cfg.client_id.as_str()),
            ("client_secret", cfg.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", cfg.redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(upstream("fallo al contactar con Discord"))?
        .error_for_status()
        .map_err(upstream("Discord rechazó el código"))?
        .json()
        .await
        .map_err(upstream("respuesta de Discord inválida"))?;
    Ok(token.access_token)
}

pub async fn fetch_user(http: &reqwest::Client, access_token: &str) -> Result<DiscordUser, AppError> {
    http.get("https://discord.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(upstream("fallo al obtener el usuario"))?
        .error_for_status()
        .map_err(upstream("Discord rechazó el token"))?
        .json()
        .await
        .map_err(upstream("usuario de Discord inválido"))
}
