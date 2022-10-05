use axum::Router;

pub mod dev;

pub fn routes() -> Router {
    Router::new().merge(dev::router())
}
