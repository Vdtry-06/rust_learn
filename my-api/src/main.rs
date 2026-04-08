mod models;
mod db;
mod handlers;
mod routes;

use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL phải có trong .env");

    let pool = PgPool::connect(&database_url).await.expect("Không thể kết nối đến cơ sở dữ liệu");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Migration thất bại");

    let app = routes::user_routes(pool);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server chạy tại http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
