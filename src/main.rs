use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::prelude::*;
use mysql::*;
use serde_json::Value;

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
    HttpResponse::Ok().body("Modern ToDoList")
}

#[post("/add")]
async fn add(req_body: web::Bytes) -> Result<HttpResponse, std::io::Error> {
    let url: &str = "mysql://root:root@localhost:8889/modern_todolist";
    let conn = Conn::new(url).unwrap();
    let body = String::from_utf8(req_body.to_vec()).unwrap();
    let json_body: Value = serde_json::from_str(&body)?;
    let content = json_body["content"]
        .as_str()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing field 'content'"))
        .unwrap();
    conn.exec_drop(r"INSERT INTO list (content) VALUES (?)", &content)
        .unwrap();

    Ok(HttpResponse::Ok().body("success"))
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

fn get_conn() -> std::result::Result<Conn, Box<dyn std::error::Error>> {
    let url = "mysql://root:root@localhost:8889/modern_todolist";
    return Ok(Conn::new(url)?);
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}
