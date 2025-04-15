use dioxus::prelude::*;
use components::db::{AddPost, Posts, Post};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize}; // Import enums if needed
use components::ui::Input;
use components::ui::InputType;




/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {

    rsx! {

        div { class: "flex flex-col justify-center pt-2 mt-2 items-center", AddPost {} }
        div { class: " flex flex-col justify-center items-center", Posts {} }
    }
}





