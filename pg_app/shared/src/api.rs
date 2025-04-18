// shared/src/api.rs
use dioxus::prelude::*;
use crate::models::Post;
#[server]
pub async fn create_post(title: String, body: String) -> Result<(), ServerFnError>;

#[server]
pub async fn delete_post(id: i32) -> Result<(), ServerFnError>;

#[server]
pub async fn update_post(id: i32, title: String, body: String) -> Result<Post, ServerFnError>;

#[server]
pub async fn get_all_posts() -> Result<Vec<Post>, ServerFnError> ;


