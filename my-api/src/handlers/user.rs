use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::user::{CreateUserRequest, UpdateUserRequest};
use crate::db;


pub async fn create(State(pool): State<PgPool>, Json(body): Json<CreateUserRequest>,) -> Result<impl axum::response::IntoResponse, StatusCode> {
    db::create_user(&pool, body) 
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<impl axum::response::IntoResponse, StatusCode> {
    db::get_all_users(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_one(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<impl axum::response::IntoResponse, StatusCode> {
    match db::get_user_by_id(&pool, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(State(pool): State<PgPool>, Path(id): Path<i32>, Json(body): Json<UpdateUserRequest>) -> Result<impl axum::response::IntoResponse, StatusCode> {
    match db::update_user(&pool, id, body).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<impl axum::response::IntoResponse, StatusCode> {
    match db::delete_user(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
