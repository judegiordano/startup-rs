use axum::{
    routing::{get, post},
    Router,
};

pub mod controller;

pub fn router() -> Router {
    Router::new().nest(
        "/dev",
        Router::new()
            .route("/ping", post(controller::ping))
            .route("/pong/:id", get(controller::pong)),
    )
}
