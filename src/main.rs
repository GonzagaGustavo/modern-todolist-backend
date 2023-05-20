use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

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

// struct List {
//     id: u32,
//     content: String,
//     created: String,
// }
#[derive(Deserialize)]
struct InsertList {
    content: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Modern ToDoList")
}

#[post("/add")]
async fn add(json_body: web::Json<InsertList>) -> impl Responder {
    println!("{}", json_body.content);

    let url: &str = "mysql://root:root@localhost:8889/modern_todolist";
    let mut conn = Conn::new(url).expect("Failed to connect to mysql");

    let query = format!(
        "INSERT INTO list (content) VALUES ('{}')",
        json_body.content
    );

    conn.query_drop(query)
        .expect("Failed to insert task on database.");

    HttpResponse::Ok().body("Task inserted successfully")
}

// #[get("/list")]
// async fn list() -> std::result::Result<impl Responder, Box<dyn std::error::Error>> {
//     let conn = get_conn();

//     let list = conn.query_map("SELECT * FROM list", |(id, content, created)| List {
//         id,
//         content,
//         created,
//     })?;

//     HttpResponse::Ok().body(list)
// }

// fn get_conn() -> std::result::Result<Conn, Box<dyn std::error::Error>> {
//     let url = "mysql://root:root@localhost:8889/modern_todolist";
//     return Ok(Conn::new(url)?);
// }

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}
