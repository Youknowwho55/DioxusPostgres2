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

pub mod layout;


// pages/src/lib.rs
use shared::{PageRenderer, Route};

pub struct AppPages;

impl PageRenderer for AppPages {
    fn home(cx: Scope) -> Element {
        cx.render(rsx! {
            Home {}
        })
    }
    
    fn blog(cx: Scope, id: i32) -> Element {
        cx.render(rsx! {
            Blog { id }
        })
    }
    
    // ... implement other routes
}