use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    discord::DiscordUser,
    error::{internal, AppError},
    util::now,
};

pub const SESSION_DAYS: i64 = 7;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub avatar: Option<String>,
    pub iat: i64,
    pub exp: i64,
}

pub fn sign(secret: &str, user: &DiscordUser) -> Result<String, AppError> {
    let now = now();
    let claims = Claims {
        sub: user.id.clone(),
        name: user.global_name.clone().unwrap_or_else(|| user.username.clone()),
        avatar: user.avatar.clone(),
        iat: now,
        exp: now + SESSION_DAYS * 86400,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(internal)
}

pub fn verify(secret: &str, token: &str) -> Option<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .ok()
        .map(|d| d.claims)
}
