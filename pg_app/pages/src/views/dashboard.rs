use dioxus::prelude::*;
use components::db::{AddPost, Posts};
use components::ui::Button;
use components::ui::{ButtonScheme, ButtonSize}; // Import enums if needed




/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Dashboard() -> Element {

    rsx! {
        h2 { class: "flex flex-col justify-center items-center", "Good morning user" }
    }
}





