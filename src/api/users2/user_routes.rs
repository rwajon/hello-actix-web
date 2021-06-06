use actix_web::{web, HttpResponse};

fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(|| HttpResponse::Ok())));
}
