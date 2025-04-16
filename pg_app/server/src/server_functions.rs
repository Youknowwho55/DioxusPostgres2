// pg_app/server/src/server_functions.rs
use dioxus::prelude::*;
use shared::models::Post;


use sqlx::PgPool;
use tokio::sync::OnceCell;
use dotenv::dotenv;
use std::env;
use tracing::info;
// use sqlx::migrate::Migrator;

static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok(); // loads from .env
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = PgPool::connect(&database_url).await?;

    sqlx::migrate!("./../migrations").run(&connection_pool).await?;
    Ok(connection_pool)
}

pub async fn get_db() -> &'static PgPool {
    // Initialize the DB pool if not already initialized
    let pool = DB.get_or_init(|| async { init_db().await.expect("Failed to initialize database") }).await;
    info!("Database pool obtained");
    pool
}
 



#[server]
pub async fn get_all_posts() -> Result<Vec<Post>, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as!(Post, "SELECT * FROM posts")
        .fetch_all(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn find_post(id: i32) -> Result<Post, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn create_post(title: String, body: String) -> Result<i32, ServerFnError> {
    let db = get_db().await;
    
    // Input validation
    if title.is_empty() || body.is_empty() {
        return Err(ServerFnError::Request("Title and body cannot be empty".into()));
    }

    sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1, $2) RETURNING id",
        title.trim(),  // Clean inputs
        body.trim()
    )
    .fetch_one(db)
    .await
    .map(|row| row.id)
    .map_err(|e| {
        tracing::error!("Failed to create post: {}", e);
        ServerFnError::ServerError("Failed to create post".into())
    })
}

#[server]
pub async fn delete_post(id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(db)
        .await?;

    match result.rows_affected() {
        0 => Err(ServerFnError::Request("No rows deleted".to_string())),
        _ => Ok(()),
    }
}

#[server]
pub async fn update_post(id: i32, title: String, body: String) -> Result<Post, ServerFnError> {
    let mut tx = get_db().await.begin().await?;
    
    let updated = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2 WHERE id = $3 RETURNING *",
        title,
        body,
        id
    )
    .fetch_one(&mut tx)
    .await?;
    
    tx.commit().await?;
    Ok(updated)
}

// #[server]
// pub async fn get_paginated_posts(
//     page: Option<i64>,
//     per_page: Option<i64>
// ) -> Result<(Vec<Post>, i64), ServerFnError> {
//     let page = page.unwrap_or(1);
//     let per_page = per_page.unwrap_or(10);
//     let offset = (page - 1) * per_page;

//     let db = get_db().await;
    
//     let posts = sqlx::query_as!(
//         Post,
//         "SELECT * FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
//         per_page,
//         offset
//     )
//     .fetch_all(db)
//     .await?;

//     let total: i64 = sqlx::query_scalar!("SELECT count(*) FROM posts")
//         .fetch_one(db)
//         .await?;

//     Ok((posts, total))
// }