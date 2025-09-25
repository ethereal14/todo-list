use crate::dbaccess::todo_item::*;
use crate::dbaccess::todo_list::*;
use crate::errors::TodoListError;
use crate::models::todo_item::*;
use crate::models::todo_list::*;
use crate::state::AppState;
use actix_web::{HttpResponse, web};

pub async fn post_new_todo_item(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    new_todo_item: web::Json<CreateTodoItem>,
) -> Result<HttpResponse, TodoListError> {
    let _list_id = params.into_inner();

    post_new_todo_item_db(&app_state.db, new_todo_item.try_into()?)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn delete_todo_item_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, TodoListError> {
    let (_list_id, _item_id) = params.into_inner();

    delete_todo_item_by_id_db(&app_state.db, _list_id, _item_id)
        .await
        .map(|response| HttpResponse::Ok().json(response))
}

pub async fn update_todo_item_by_id(
    app_state: web::Data<AppState>,
    update_todo_item: web::Json<UpdateTodoItem>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, TodoListError> {
    let (list_id, item_id) = params.into_inner();
    update_todo_item_by_id_db(&app_state.db, list_id, item_id, update_todo_item.into())
        .await
        .map(|response| HttpResponse::Ok().json(response))
}
