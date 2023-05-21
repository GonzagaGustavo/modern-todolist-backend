use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("Error to get the PORT.");
    let port: u16 = port.parse::<u16>().expect("Error to parse PORT to integer");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(index)
            .service(list)
            .service(add)
            .service(delete)
            .route("/hey", web::get().to(hey))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[derive(Debug, Serialize)]
struct List {
    id: u32,
    content: String,
    created: String,
}
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

    let url: String = env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
    let opts = Opts::from_url(&url).expect("Failed to generate options.");
    let mut conn = Conn::new(opts).expect("Failed to connect to mysql");

    let query = format!(
        "INSERT INTO list (content) VALUES ('{}')",
        json_body.content
    );

    conn.query_drop(query)
        .expect("Failed to insert task on database.");

    HttpResponse::Ok().body("Task inserted successfully")
}

#[get("/list")]
async fn list() -> impl Responder {
    let url: String = env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
    let opts = OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());
    let mut conn = Conn::new(opts.ssl_opts(mysql::SslOpts::default())).expect("Failed to connect to mysql");

    let query = "SELECT * FROM list";

    let list: Vec<List> = conn
        .query_map(query, |(id, content, created)| List {
            content,
            created,
            id,
        })
        .expect("FAILED TO GET the tasks");

    HttpResponse::Ok().json(list)
}

#[delete("/list/{task_id}")]
async fn delete(task_id: web::Path<u32>) -> impl Responder {
    let url: String = env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
    let opts = Opts::from_url(&url).expect("Failed to generate options.");
    let mut conn = Conn::new(opts).expect("Failed to connect to mysql");

    let query = format!("DELETE FROM list WHERE id = {}", task_id);

    conn.query_drop(query).expect("Failed to delete user");

    HttpResponse::Ok().body("Task deleted successfully")
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}
