use dioxus::prelude::*;
use components::db::{AddPost, Posts, PostsContainer};

#[component]
pub fn Home() -> Element {
    // Create a signal to count refreshes
    let mut refresh_count = use_signal(|| 0);

    // Callback for when a post is added
    let on_post_added = move |_| {
        refresh_count.set(refresh_count() + 1);
    };

    rsx! {
        div { class: "flex flex-col justify-center pt-2 mt-2 items-center",
            AddPost { on_post_added }
        }
        div { class: "flex flex-col justify-center items-center", Posts {} }
        div { class: "flex flex-col justify-center items-center",
            PostsContainer { refresh_count }
        }
    }
}