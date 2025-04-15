#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerSize {
    #[default]
    Default,
    Small,
    Large,
    ExtraLarge,
}

impl SpinnerSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Default => "w-8 h-8",
            Self::Small => "w-4 h-4",
            Self::Large => "w-12 h-12",
            Self::ExtraLarge => "w-16 h-16",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerColor {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl SpinnerColor {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Primary => "text-blue-600",
            Self::Secondary => "text-gray-600",
            Self::Success => "text-green-600",
            Self::Danger => "text-red-600",
            Self::Warning => "text-yellow-500",
            Self::Info => "text-cyan-600",
            Self::Light => "text-gray-200",
            Self::Dark => "text-gray-800",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerVariant {
    #[default]
    Border,
    Dots,
    Grow,
}

impl SpinnerVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::Border => "animate-spin border-4 border-solid border-current border-r-transparent rounded-full",
            Self::Dots => "flex space-x-1",
            Self::Grow => "animate-pulse rounded-full bg-current",
        }
    }
    
    pub fn dot_animation_class(index: usize) -> &'static str {
        match index {
            0 => "animate-bounce",
            1 => "animate-bounce delay-100",
            2 => "animate-bounce delay-200",
            _ => "animate-bounce delay-300",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SpinnerProps {
    #[props(default)]
    size: SpinnerSize,
    #[props(default)]
    color: SpinnerColor,
    #[props(default)]
    variant: SpinnerVariant,
    class: Option<String>,
    label: Option<String>,
}

#[component]
pub fn LoadingSpinner(props: SpinnerProps) -> Element {
    let SpinnerProps { size, color, variant, class, label } = props;
    
    let base_class = format!(
        "inline-block {} {} {} {}",
        size.to_class(),
        color.to_class(),
        variant.to_class(),
        class.unwrap_or_default()
    );

    let spinner = match variant {
        SpinnerVariant::Border => rsx! {
            div { 
                class: "{base_class}", 
                role: "status",
                span { class: "sr-only", "Loading..." }
            }
        },
        SpinnerVariant::Dots => {
            let dots = (0..3).map(|i| rsx! {
                div {
                    key: "{i}",
                    class: "w-2 h-2 bg-current rounded-full {}",
                    SpinnerVariant::dot_animation_class(i)
                }
            });
            
            rsx! {
                div { 
                    class: "{base_class}", 
                    role: "status",
                    {dots.collect::<Vec<_>>()}
                    span { class: "sr-only", "Loading..." }
                }
            }
        },
        SpinnerVariant::Grow => rsx! {
            div { 
                class: "{base_class}", 
                role: "status",
                span { class: "sr-only", "Loading..." }
            }
        },
    };

    rsx! {
        div { class: "flex items-center",
            {spinner}
            {label.map(|text| rsx! {
                span { class: "ml-2", "{text}" }
            })}
        }
    }
}