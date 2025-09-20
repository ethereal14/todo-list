use crate::dbaccess::todo_list::*;
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use std::io::Error;

pub async fn get_all_todo_list(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    get_all_todo_list_db(&app_state.db)
        .await
        .map(|todo_list| HttpResponse::Ok().json(todo_list))
}
