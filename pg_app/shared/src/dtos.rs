// shared/src/dtos.rs
use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::{DateTime, Utc};

// For creating new posts
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,
    
    // Optional field for immediate publishing
    #[serde(default)]
    pub publish: bool,
}

// For updating existing posts
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostDto {
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: Option<String>,
    
    pub publish: Option<bool>,
}

// For API responses
#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: String, // Formatted date
    pub updated_at: Option<String>, // Formatted date
    pub is_published: bool,
    pub author: Option<UserBriefResponse>,
}

// Brief user information included in responses
#[derive(Debug, Serialize)]
pub struct UserBriefResponse {
    pub id: i32,
    pub username: String,
    pub full_name: String,
}

// User authentication request
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

// Registration request
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,
    
    #[validate(length(min = 1, max = 100, message = "First name must be 1-100 characters"))]
    pub first_name: String,
    
    #[validate(length(min = 1, max = 100, message = "Last name must be 1-100 characters"))]
    pub last_name: String,
    
    #[validate(email(message = "Must provide a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

// Authentication response containing token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

// Full user response
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub last_login: Option<String>,
}