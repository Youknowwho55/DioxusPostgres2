// pg_app/components/src/db/all_posts.rs
use dioxus::{
    logger::tracing, prelude::*
};
use server::server_functions::get_all_posts;
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;
use crate::db::Post;
use std::sync::Arc;

#[component]
pub fn Posts() -> Element {
    // Create a signal to trigger refreshes
    let refresh_counter = Arc::new(std::sync::Mutex::new(0));
    
    // Single resource with refresh mechanism
    let refresh_counter_clone = Arc::clone(&refresh_counter);
    let resource = use_resource(move || {
        let refresh_counter = Arc::clone(&refresh_counter_clone);
        async move {
            // The dependency on refresh_counter causes a refetch when it changes
            let _ = *refresh_counter.lock().unwrap();
            
            match get_all_posts().await {
                Ok(posts) => *POSTS.write() = posts,
                Err(err) => tracing::error!("get all post error: {err}"),
            }
        }
    });
    
    // Function to trigger a refresh
    let refresh_posts = {
        let refresh_counter = Arc::clone(&refresh_counter);
        move || {
            let mut counter = refresh_counter.lock().unwrap();
            *counter += 1;
        }
    };
    
    // Make refresh_posts available to other components
    provide_context(Arc::new(refresh_posts) as Arc<dyn Fn()>);
    
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