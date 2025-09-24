use crate::errors::TodoListError;

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CreateTodoItem {
    pub list_id: Option<i32>,
    pub title: Option<String>,
    pub checked: Option<bool>,
}

impl TryFrom<web::Json<CreateTodoItem>> for CreateTodoItem {
    type Error = TodoListError;

    fn try_from(value: web::Json<CreateTodoItem>) -> Result<Self, Self::Error> {
        Ok(CreateTodoItem {
            list_id: value.list_id,
            title: value.title.clone(),
            checked: value.checked,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTodoItem {
    pub list_id: Option<i32>,
    pub title: Option<String>,
    pub checked: Option<bool>,
}

impl From<web::Json<UpdateTodoItem>> for UpdateTodoItem {
    fn from(value: web::Json<UpdateTodoItem>) -> Self {
        UpdateTodoItem {
            list_id: value.list_id,
            title: value.title.clone(),
            checked: value.checked,
        }
    }
}
