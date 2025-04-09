use dioxus::{
    logger::{self, tracing}, prelude::*
};
use server::server_functions::{delete_post, update_post};
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;

#[component]
pub fn Post(post: models::Post) -> Element {
    let mut title = use_signal(|| post.title.clone());
    let mut body = use_signal(|| post.body.clone());

    rsx! {
        div { class: "my-1",

            input {
                class: "border p-2 rounded-sm mx-2",
                name: "title",
                value: "{title}",
                oninput: move |event| title.set(event.value()),
            }
            input {
                class: "border p-2 rounded-sm mx-2",
                name: "body",
                value: "{body}",
                oninput: move |event| body.set(event.value()),
            }
            button {
                class: "border p-1 rounded-sm mx-3",
                onclick: move |_| async move {
                    match update_post(post.id, title(), body()).await {
                        Ok(_) => {
                            if let Some(post) = POSTS.write().iter_mut().find(|t| t.id == post.id) {
                                post.title = title();
                                post.body = body();
                            }
                        }
                        Err(err) => tracing::error!("update posts error: {err}"),
                    }
                },
                "update"
            }
            button {
                class: "border p-1 rounded-sm",
                onclick: move |_| async move {
                    match delete_post(post.id).await {
                        Ok(_) => POSTS.write().retain(|t| t.id != post.id),
                        Err(err) => tracing::error!("delete post error: {err}"),
                    }
                },
                "delete"
            }
        }
    }
}