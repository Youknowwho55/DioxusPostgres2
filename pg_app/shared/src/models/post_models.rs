// pg_app/shared/src/models/post_models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use super::role_models::Permission;
use super::user_models::User;


// ===== Post Model =====

/// Represents a blog post or content entry in the system
///
/// # Fields
/// - `id`: Unique database identifier (auto-incremented)
/// - `title`: Human-readable title of the post
/// - `body`: Main content in markdown or HTML format  
/// - `created_at`: Timestamp of initial creation
/// - `updated_at`: Timestamp of last modification
/// - `is_published`: Visibility status
/// - `author_id`: User who created the post
///
/// # Validation Rules
/// - Title: 1-100 characters
/// - Body: 1-5000 characters
///
/// # Example
/// ```rust
/// let post = Post {
///     id: 1,
///     title: "My First Post".into(),
///     body: "This is the content...".into(),
///     created_at: Some(Utc::now()),
///     updated_at: None,
///     is_published: true,
///     author_id: Some(123),
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow, Validate)]
pub struct Post {
    /// Unique database identifier (auto-incremented)
    pub id: i32,
    
    /// Human-readable title of the post
    ///
    /// # Validation
    /// - Minimum length: 1 character
    /// - Maximum length: 100 characters
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,
    
    /// Main content in markdown or HTML format
    ///
    /// # Validation  
    /// - Minimum length: 1 character
    /// - Maximum length: 5000 characters
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,
    
    /// Timestamp of when the post was initially created
    ///
    /// # Serialization
    /// Stored as UNIX timestamp in seconds
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    
    /// Timestamp of when the post was last modified  
    ///
    /// # Note
    /// Will be `None` if never updated after creation
    #[serde(default, with = "chrono::serde::ts_seconds_option")] 
    pub updated_at: Option<DateTime<Utc>>,
    
    /// Visibility status of the post
    ///
    /// # Default
    /// `false` (draft state)
    #[serde(default)]
    pub is_published: bool,
    
    /// ID of the user who created this post
    ///
    /// # Note
    /// - `None` indicates anonymous/ghost author  
    /// - Set automatically during creation
    pub author_id: Option<i32>,
}

impl Post {
    pub(super) fn new(
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
    pub(super) fn published(&self) -> bool {
        self.is_published
    }
    
    /// Publishes a post
    pub(super) fn publish(&mut self) {
        self.is_published = true;
        self.updated_at = Some(Utc::now());
    }
    
    /// Unpublishes a post
    pub(super) fn unpublish(&mut self) {
        self.is_published = false;
        self.updated_at = Some(Utc::now());
    }
    
    /// Updates the post content
    pub(super) fn update_content(&mut self, title: String, body: String) -> Result<(), ValidationErrors> {
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