// pg_app/shared/src/lib.rs
//! Shared types and utilities across the application

#![forbid(unsafe_code)]
#![warn(missing_docs, unreachable_pub)]
/// Module for Data Transfer Objects (DTOs)
pub mod dtos;
/// Module for models
pub mod models;
/// Module for validation utilities and helpers
pub mod validation;
// pub mod routes;

pub mod error;

// Re-exports for convenient access
pub use dtos::*;
pub use validation::*;
pub use error::*;
// pub use routes::*;

pub mod api;


