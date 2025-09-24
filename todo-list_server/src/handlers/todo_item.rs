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

// pub async fn delete_todo_item_by_id_db(pool: &MySqlPool, id: i32) -> Result<String, TodoListError> {
//     println!("delete id: {}", id);
//     let rows = sqlx::query!("DELETE FROM todo_item WHERE id = ?", id)
//         .execute(pool)
//         .await
//         .map_err(|_err| TodoListError::NotFound("todo_item id not found".into()))?;

//     match rows.rows_affected() {
//         0 => Err(TodoListError::NotFound("todo_item id not found".into())),
//         _ => Ok(format!("deleted {:?} record", rows)),
//     }
// }

// pub async fn update_todo_item_by_id_db(
//     pool: &MySqlPool,
//     id: i32,
//     update_todoitem: UpdateTodoItem,
// ) -> Result<String, TodoListError> {
//     let current_todoitem_row: TodoItem = sqlx::query_as("SELECT * FROM todo_item WHERE id = ?")
//         .bind(id)
//         .fetch_one(pool)
//         .await
//         .map_err(|_err| TodoListError::NotFound("Todo list id not found".into()))?;

//     let todo_item = TodoItem {
//         id: id,
//         list_id: if let Some(list_id) = update_todoitem.list_id {
//             list_id
//         } else {
//             current_todoitem_row.list_id
//         },
//         title: if let Some(title) = update_todoitem.title {
//             title
//         } else {
//             current_todoitem_row.title
//         },
//         checked: if let Some(checked) = update_todoitem.checked {
//             checked
//         } else {
//             current_todoitem_row.checked
//         },
//     };

//     let row = sqlx::query!(
//         "UPDATE todo_item SET title = ?, list_id = ?, checked = ? WHERE id = ?",
//         todo_item.title,
//         todo_item.list_id,
//         todo_item.checked,
//         id
//     )
//     .execute(pool)
//     .await?;

//     Ok(format!("update {:?} record", row))
// }
