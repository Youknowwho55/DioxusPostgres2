// pages/src/blog.rs
use dioxus::prelude::*;
use shared::routes::Route;

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
    let posts = use_resource(|| async { get_posts().await });
    
    rsx! {
        div {
            posts.read().map(|posts| rsx! {
                for post in posts {
                    BlogPostCard { key: "{post.id}", post }
                }
            })
        }
    }
}