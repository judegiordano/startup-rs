use axum::{routing::get, Router};

pub mod controller;

pub fn router() -> Router {
    Router::new().nest("/dev", Router::new().route("/ping", get(controller::ping)))
}
