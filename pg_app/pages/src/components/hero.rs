use dioxus::prelude::*;
use assets::Assets;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "flex flex-col items-center justify-center gap-8 py-12 px-4 bg-gradient-to-b from-blue-50 to-white",
            img {
                src: "{Assets::HEADER_SVG}",
                alt: "Hero Header",
                class: "max-w-full h-auto",
            }

            div {
                id: "links",
                class: "flex flex-wrap justify-center gap-4 text-center",

                a {
                    href: "https://dioxuslabs.com/learn/0.6/",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "📚 Learn Dioxus"
                }
                a {
                    href: "https://dioxuslabs.com/awesome",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "🚀 Awesome Dioxus"
                }
                a {
                    href: "https://github.com/dioxus-community/",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "📡 Community Libraries"
                }
                a {
                    href: "https://github.com/DioxusLabs/sdk",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "⚙️ Dioxus Development Kit"
                }
                a {
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "💫 VSCode Extension"
                }
                a {
                    href: "https://discord.gg/XgGxMSkvUM",
                    class: "text-blue-600 hover:underline font-medium transition duration-150",
                    "👋 Community Discord"
                }
            }
        }
    }
}
