use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn new(id: i32, title: String, body: String ) -> Self {
        Self {
            id,
            title,
            body,
        }
    }
}