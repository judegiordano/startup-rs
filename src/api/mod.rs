use actix_web::web::{scope, ServiceConfig};

pub mod dev;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/dev").configure(dev::router));
}
