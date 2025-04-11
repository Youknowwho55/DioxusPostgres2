use dioxus::{
    logger::{self, tracing}, prelude::*
};
use server::server_functions::get_user;
static USERS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;

use crate::db::Post; 

#[component]
pub fn Users() -> Element {
    let resource = use_resource(|| async {
        match get_user().await {
            Ok(users) => *POSTS.write() = users,
            Err(err) => tracing::error!("get all user error: {err}"),
        }
    });

    match resource() {
        Some(_) if !USERS().is_empty() => {
            let users_data = USERS();
            let users = users_data.iter().map(|user| {
                rsx! {
                    User { key: "{post.id}", user: user.clone() }
                }
            });
            
            rsx! {
                div { {users} }
            }
        }
        _ => rsx! {
            div { "No User Yet" }
        }
    }
}