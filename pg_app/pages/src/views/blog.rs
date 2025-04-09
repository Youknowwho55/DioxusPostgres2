use crate::Route;
use dioxus::prelude::*;

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",
            class: "max-w-3xl mx-auto px-4 py-12 flex flex-col gap-6",

            h1 { class: "text-3xl font-bold text-gray-800", "This is blog #{id}!" }

            p { class: "text-lg text-gray-600 leading-relaxed",
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            div { class: "flex items-center justify-center gap-4 mt-8",

                Link {
                    to: Route::Blog { id: id - 1 },
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition",
                    "← Previous"
                }

                span { class: "text-gray-400", "Page {id}" }

                Link {
                    to: Route::Blog { id: id + 1 },
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition",
                    "Next →"
                }
            }
        }
    }
}
