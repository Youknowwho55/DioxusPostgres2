use dioxus::prelude::*;
use components::db::{AddPost, Posts, Post};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize};
use components::ui::Input;
use components::ui::InputType;
use components::ui::toast::{ToastInfo, ToastManager}; // Added ToastManager

/// The Random page component
#[component]
pub fn Random() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };
    
    // Get the toast manager from context
    let mut toast_manager = use_context::<Signal<ToastManager>>();

    rsx! {
        Button {
            button_scheme: ButtonScheme::Success,
            button_size: ButtonSize::Large,
            on_click: handle_click,
            "Success Button"
        }
        Input {
            name: "username".to_string(),
            label: Some("Username".to_string()),
            placeholder: Some("Enter your username".to_string()),
            required: Some(true),
        }
        Input {
            name: "file".to_string(),
            input_type: Some(InputType::File),
            label: Some("Upload".to_string()),
        }
        button {
            class: "px-4 py-2 bg-blue-500 text-white rounded",
            onclick: move |_| {
                toast_manager
                    .write()
                    .popup(ToastInfo::success("Operation completed!", Some("Success")));
            },
            "Show Toast Examples"
        }

        LoadingSpinner {
            size: SpinnerSize::Large,
            color: SpinnerColor::Primary,
            variant: SpinnerVariant::Border,
            label: Some("Processing...".to_string()),
        }
        // Simple spinner
        LoadingSpinner {}
        // Bouncing dots variant
        LoadingSpinner { variant: SpinnerVariant::Dots, color: SpinnerColor::Success }
    }
}