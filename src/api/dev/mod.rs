use actix_web::web::{self, ServiceConfig};

pub mod controller;
pub mod validator;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/ping", web::post().to(controller::ping));
    cfg.route("/pong/{id}", web::get().to(controller::pong));
}
