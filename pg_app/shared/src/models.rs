// pg_app/shared/src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use sqlx::{Type, FromRow};
use std::collections::HashSet;
use crate::validation::{validate_username, hash_password, verify_password};


// ===== Role and Permission Models =====


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, strum::Display, strum::EnumIter, Default)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
/// Represents the role of a user in the system.
pub enum UserRole {
    #[strum(serialize = "Administrator")]
    Admin,
    #[strum(serialize = "Loan Officer")]
    #[default]
    LoanOfficer,
    Processor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumIter)]
pub enum Permission {
    All,
    ViewLoans,
    CreateLoans,
    EditOwnLoans,
    ProcessLoans,
    ManageUsers,
}

impl UserRole {
    /// Returns the set of permissions granted to this role
    pub fn permissions(&self) -> HashSet<Permission> {
        use Permission::*;
        let mut perms = HashSet::new();
        
        match self {
            UserRole::Admin => {
                perms.insert(All);
            },
            UserRole::LoanOfficer => {
                perms.insert(ViewLoans);
                perms.insert(CreateLoans);
                perms.insert(EditOwnLoans);
            },
            UserRole::Processor => {
                perms.insert(ViewLoans);
                perms.insert(ProcessLoans);
            }
        }
        
        perms
    }
    
    /// Checks if the role has a specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission) || self.permissions().contains(&Permission::All)
    }
}
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

// ===== Post Model =====

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Post {
    pub id: i32,
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub is_published: bool,
    // New field to track who created the post
    pub author_id: Option<i32>,
}

impl Post {
    pub fn new(
        title: String,
        body: String,
        author_id: Option<i32>,
    ) -> Result<Self, ValidationErrors> {
        let post = Self {
            id: 0, // Temporary ID before DB insertion
            title,
            body,
            created_at: None,
            updated_at: None,
            is_published: false,
            author_id,
        };
        post.validate()?;
        Ok(post)
    }
    
    /// Checks if the post is published
    pub fn published(&self) -> bool {
        self.is_published
    }
    
    /// Publishes a post
    pub fn publish(&mut self) {
        self.is_published = true;
        self.updated_at = Some(Utc::now());
    }
    
    /// Unpublishes a post
    pub fn unpublish(&mut self) {
        self.is_published = false;
        self.updated_at = Some(Utc::now());
    }
    
    /// Updates the post content
    pub fn update_content(&mut self, title: String, body: String) -> Result<(), ValidationErrors> {
        self.title = title;
        self.body = body;
        self.updated_at = Some(Utc::now());
        self.validate()
    }
    
    /// Determines if a user can edit this post
    pub fn can_edit(&self, user: &User) -> bool {
        // Admins can edit any post
        if user.has_permission(Permission::All) {
            return true;
        }
        
        // Author can edit own post if they have EditOwnLoans permission
        if user.has_permission(Permission::EditOwnLoans) && 
           self.author_id.map_or(false, |id| id == user.id) {
            return true;
        }
        
        false
    }
}