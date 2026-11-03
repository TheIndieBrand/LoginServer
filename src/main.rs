mod config;
mod cookies;
mod db;
mod discord;
mod error;
mod handlers;
mod jwt;
mod redirect;
mod state;
mod util;

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    config::Config,
    db::Database,
    handlers::{callback::callback, health::health, login::login, logout::logout, me::me},
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = Config::from_env();
    let db = Database::open(&cfg.database_path).expect("no se pudo abrir la base de datos");
    let http = reqwest::Client::builder()
        .user_agent("login-server (https://login.theindiebrand.es, 0.1)")
        .build()
        .expect("cliente http");

    let addr = cfg.bind_addr.clone();
    let state = Arc::new(AppState { cfg, db, http });

    let app = Router::new()
        .route("/login", get(login))
        .route("/auth/discord/callback", get(callback))
        .route("/me", get(me))
        .route("/logout", get(logout))
        .route("/health", get(health))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("no se pudo abrir el puerto");
    tracing::info!("escuchando en {addr}");
    axum::serve(listener, app).await.expect("fallo del servidor");
}
