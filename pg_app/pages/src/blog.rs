// pages/src/blog.rs
use dioxus::prelude::*;
use super::routes::Route;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div { class: "blog-layout",
            h1 { "Blog" }
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    
    rsx! {
        div {
            p { "This is the blog list" }
        }
    }
    }
