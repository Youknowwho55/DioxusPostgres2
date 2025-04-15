#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    id: String,
    children: Element,
    submit_action: Option<String>,
    class: Option<String>,
    size: Option<ModalSize>,
    open: Signal<bool>,  // Changed from use_signal<bool> to Signal<bool>
    close_on_backdrop: Option<bool>,
    close_button: Option<bool>,
}

#[derive(Clone, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[component]
pub fn Modal(mut props: ModalProps) -> Element {
    let size_class = match props.size.unwrap_or(ModalSize::Medium) {
        ModalSize::Small => "max-w-sm",
        ModalSize::Medium => "max-w-md",
        ModalSize::Large => "max-w-lg",
        ModalSize::ExtraLarge => "max-w-2xl",
    };
    
    let close_on_backdrop = props.close_on_backdrop.unwrap_or(true);
    let close_button = props.close_button.unwrap_or(true);
    let is_open = *props.open.read();  // Correct way to read the signal value
    
    let base_class = "fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center p-4 z-50";
    let display_class = if is_open { "block" } else { "hidden" };
    let class = format!("{} {} {}", base_class, display_class, props.class.clone().unwrap_or_default());
    
    let modal_id = props.id.clone();
    
    let wrapper = if let Some(action) = &props.submit_action {
        rsx! {
            form { action: "{action}", method: "post",
                div {
                    class: "{class}",
                    id: "{props.id}",
                    onclick: move |event| {
                        if close_on_backdrop {
                            #[cfg(target_family = "wasm")]
                            if let Some(element) = event
                                .web_event()
                                .and_then(|e| e.target().dyn_into::<web_sys::Element>().ok())
                            {
                                if element.id() == modal_id {
                                    props.open.set(false);
                                }
                            }
                            #[cfg(not(target_family = "wasm"))]
                            {
                                props.open.set(false);
                            }
                        }
                    },
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl {size_class} w-full relative overflow-hidden",
                        onclick: move |event| {
                            event.stop_propagation();
                        },
                        if close_button {
                            button {
                                class: "absolute top-2 right-2 text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400",
                                r#type: "button",
                                onclick: move |_| {
                                    props.open.set(false);
                                },
                                span { class: "sr-only", "Close" }
                                svg {
                                    class: "h-5 w-5",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                        {props.children}
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "{class}",
                id: "{props.id}",
                onclick: move |event| {
                    if close_on_backdrop {
                        #[cfg(target_family = "wasm")]
                        if let Some(element) = event
                            .web_event()
                            .and_then(|e| e.target().dyn_into::<web_sys::Element>().ok())
                        {
                            if element.id() == modal_id {
                                props.open.set(false);
                            }
                        }
                        #[cfg(not(target_family = "wasm"))]
                        {
                            props.open.set(false);
                        }
                    }
                },
                div {
                    class: "bg-white dark:bg-gray-800 rounded-lg shadow-xl {size_class} w-full relative overflow-hidden",
                    onclick: move |event| {
                        event.stop_propagation();
                    },
                    if close_button {
                        button {
                            class: "absolute top-2 right-2 text-gray-400 hover:text-gray-500 dark:text-gray-500 dark:hover:text-gray-400",
                            r#type: "button",
                            onclick: move |_| {
                                props.open.set(false);
                            },
                            span { class: "sr-only", "Close" }
                            svg {
                                class: "h-5 w-5",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M6 18L18 6M6 6l12 12",
                                }
                            }
                        }
                    }
                    {props.children}
                }
            }
        }
    };
    
    wrapper
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalHeaderProps {
    children: Element,
    class: Option<String>,
}

#[component]
pub fn ModalHeader(props: ModalHeaderProps) -> Element {
    let base_class = "px-6 py-4 border-b border-gray-200 dark:border-gray-700";
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        div { class: "{class}",
            h3 { class: "text-lg font-medium text-gray-900 dark:text-white", {props.children} }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
    padding: Option<bool>,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    let padding = props.padding.unwrap_or(true);
    let base_class = if padding { "p-6" } else { "" };
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        div { class: "{class}", {props.children} }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
    justify: Option<String>,
}

#[component]
pub fn ModalAction(props: ModalActionProps) -> Element {
    let justify = props.justify.clone().unwrap_or("end".to_string());
    let justify_class = match justify.as_str() {
        "start" => "justify-start",
        "center" => "justify-center",
        "between" => "justify-between",
        "around" => "justify-around",
        "evenly" => "justify-evenly",
        _ => "justify-end", // Default
    };
    
    let base_class = format!("px-6 py-4 border-t border-gray-200 dark:border-gray-700 flex gap-2 {}", justify_class);
    let class = format!("{} {}", base_class, props.class.clone().unwrap_or_default());
    
    rsx!(
        div { class: "{class}", {props.children} }
    )
}

// Example usage:
/*
fn app() -> Element {
    let modal_open = use_signal(|| false);
    
    rsx!(
        // Modal trigger button
        button {
            class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
            onclick: move |_| {
                modal_open.set(true);
            },
            "Open Modal"
        }
        
        // Modal component
        Modal {
            id: "example-modal".to_string(),
            size: Some(ModalSize::Medium),
            close_button: Some(true),
            open: modal_open,
            
            ModalHeader {
                "Confirmation"
            }
            
            ModalBody {
                p { "Are you sure you want to perform this action?" }
            }
            
            ModalAction {
                button {
                    class: "px-4 py-2 bg-gray-200 text-gray-800 rounded hover:bg-gray-300",
                    onclick: move |_| {
                        modal_open.set(false);
                    },
                    "Cancel"
                }
                
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                    "Confirm"
                }
            }
        }
    )
}
*/