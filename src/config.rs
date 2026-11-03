use std::env;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub jwt_secret: String,
    pub cookie_domain: Option<String>,
    pub cookie_secure: bool,
    pub allowed_suffix: String,
    pub bind_addr: String,
    pub database_path: String,
}

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("falta la variable de entorno {name}"))
}

impl Config {
    pub fn from_env() -> Self {
        let jwt_secret = required("JWT_SECRET");
        assert!(jwt_secret.len() >= 32, "JWT_SECRET debe tener al menos 32 caracteres");

        Config {
            client_id: required("DISCORD_CLIENT_ID"),
            client_secret: required("DISCORD_CLIENT_SECRET"),
            redirect_uri: required("DISCORD_REDIRECT_URI"),
            jwt_secret,
            cookie_domain: env::var("COOKIE_DOMAIN").ok().filter(|d| !d.is_empty()),
            cookie_secure: env::var("COOKIE_SECURE").map(|v| v != "false").unwrap_or(true),
            allowed_suffix: required("ALLOWED_REDIRECT_SUFFIX"),
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into()),
            database_path: env::var("DATABASE_PATH").unwrap_or_else(|_| "login.db".into()),
        }
    }
}
