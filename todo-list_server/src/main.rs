use actix_web::{App, HttpServer, Responder, get};
use std::io::Result;

mod dbaccess;
mod models;
mod handlers;
mod errors;
mod state;

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    println!("Starting server at http://127.0.0.1:8000");
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
