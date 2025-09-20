use sqlx::MySqlPool;
use std::io::Error;

use crate::models::todo_item::*;
use crate::models::todo_list::*;

pub async fn get_all_todo_list_db(pool: &MySqlPool) -> Result<Vec<TodoList>, Error> {
    let rows: Vec<TodoList> = sqlx::query_as("SELECT * FROM todo_list")
        .fetch_all(pool)
        .await
        .unwrap();

    Ok(rows)
}

pub async fn get_todo_list_detail_db(
    pool: &MySqlPool,
    id: i32,
) -> Result<(TodoList, Vec<TodoItem>), Error> {
    let list_rows: TodoList = sqlx::query_as("SELECT * FROM todo_list WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();

    let item_rows: Vec<TodoItem> = sqlx::query_as("SELECT * FROM todo_item WHERE list_id = ?")
        .bind(id)
        .fetch_all(pool)
        .await
        .unwrap();

    Ok((list_rows, item_rows))
}
