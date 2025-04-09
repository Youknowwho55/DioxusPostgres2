use dioxus::{
    logger::{self, tracing}, prelude::*
};
use server::server_functions::create_post;
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;

#[component]
pub fn AddPost() -> Element {
    let mut title: Signal<String> = use_signal(String::new);
    let mut body: Signal<String> = use_signal(String::new);

    rsx! {
        div { class: "mb-4",
            input {
                class: "border p-2 rounded-sm mr-2",
                placeholder: "Title",
                value: "{title}",
                oninput: move |event| title.set(event.value()),
            }
            input {
                class: "border p-2 rounded-sm mr-2",
                placeholder: "Body",
                value: "{body}",
                oninput: move |event| body.set(event.value()),
            }

            button {
                class: "border p-1 rounded-sm",
                onclick: move |_| async move {
                    match create_post(title(), body()).await {
                        Ok(id) => POSTS.write().push(models::Post::new(id, title(), body())),
                        Err(err) => tracing::error!("create post error: {err}"),
                    }
                },
                "Add Post"
            }
        }
    }
}