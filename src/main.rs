use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod api;
mod utils;

use api::users::User;

#[get("/")]
async fn hello() -> impl Responder {
    let response = User::new("John Smith".to_string(), 45).await;
    HttpResponse::Ok().json(response)
}

#[post("/users")]
async fn echo(req_body: String) -> impl Responder {
    let body = utils::json::parse(&req_body);
    let (mut name, mut age): (String, i32) = (String::from(""), 0);

    if body.get("name") != None {
        name = body.get("name").unwrap().to_string();
    }
    if body.get("age") != None {
        match body.get("age").unwrap().to_string().parse::<i32>() {
            Ok(n) => age = n,
            Err(_) => age = 0,
        }
    }

    let response = User::new(name, age).await;

    HttpResponse::Ok().json(response)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
        // .configure(api::routes::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
