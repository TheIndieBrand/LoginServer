use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError(pub StatusCode, pub &'static str);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

pub fn upstream<E: Display>(msg: &'static str) -> impl FnOnce(E) -> AppError {
    move |e| {
        tracing::error!("{msg}: {e}");
        AppError(StatusCode::BAD_GATEWAY, msg)
    }
}

pub fn internal<E: Display>(e: E) -> AppError {
    tracing::error!("error interno: {e}");
    AppError(StatusCode::INTERNAL_SERVER_ERROR, "error interno")
}
