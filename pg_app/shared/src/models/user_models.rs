use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use sqlx::{Type, FromRow};
use std::collections::HashSet;
use crate::validation::{validate_username, hash_password, verify_password};
use super::role_models::{Permission, UserRole};



// ===== User Model =====

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow, Validate)]
pub struct User {
    pub id: i32,
    #[validate(custom(function = "validate_username"))]
    pub username: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "First name must be between 1-100 characters"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Last name must be between 1-100 characters"
    ))]
    pub last_name: String,
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub last_login: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
}

impl User {
    /// Creates a new user with validation and password hashing
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        role: UserRole,
    ) -> Result<Self, ValidationErrors> {
        // Validate username first
        validate_username(&username).map_err(|e| {
            let mut errors = ValidationErrors::new();
            errors.add("username", e);
            errors
        })?;
        
        // Hash the password
        let password_hash = hash_password(&password).map_err(|e| {
            let mut errors = ValidationErrors::new();
            errors.add("password", e);  // e is already a ValidationError
            errors
        })?;
        
        let user = Self {
            id: 0,
            username,
            first_name,
            last_name,
            email,
            password_hash,
            role,
            is_active: true,
            created_at: None,
            last_login: None,
            failed_login_attempts: 0,
        };

        user.validate()?;
        Ok(user)
    }

    /// Returns the user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Checks if the user is allowed to log in
    pub fn can_login(&self) -> bool {
        self.is_active && self.failed_login_attempts < 5
    }
    
    /// Verifies if the provided password matches the stored hash
    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(password, &self.password_hash)
    }
    
    /// Checks if the user has a specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.role.has_permission(permission)
    }
    
    /// Reset failed login attempts counter
    pub fn reset_login_attempts(&mut self) {
        self.failed_login_attempts = 0;
    }
    
    /// Increment failed login attempts counter
    pub fn increment_failed_login(&mut self) {
        self.failed_login_attempts += 1;
    }
}

