use sqlx::MySqlPool;
use std::io::Error;

use crate::errors::TodoListError;
use crate::models::todo_item::*;
use crate::models::todo_list::*;

pub async fn get_all_todo_list_db(pool: &MySqlPool) -> Result<Vec<TodoList>, TodoListError> {
    let rows: Vec<TodoList> = sqlx::query_as("SELECT * FROM todo_list")
        .fetch_all(pool)
        .await
        .unwrap();

    Ok(rows)
}

pub async fn get_todo_list_detail_db(
    pool: &MySqlPool,
    id: i32,
) -> Result<(TodoList, Vec<TodoItem>), TodoListError> {
    let list_rows: TodoList = sqlx::query_as("SELECT * FROM todo_list WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let item_rows: Vec<TodoItem> = sqlx::query_as("SELECT * FROM todo_item WHERE list_id = ?")
        .bind(id)
        .fetch_all(pool)
        .await?;

    Ok((list_rows, item_rows))
}

pub async fn post_new_todo_list_db(
    pool: &MySqlPool,
    new_todo_list: CreateTodoList,
) -> Result<String, TodoListError> {
    let _insert_query = sqlx::query!(
        "INSERT INTO todo_list (title) VALUES (?)",
        new_todo_list.title
    )
    .execute(pool)
    .await?;

    Ok(format!("post new todo_list {:?} success!", _insert_query))
}

pub async fn delete_todo_list_by_id_db(pool: &MySqlPool, id: i32) -> Result<String, TodoListError> {
    println!("delete id: {}", id);
    let rows = sqlx::query!("DELETE FROM todo_list WHERE id = ?", id)
        .execute(pool)
        .await
        .map_err(|_err| TodoListError::NotFound("todo_list id not found".into()))?;

    Ok(format!("deleted {:?} record", rows))
}
