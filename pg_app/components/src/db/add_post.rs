use dioxus::{
    logger::{self, tracing}, 
    prelude::*,
    events::FormEvent,  // Add this import
};
use server::server_functions::create_post;
static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);
use shared::models;
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};

#[component]
pub fn AddPost() -> Element {
    let mut title: Signal<String> = use_signal(String::new);
    let mut body: Signal<String> = use_signal(String::new);
    
    rsx! {
        div { class: " flex items-center space-x-2 mb-2",
            Input {
                name: "title".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Title".to_string()),
                value: Some(title()),
                label: Some("Post Title".to_string()),
                oninput: move |event: FormEvent| title.set(event.value()), // Add the type annotation here
            }
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
                    match create_post(title(), body()).await {
                        Ok(id) => POSTS.write().push(models::Post::new(id, title(), body())),
                        Err(err) => tracing::error!("create post error: {err}"),
                    }
                },
                text: "Add Post".to_string(),
            }
        }
    }
}