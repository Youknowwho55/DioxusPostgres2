// pg_app/components/src/db/all_posts.rs
use dioxus::{
    logger::tracing, prelude::*
};
use server::server_functions::get_all_posts;
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;

use crate::db::Post; 

#[component]
pub fn Posts() -> Element {
    let resource = use_resource(|| async {
        match get_all_posts().await {
            Ok(posts) => *POSTS.write() = posts,
            Err(err) => tracing::error!("get all post error: {err}"),
        }
    });

    match resource() {
        Some(_) if !POSTS().is_empty() => {
            let posts_data = POSTS();
            let posts = posts_data.iter().map(|post| {
                rsx! {
                    Post { key: "{post.id}", post: post.clone() }
                }
            });
            
            rsx! {
                div { {posts} }
            }
        }
        _ => rsx! {
            div { "No Posts Yet" }
        }
    }
}