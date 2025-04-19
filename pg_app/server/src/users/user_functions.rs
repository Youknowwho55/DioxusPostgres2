use dioxus::prelude::*;
use shared::models::{User, UserRole};
use crate::get_db;
use std::env;



#[server]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn get_user(id: i32) -> Result<User, ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(result)
}

#[server]
pub async fn create_user(
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password_hash: String,
    role: UserRole,    
    is_active: bool,
) -> Result<i32, ServerFnError> {
    let db = get_db().await;



    let row = sqlx::query!(
        r#"
        INSERT INTO users (first_name, last_name, username, email, password_hash, role, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        first_name,
        last_name,
        username,
        email,
        password_hash,
        role as UserRole,
        is_active
    )
    .fetch_one(db)
    .await?;

    Ok(row.id)
}

#[server]
pub async fn delete_user(id: i32) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        Err(ServerFnError::Request("No rows deleted".to_string()))
    } else {
        Ok(())
    }
}

#[server]
pub async fn update_user(
    id: i32,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password_hash: String,
    role: UserRole,
    is_active: bool,
) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET 
            first_name = $1,
            last_name = $2,
            username = $3,
            email = $4,
            password_hash = $5,
            role = $6,
            is_active = $7
        WHERE id = $8
        "#,
        first_name,
        last_name,
        username,
        email,
        password_hash,
        role as UserRole,
        is_active,
        id
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        Err(ServerFnError::Request("No rows updated".to_string()))
    } else {
        Ok(())
    }
}

#[server]
pub async fn update_is_active(id: i32, is_active: bool) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let result = sqlx::query!(
        r#"
        UPDATE users
        SET is_active = $1
        WHERE id = $2
        "#,
        is_active,
        id
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        Err(ServerFnError::Request("No rows updated".to_string()))
    } else {
        Ok(())
    }
}