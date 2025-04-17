// mod.rs
pub mod notes;
pub mod footer;
pub mod navbar;
pub mod app_layout;
// Re-export from button module
pub use footer::Footer;
pub use navbar::Navbar;
pub use app_layout::AppLayout;