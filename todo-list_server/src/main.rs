use actix_web::{App, HttpServer, Responder, get, web};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, io::Result, sync::Mutex};

use crate::{routers::*, state::AppState};

mod dbaccess;
mod errors;
mod handlers;
mod models;
mod routers;
mod state;

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPoolOptions::new().connect(&db_url).await.unwrap();

    let app_state = web::Data::new(AppState {
        health_check_response: "I'm OK".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    println!("Starting server at http://127.0.0.1:8000");
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .app_data(app_state.clone())
            .configure(general_router)
            .configure(todo_list_routers)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
