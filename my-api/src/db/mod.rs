use sqlx::PgPool;
use crate::models::user::{User, CreateUserRequest, UpdateUserRequest};

pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, age) VALUES ($1, $2, $3) RETURNING *",
        req.name,
        req.email,
        req.age
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT * FROM users ORDER BY id")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn update_user(pool: &PgPool, id: i32, req: UpdateUserRequest) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET
            email = COALESCE($1, email),
            age   = COALESCE($2, age)
         WHERE id = $3
         RETURNING *",
        req.email,
        req.age,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}