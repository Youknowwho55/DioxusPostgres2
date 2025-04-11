use dioxus::prelude::*;
use crate::ui::{Button, ButtonScheme, ButtonSize, Input, InputType};
use crate::ui::Avatar; // Make sure Avatar exists and is imported

#[component]
pub fn Header() -> Element {
    let handle_click = move |_| {
        println!("Button clicked!");
    };

    let mut show_dropdown = use_signal(|| false); // create a reactive signal

    let toggle_dropdown = move |_| {
        show_dropdown.set(!show_dropdown());
    };
    rsx!(

        div {
            Button {
                button_scheme: ButtonScheme::Success,
                button_size: ButtonSize::Default,
                on_click: toggle_dropdown,
                "Toggle Dropdown"
            }

            if show_dropdown() {
                div { class: "mt-2 p-4 rounded shadow bg-white dark:bg-gray-700",
                    "This is the dropdown content!"
                }
            }
        }
        div {
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



            Avatar {
                src: "/path/to/avatar.jpg".to_string(),
                alt: Some("User profile".to_string()),
            }

            nav { class: "bg-white border-gray-200 dark:bg-gray-900",
                div { class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",

                    div { class: "flex items-center md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse",


                        div {
                            class: "z-50 hidden my-4 text-base list-none bg-white divide-y divide-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:divide-gray-600",
                            id: "user-dropdown",
                            div { class: "px-4 py-3",
                                span { class: "block text-sm text-gray-900 dark:text-white",
                                    "Bonnie Green"
                                }
                                span { class: "block text-sm text-gray-500 truncate dark:text-gray-400",
                                    "name@flowbite.com"
                                }
                            }
                            ul { class: "py-2",
                                // "aria-labelledby": "user-menu-button",
                                li {
                                    a {
                                        href: "#",
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                        "Dashboard"
                                    }
                                }
                                li {
                                    a {
                                        href: "#",
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                        "Settings"
                                    }
                                }
                                li {
                                    a {
                                        href: "#",
                                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white",
                                        "Sign out"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
