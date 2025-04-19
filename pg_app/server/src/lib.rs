//pg_app/server/src/lib.rs
pub mod post_functions;  // Contains server logic (e.g., handling requests, etc.)
pub mod users;             // Contains user management logic (e.g., authentication, CRUD operations)

pub mod db_connection;
pub use db_connection::{get_db, init_db};