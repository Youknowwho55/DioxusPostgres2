// pg_app/components/src/db/add_post.rs
use dioxus::prelude::*;
use server::server_functions::create_post;
use shared::models;
use crate::ui::input::{Input, InputType};
use std::sync::Arc;
use crate::ui::button::{Button, ButtonScheme};

#[component]
pub fn AddPost() -> Element {
    let refresh_posts = use_context::<Arc<dyn Fn()>>();
    let mut title = use_signal(String::new);
    let mut body = use_signal(String::new);
    

    // let on_submit = move |_| {
    //     to_owned![title, body, refresh_posts];
    //     spawn(async move {
    //         let current_title = title.read().clone();
    //         let current_body = body.read().clone();

    //         if let Ok(_) = create_post(current_title, current_body).await {
    //             title.set(String::new());
    //             body.set(String::new());
                
    //             // Only call refresh if context is available
    //             refresh_posts();
    //         }
    //     });
    // };

    let on_submit = move |_| {
        to_owned![title, body, refresh_posts];
        spawn(async move {
            let current_title = title.read().clone();
            let current_body = body.read().clone();
            
            // Clear input fields immediately for better UX
            title.set(String::new());
            body.set(String::new());
            
            // Create the post
            match create_post(current_title, current_body).await {
                Ok(_) => {
                    // Refresh the posts list to show the new post
                    refresh_posts();
                }
                Err(err) => {
                    tracing::error!("Failed to save post: {err}");
                    // Show error to user (you might want to add a toast/notification component)
                }
            }
        });
    };
    
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
                on_click: on_submit,
                text: "Add Post".to_string(),
            }
        }
    }
}




