use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id : i32,
    pub name : String,
    pub email : String,
    pub age : i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name : String,
    pub email : String,
    pub age : i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email : Option<String>,
    pub age: Option<i32>,
}