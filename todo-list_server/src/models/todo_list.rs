use std::io;

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct TodoList {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct CreateTodoList {
    pub title: String,
}

impl TryFrom<web::Json<CreateTodoList>> for CreateTodoList {
    type Error = io::Error;

    fn try_from(value: web::Json<CreateTodoList>) -> Result<Self, Self::Error> {
        Ok(CreateTodoList {
            title: value.title.clone(),
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTodoList {
    pub title: String,
}

impl From<web::Json<UpdateTodoList>> for UpdateTodoList {
    fn from(value: web::Json<UpdateTodoList>) -> Self {
        UpdateTodoList {
            title: value.title.clone(),
        }
    }
}
