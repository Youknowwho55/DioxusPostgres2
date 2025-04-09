
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

    sqlx::migrate!("../migrations").run(&connection_pool).await?;
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
pub async fn create_post(title: String, body:String) -> Result<i32, ServerFnError> {
    let db = get_db().await;

    let row = sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1, $2) RETURNING id",
        title,
        body,
    )
    .fetch_one(db)
    .await?;
    
    Ok(row.id)
    
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
pub async fn update_post(id: i32, title: String,  body: String) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!(
        "UPDATE posts SET title = $1, body = $2 WHERE id = $3",
        title,
        body,
        id,
    )
    .execute(db)
    .await?;

    match result.rows_affected() {
        0 => Err(ServerFnError::Request("No rows updated".to_string())),
        _ => Ok(()),
    }
}