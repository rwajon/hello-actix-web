use actix_web::{get, post, web, HttpResponse, Responder};

#[path = "./controller.rs"]
mod controller;

pub fn index(cfg: &mut web::ServiceConfig) {
    #[get("/users")]
    async fn get_all() -> impl Responder {
        HttpResponse::Ok().json(controller::User::get_all().await)
    }

    #[post("/users")]
    async fn add_user(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }

    cfg.service(get_all);
    cfg.service(add_user);
}
