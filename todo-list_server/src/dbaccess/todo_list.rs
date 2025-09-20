use sqlx::MySqlPool;
use std::io::Error;

use crate::models::todo_list::*;

pub async fn get_all_todo_list_db(pool: &MySqlPool) -> Result<Vec<TodoList>, Error> {
    let rows: Vec<TodoList> = sqlx::query_as("SELECT * FROM todo_list")
        .fetch_all(pool)
        .await
        .unwrap();

    Ok(rows)
}
