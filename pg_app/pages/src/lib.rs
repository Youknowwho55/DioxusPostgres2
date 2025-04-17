// pg_app/pages/src/views/mod.rs
mod home;
pub use home::Home;

mod blog;
pub use blog::Blog;


mod dashboard;
pub use dashboard::Dashboard;


mod random;
pub use random::Random;

mod not_found;
pub use not_found::NotFound;
