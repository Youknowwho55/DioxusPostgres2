// validators.rs
use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

// Regular expressions used for validating user input.
lazy_static! {
    /// Regex for validating usernames (3-50 alphanumeric characters or underscores).
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{3,50}$").unwrap();
    /// Regex for validating passwords (at least 8 characters).
    pub static ref PASSWORD_REGEX: Regex = Regex::new(r"^.{8,}$").unwrap();
    /// Regex for validating email addresses.
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
}

/// Validates a username to ensure it is 3-50 alphanumeric characters or underscores.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username to validate.
///
/// # Returns
///
/// * `Ok(())` if the username is valid.
/// * `Err(ValidationError)` if the username is invalid.
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if !USERNAME_REGEX.is_match(username) {
        return Err(ValidationError::new(
            "Username must be 3-50 alphanumeric characters or underscores"
        ));
    }
    Ok(())
}

/// Hashes a password to ensure secure storage.
///
/// # Arguments
///
/// * `password` - A string slice that holds the password to hash.
///
/// # Returns
///
/// * `Ok(String)` containing the hashed password if the input is valid.
/// * `Err(ValidationError)` if the password is invalid (e.g., too short).
pub fn hash_password(password: &str) -> Result<String, ValidationError> {
    if !PASSWORD_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must be at least 8 characters"
        ));
    }
    // Choose one implementation:
    Ok(format!("hashed_{}", password)) // Dummy implementation
    // Ok(sha256_hash(password)) // Option 2
    // Ok(bcrypt_hash(password)?) // Option 3
}

/// Verifies if a given password matches the provided hash.
///
/// # Arguments
///
/// * `password` - A string slice that holds the password to verify.
/// * `hash` - A string slice that holds the hashed password to compare against.
///
/// # Returns
///
/// * `true` if the password matches the hash.
/// * `false` otherwise.
pub fn verify_password(password: &str, hash: &str) -> bool {
    // Dummy implementation matching the hash_password function above
    hash == format!("hashed_{}", password)
    // Real implementation would use bcrypt::verify or equivalent
}

/*
// Option 2 helper
fn sha256_hash(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Option 3 helper
fn bcrypt_hash(input: &str) -> Result<String, ValidationError> {
    bcrypt::hash(input, bcrypt::DEFAULT_COST)
        .map_err(|e| ValidationError::new(e.to_string().as_str()))
}
*/