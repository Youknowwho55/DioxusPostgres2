                // toast_manager.write().popup(ToastInfo::simple("Hello world!"));
                // toast_manager
                //     .write()
                //     .popup(ToastInfo::error("Something went wrong", Some("Error")));


#![allow(non_snake_case)]
use std::fmt::Display;
use std::collections::BTreeMap;
use dioxus::prelude::*;


#[derive(Debug, Clone)]
pub struct ID(usize);

impl ID {
    pub fn new() -> Self {
        Self(100000)
    }

    pub fn add(&mut self) -> usize {
        let current = self.0;
        if self.0 == usize::MAX {
            self.0 = 100000;
        } else {
            self.0 += 1;
        }

        current
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}







#[derive(Debug, Clone)]
struct ToastManagerItem {
    info: ToastInfo,
    hide_after: Option<i64>,
}

#[derive(Debug)]
pub struct ToastManager {
    list: BTreeMap<usize, ToastManagerItem>,
    maximum_toast: u8,
    id_manager: ID,
}

impl ToastManager {
    pub fn new(maximum_toast: u8) -> Self {
        Self {
            list: BTreeMap::new(),
            maximum_toast,
            id_manager: ID::new(),
        }
    }

    pub fn popup(&mut self, info: ToastInfo) -> usize {
        let toast_id = self.id_manager.add();

        if self.list.len() >= self.maximum_toast.into() {
            if let Some(result) = self.list.first_key_value() {
                let id = *result.0;
                println!("Deleted Toast ID: {:?}", id);
                self.list.remove(&id);
            }
        }

        let hide_after = info
            .hide_after
            .map(|duration| chrono::Local::now().timestamp() + duration as i64);

        self.list
            .insert(toast_id, ToastManagerItem { info, hide_after });

        toast_id
    }

    pub fn remove(&mut self, id: usize) {
        self.list.remove(&id);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self {
            list: Default::default(),
            maximum_toast: 6,
            id_manager: ID::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ToastScheme {
    Success,
    Warning,
    Error,
    Info,
    Default,
}

impl Default for ToastScheme {
    fn default() -> Self {
        Self::Default
    }
}

impl ToastScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ToastScheme::Success => "bg-green-700 text-green-100 dark:bg-green-600 dark:text-green-50",
            ToastScheme::Warning => "bg-yellow-700 text-yellow-100 dark:bg-yellow-600 dark:text-yellow-50",
            ToastScheme::Error => "bg-red-700 text-red-100 dark:bg-red-600 dark:text-red-50",
            ToastScheme::Info => "bg-blue-700 text-blue-100 dark:bg-blue-600 dark:text-blue-50",
            ToastScheme::Default => "bg-gray-700 text-white dark:bg-gray-600 dark:text-gray-50",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum ToastSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ToastSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Small => "p-2 text-sm",
            Self::Default => "p-3 text-base", 
            Self::Large => "p-4 text-lg",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastInfo {
    pub heading: Option<String>,
    pub context: String,
    pub allow_toast_close: bool,
    pub position: Position,
    pub scheme: ToastScheme,
    pub size: ToastSize,
    pub hide_after: Option<usize>,
}

impl ToastInfo {
    pub fn simple(text: &str) -> Self {
        Self {
            heading: None,
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            scheme: ToastScheme::Default,
            size: ToastSize::Default,
            hide_after: Some(6),
        }
    }

    pub fn success(text: &str, heading: Option<&str>) -> Self {
        Self {
            heading: heading.map(|h| h.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            scheme: ToastScheme::Success,
            size: ToastSize::Default,
            hide_after: Some(6),
        }
    }

    pub fn warning(text: &str, heading: Option<&str>) -> Self {
        Self {
            heading: heading.map(|h| h.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            scheme: ToastScheme::Warning,
            size: ToastSize::Default,
            hide_after: Some(6),
        }
    }

    pub fn info(text: &str, heading: Option<&str>) -> Self {
        Self {
            heading: heading.map(|h| h.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            scheme: ToastScheme::Info,
            size: ToastSize::Default,
            hide_after: Some(6),
        }
    }

    pub fn error(text: &str, heading: Option<&str>) -> Self {
        Self {
            heading: heading.map(|h| h.to_string()),
            context: text.to_string(),
            allow_toast_close: true,
            position: Position::BottomLeft,
            scheme: ToastScheme::Error,
            size: ToastSize::Default,
            hide_after: Some(6),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToastFrameProps {
    manager: Signal<ToastManager>,
}

#[component]
pub fn ToastFrame(props: ToastFrameProps) -> Element {
    let mut manager = props.manager;

    let toast_list = &manager.read().list;

    let mut bottom_left_ele: Vec<VNode> = vec![];
    let mut bottom_right_ele: Vec<VNode> = vec![];
    let mut top_left_ele: Vec<VNode> = vec![];
    let mut top_right_ele: Vec<VNode> = vec![];

    for (id, item) in toast_list.iter() {
        let current_id = *id;

        let position_class = match item.info.position {
            Position::BottomLeft => "bottom-5 left-5",
            Position::BottomRight => "bottom-5 right-5",
            Position::TopLeft => "top-5 left-5",
            Position::TopRight => "top-5 right-5",
        };

        let scheme_class = item.info.scheme.to_string();
        let size_class = item.info.size.to_string();

        let element = rsx! {
            div { class: "fixed {position_class} w-64 rounded-lg shadow-lg pointer-events-auto",
                div { class: "relative {scheme_class} {size_class} rounded-lg",
                    if item.info.allow_toast_close {
                        button {
                            class: "absolute top-2 right-2 text-lg font-bold hover:opacity-80 focus:outline-none",
                            onclick: move |_| {
                                manager.write().list.remove(&current_id);
                            },
                            "×"
                        }
                    }
                    if let Some(v) = &item.info.heading {
                        h2 { class: "font-bold mb-1", "{v}" }
                    }
                    div { dangerous_inner_html: "{item.info.context}" }
                }
            }
        };

        match item.info.position {
            Position::BottomLeft => bottom_left_ele.push(element?),
            Position::BottomRight => bottom_right_ele.push(element?),
            Position::TopLeft => top_left_ele.push(element?),
            Position::TopRight => top_right_ele.push(element?),
        }
    }

    let _ = use_resource(move || async move {
        loop {
            let now = chrono::Local::now().timestamp();
            manager.write().list.retain(|_, item| {
                if let Some(hide_after) = item.hide_after {
                    now < hide_after
                } else {
                    true
                }
            });
            time_sleep(100).await;
        }
    });

    rsx! {
        div { class: "fixed z-50",
            {bottom_left_ele.into_iter()}
            {bottom_right_ele.into_iter()}
            {top_left_ele.into_iter()}
            {top_right_ele.into_iter()}
        }
    }
}

// Default desktop implementation
async fn time_sleep(millis: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;
}

// Web override when web feature is enabled
#[cfg(feature = "web")]
async fn time_sleep(millis: u64) {
    use wasm_bindgen_futures::JsFuture;
    use web_sys::window;

    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let window = window().expect("no global window exists");
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve,
            millis as i32,
        ).expect("should register timeout");
    });

    JsFuture::from(promise).await.expect("should await timeout");
}