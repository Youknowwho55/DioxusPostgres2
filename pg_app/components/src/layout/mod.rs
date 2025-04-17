// mod.rs
pub mod notes;
pub mod footer;
pub mod navbar;
// Re-export from button module
pub use footer::Footer;
pub use navbar::Navbar;