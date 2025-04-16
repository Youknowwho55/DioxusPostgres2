use dioxus::prelude::*;
use server::server_functions::create_post;
use shared::models;
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};

static POSTS: GlobalSignal<Vec<models::Post>> = GlobalSignal::new(Vec::new);

#[component]
pub fn AddPost() -> Element {
    let mut title = use_signal(String::new);
    let mut body = use_signal(String::new);
    
    rsx! {
        div { class: "flex items-center space-x-2 mb-2",
            Input {
                name: "title".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Title".to_string()),
                value: Some(title()),
                label: Some("Post Title".to_string()),
                oninput: move |event: FormEvent| title.set(event.value()),
            }
            Input {
                name: "body".to_string(),
                input_type: Some(InputType::Text),
                placeholder: Some("Body".to_string()),
                value: Some(body()),
                label: Some("Post Body".to_string()),
                oninput: move |event: FormEvent| body.set(event.value()),
            }
            Button {
                button_scheme: ButtonScheme::Success,
                on_click: move |_| {
                    let current_title = title();
                    let current_body = body();
                    let new_post = models::Post {
                        id: 0,
                        title: current_title.clone(),
                        body: current_body.clone(),
                        created_at: None,
                        updated_at: None,
                        is_published: false,
                        author_id: None,
                    };
                    POSTS.write().push(new_post);
                    title.set(String::new());
                    body.set(String::new());
                    spawn(async move {
                        match create_post(current_title, current_body).await {
                            Ok(id) => {
                                POSTS.write().last_mut().unwrap().id = id;
                            }
                            Err(err) => {
                                POSTS.write().pop();
                                tracing::error!("Failed to save post: {err}");
                            }
                        }
                    });
                },
                text: "Add Post".to_string(),
            }
        }
    }
}