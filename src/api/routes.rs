use actix_web::web;

pub mod users;

pub fn routes(cfg: &mut web::ServiceConfig) {
    users::routes(cfg);
}
