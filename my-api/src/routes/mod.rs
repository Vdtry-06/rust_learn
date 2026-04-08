use axum::{Router, routing:: get};
use sqlx::PgPool;
use crate::handlers::user as user_handlers;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(user_handlers::get_all).post(user_handlers::create))
        .route("/users/:id", get(user_handlers::get_one).put(user_handlers::update).delete(user_handlers::delete))
        .with_state(pool)
}