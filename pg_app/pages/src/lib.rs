// pg_app/pages/src/mod.rs
pub mod home;
pub use home::Home;

pub mod blog;
pub use blog::{Blog,BlogList};


pub mod dashboard;
pub use dashboard::Dashboard;


pub mod random;
pub use random::Random;

pub mod not_found;
pub use not_found::NotFound;

pub mod layout;

pub mod routes;

