use dioxus::prelude::*;
use server::server_functions::create_post;
use crate::ui::input::{Input, InputType};
use crate::ui::button::{Button, ButtonScheme};

#[component]
pub fn AddPost(
    // Callback for when a post is added successfully
    on_post_added: EventHandler<()>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut body = use_signal(String::new);

    let on_submit = move |_| {
        to_owned![title, body, on_post_added];
        spawn(async move {
            let current_title = title.read().clone();
            let current_body = body.read().clone();
            
            // Don't submit empty posts
            if current_title.trim().is_empty() || current_body.trim().is_empty() {
                return;
            }
            
            // Clear input fields immediately for better UX
            title.set(String::new());
            body.set(String::new());
            
            // Create the post
            match create_post(current_title, current_body).await {
                Ok(_) => {
                    // Trigger refresh in parent component
                    on_post_added.call(());
                }
                Err(err) => {
                    tracing::error!("Failed to save post: {err}");
                }
            }
        });
    };

    rsx! {
        div { class: "bg-gray-50 p-4 rounded border",
            h2 { class: "text-lg font-medium mb-3", "Add New Post" }
            div { class: "space-y-3",
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
                div { class: "mt-4",
                    Button {
                        button_scheme: ButtonScheme::Success,
                        on_click: on_submit,
                        text: "Add Post".to_string(),
                    }
                }
            }
        }
    }
}