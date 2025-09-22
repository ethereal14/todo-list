use actix_web::{error, http::StatusCode};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TodoListError {
    DataBaseError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct TodoListErrorResponse {
    error_message: String,
}

impl TodoListError {
    fn error_message(&self) -> String {
        match self {
            TodoListError::DataBaseError(msg) => {
                println!("Databse error occurred: {}", msg);
                "Database error".into()
            }
            TodoListError::ActixError(msg) => {
                println!("Server error occurred: {}", msg);
                "Server error".into()
            }
            TodoListError::NotFound(msg) => {
                println!("NotFound error occurred: {}", msg);
                "NotFound error".into()
            }
            TodoListError::InvalidInput(msg) => {
                println!("InvalidInput error occurred: {}", msg);
                "InvalidInput error".into()
            }
        }
    }
}

impl error::ResponseError for TodoListError {
    fn status_code(&self) -> StatusCode {
        match self {
            TodoListError::DataBaseError(_msg) | TodoListError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TodoListError::NotFound(_msg) => StatusCode::NOT_FOUND,
            TodoListError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for TodoListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for TodoListError {
    fn from(value: actix_web::error::Error) -> Self {
        TodoListError::ActixError(value.to_string())
    }
}

impl From<SqlxError> for TodoListError {
    fn from(value: SqlxError) -> Self {
        TodoListError::DataBaseError(value.to_string())
    }
}
