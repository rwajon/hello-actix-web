use actix_web::web;

pub mod users;

// routes
pub fn routes(cfg: &mut web::ServiceConfig) {
    users::routes(cfg);
}
