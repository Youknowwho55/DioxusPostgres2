// pg_app/components/src/db/post.rs
use dioxus::{
    logger:: tracing, prelude::*
};
use crate::ui::input::{Input,InputType};
use crate::ui::button::{Button, ButtonScheme};

use server::server_functions::{delete_post, update_post};
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;

#[component]
pub fn Post(post: models::Post) -> Element {
    let mut title = use_signal(|| post.title.clone());
    let mut body = use_signal(|| post.body.clone());

    rsx! {
        div { class: "my-1 flex flex-row items-center gap-2",

            Input {
                name: "title".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Title".to_string()),
                value: Some(title()),
                label: Some("Post Title".to_string()),
                oninput: move |event: FormEvent| title.set(event.value()), // Add the type annotation here
            }
            // input {
            //     class: "border p-2 rounded-sm mx-2",
            //     name: "title",
            //     value: "{title}",
            //     oninput: move |event| title.set(event.value()),
            // }
            Input {
                name: "body".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Body".to_string()),
                value: Some(body()),
                label: Some("Post Body".to_string()),
                oninput: move |event: FormEvent| body.set(event.value()), // Add the type annotation here
            }


            Button {
                button_scheme: ButtonScheme::Default,
                on_click: move |_| async move {
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

            Button {
                button_scheme: ButtonScheme::Danger,
                on_click: move |_| async move {
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