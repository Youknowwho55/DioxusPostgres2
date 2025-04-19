// pg_app/components/src/layout/navbar.rs
use crate::routes::Route;
use dioxus::prelude::*;
// pg_app/components/src/ui/avatar_drop.rs
use components::ui::avatar_drop::AvatarDrop;


/// The Navbar component that will be rendered on all pages of our app
#[allow(non_snake_case)]
#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar", class: "w-full bg-gray-900 text-white shadow-md",
            div { class: "max-w-6xl mx-auto px-4 py-2 flex items-center justify-between",
                h1 { class: "text-xl font-semibold", "MyApp" }
                div { class: "flex gap-6 items-center text-sm",
                    Link {
                        to: Route::Home {},
                        class: "hover:text-blue-400 transition",
                        "Home"
                    }
                    Link {
                        to: Route::Blog {},
                        class: "hover:text-blue-400 transition",
                        "Blog"
                    }
                    Link {
                        to: Route::Dashboard {},
                        class: "hover:text-blue-400 transition",
                        "Dashboard"
                    }
                    Link {
                        to: Route::Random {},
                        class: "hover:text-blue-400 transition",
                        "Random"
                    }
                    AvatarDrop {}
                }
            }
        }
    }
}