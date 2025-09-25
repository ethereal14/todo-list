use sqlx::MySqlPool;

use crate::errors::TodoListError;
use crate::models::todo_item::*;
use crate::models::todo_list::*;

pub async fn post_new_todo_item_db(
    pool: &MySqlPool,
    new_todo_item: CreateTodoItem,
) -> Result<String, TodoListError> {
    let _insert_query = sqlx::query!(
        "INSERT INTO todo_item (list_id, title, checked) VALUES (?, ?, ?)",
        new_todo_item.list_id,
        new_todo_item.title,
        new_todo_item.checked
    )
    .execute(pool)
    .await?;

    Ok(format!("post new todo_item {:?} success!", _insert_query))
}

pub async fn delete_todo_item_by_id_db(
    pool: &MySqlPool,
    list_id: i32,
    item_id: i32,
) -> Result<String, TodoListError> {
    let rows = sqlx::query!(
        "DELETE FROM todo_item WHERE id = ? AND list_id = ?",
        item_id,
        list_id
    )
    .execute(pool)
    .await
    .map_err(|_err| TodoListError::NotFound("todo_item id not found".into()))?;

    match rows.rows_affected() {
        0 => Err(TodoListError::NotFound("todo_item id not found".into())),
        _ => Ok(format!("deleted {:?} record", rows)),
    }
}

pub async fn update_todo_item_by_id_db(
    pool: &MySqlPool,
    list_id: i32,
    item_id: i32,
    update_todoitem: UpdateTodoItem,
) -> Result<String, TodoListError> {
    let current_todoitem_row: TodoItem =
        sqlx::query_as("SELECT * FROM todo_item WHERE id = ? AND list_id = ?")
            .bind(item_id)
            .bind(list_id)
            .fetch_one(pool)
            .await
            .map_err(|_err| TodoListError::NotFound("Todo list id not found".into()))?;

    let todo_item = TodoItem {
        id: item_id,
        list_id: if let Some(list_id) = update_todoitem.list_id {
            list_id
        } else {
            current_todoitem_row.list_id
        },
        title: if let Some(title) = update_todoitem.title {
            title
        } else {
            current_todoitem_row.title
        },
        checked: if let Some(checked) = update_todoitem.checked {
            checked
        } else {
            current_todoitem_row.checked
        },
    };

    let row = sqlx::query!(
        "UPDATE todo_item SET title = ?, list_id = ?, checked = ? WHERE id = ?",
        todo_item.title,
        todo_item.list_id,
        todo_item.checked,
        item_id
    )
    .execute(pool)
    .await?;

    Ok(format!("update {:?} record", row))
}
