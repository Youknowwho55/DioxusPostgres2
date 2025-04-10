use crate::components::Echo;
use dioxus::prelude::*;
use components::db::{AddPost, Posts};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize, ButtonType}; // Import enums if needed


/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };
    rsx! {
        div { class: "flex flex-col justify-center items-center", AddPost {} }
        div { class: " flex flex-col justify-center items-center", Posts {} }
        Button {
            button_scheme: ButtonScheme::Success,
            button_size: ButtonSize::Large,
            on_click: handle_click,
            "Success Button"
        }
        Echo {}
    }
}





