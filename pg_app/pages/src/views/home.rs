use dioxus::prelude::*;
use components::db::{AddPost, Posts};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize}; // Import enums if needed
use components::ui::Input;
use components::ui::InputType;




/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };
    rsx! {

        div { class: "flex flex-col justify-center pt-2 mt-2 items-center", AddPost {} }
        div { class: " flex flex-col justify-center items-center", Posts {} }
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
            input_type: Some(InputType::File), // Passing the enum variant here
            label: Some("Upload".to_string()),
        }
    }
}





