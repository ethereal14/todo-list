use crate::errors::TodoListError;
use crate::models::todo_list::UpdateTodoList;
use crate::state::AppState;
use crate::{dbaccess::todo_list::*, models::todo_list::CreateTodoList};
use actix_web::{HttpResponse, web};

pub async fn get_all_todo_list(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, TodoListError> {
    get_all_todo_list_db(&app_state.db)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn get_todo_list_detail(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, TodoListError> {
    let id = params.into_inner();
    get_todo_list_detail_db(&app_state.db, id)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn post_new_todo_list(
    app_state: web::Data<AppState>,
    new_todo_list: web::Json<CreateTodoList>,
) -> Result<HttpResponse, TodoListError> {
    post_new_todo_list_db(&app_state.db, new_todo_list.try_into()?)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn delete_todo_list_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, TodoListError> {
    let id = params.into_inner();

    delete_todo_list_by_id_db(&app_state.db, id)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn update_todo_list_by_id(
    app_state: web::Data<AppState>,
    update_todolist: web::Json<UpdateTodoList>,
    params: web::Path<i32>,
) -> Result<HttpResponse, TodoListError> {
    let id = params.into_inner();
    update_todo_list_by_id_db(&app_state.db, id, update_todolist.into())
        .await
        .map(|response| HttpResponse::Ok().json(response))
}
