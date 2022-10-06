use actix_web::web::ServiceConfig;

pub mod controller;

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(controller::ping);
    cfg.service(controller::pong);
}
