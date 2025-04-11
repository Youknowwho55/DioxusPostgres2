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


use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    LoanOfficer,
    Processor,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub is_active: bool,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        first_name: String,
        password: String,
        last_name: String,
        email: String,
        role: UserRole,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            email,
            password,
            role,
            is_active: true,
        }
    }
}
