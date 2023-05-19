use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::*;
use mysql::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8000;

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .route("/hey", web::get().to(hey))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

struct List {
    id: i32,
    content: &'static str,
    created: &'static str,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/add")]
async fn add() -> impl Responder {
    let url = "mysql://root:root@localhost:8889/modern_todolist";
    let conn = Conn::new(url)?;

    HttpResponse::Ok().body("success")
}
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}
