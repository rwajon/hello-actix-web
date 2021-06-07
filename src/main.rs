use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod api;

async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/welcome", web::get().to(welcome))
            .configure(api::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
