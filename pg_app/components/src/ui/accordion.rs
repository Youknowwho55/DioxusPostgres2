// src/components/daisy_ui/accordion.rs
use dioxus::prelude::*;

/// A customizable accordion component with multiple behavior variants
///
/// # Example
/// ```rust
/// Accordion {
///     name: "faq-group".into(),
///     title: "How does this work?".into(),
///     children: rsx! { p { "Detailed explanation here..." } }
/// }
/// ```
/// 



/// Visual and behavioral variants for accordions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccordionType {
    /// Checkbox behavior without visual indicator
    Default,
    /// Checkbox behavior with arrow indicator
    Arrow,
    /// Checkbox behavior with plus/minus indicator
    Plus,
    /// Radio behavior (single open) with arrow indicator
    Radio,
    /// Radio behavior (single open) with plus/minus indicator
    RadioPlus,
}

impl Default for AccordionType {
    fn default() -> Self {
        Self::Default
    }
}

impl AccordionType {
    /// Returns CSS classes needed for the accordion variant
    pub fn indicator_class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Arrow | Self::Radio => "accordion-arrow",
            Self::Plus | Self::RadioPlus => "accordion-plus",
        }
    }

    /// Returns true for radio-style accordions (single open)
    pub fn is_radio_behavior(&self) -> bool {
        matches!(self, Self::Radio | Self::RadioPlus)
    }
}

/// Properties for the Accordion component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Content shown when expanded
    pub children: Element,

    /// Header text
    pub title: String,

    /// Unique identifier
    #[props(default)]
    pub id: Option<String>,

    /// Group name for radio behavior
    pub name: String,

    /// Initial open state
    #[props(default)]
    pub open: Option<bool>,

    /// Visual style variant
    #[props(default)]
    pub accordion_type: Option<AccordionType>,

    /// Whether to visually join with adjacent accordions
    #[props(default)]
    pub join: Option<bool>,

    /// Container classes
    #[props(default)]
    pub class: Option<String>,

    /// Title section classes
    #[props(default)]
    pub title_class: Option<String>,

    /// Content section classes
    #[props(default)]
    pub content_class: Option<String>,

    /// Disabled state
    #[props(default)]
    pub disabled: Option<bool>,

    /// Callback when toggle state changes
    #[props(default)]
    pub on_toggle: Option<EventHandler<bool>>,

    /// Background color on hover
    #[props(default)]
    pub hover_bg: Option<String>,

    /// Border color
    #[props(default)]
    pub border_color: Option<String>,

    /// Enable open/close animation
    #[props(default)]
    pub animated: Option<bool>,

    /// Additional inline styles
    #[props(default)]
    pub style: Option<String>,
}


#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    // Determine accordion behavior and styling
    let accordion_type = props.accordion_type.unwrap_or_default();
    let is_radio = accordion_type.is_radio_behavior();
    let input_type = if is_radio { "radio" } else { "checkbox" };

    // Build conditional classes
    let container_classes = format!(
        "collapse bg-white border rounded-lg shadow-sm {} {} {} {} {} {}",
        accordion_type.indicator_class(),
        if props.join.unwrap_or(false) { 
            "join-item border-t-0 first:rounded-t-lg last:rounded-b-lg" 
        } else { "" },
        if props.disabled.unwrap_or(false) { 
            "opacity-60 cursor-not-allowed" 
        } else { "" },
        if props.animated.unwrap_or(true) { 
            "transition-all duration-300 ease-in-out" 
        } else { "" },
        props.border_color.as_deref().unwrap_or("border-gray-200"),
        props.class.as_deref().unwrap_or("")
    );

    let title_classes = format!(
        "collapse-title text-lg font-medium p-4 min-h-0 {} {}",
        props.hover_bg.as_deref().unwrap_or("hover:bg-gray-500"),
        props.title_class.as_deref().unwrap_or("")
    );

    let content_classes = format!(
        "collapse-content p-4 {}",
        props.content_class.as_deref().unwrap_or("")
    );

    // State management
    let mut is_open = use_signal(|| props.open.unwrap_or(false));
    let content_id = format!("{}-content", props.id.as_deref().unwrap_or("accordion"));
    let label_id = format!("{}-label", props.id.as_deref().unwrap_or("accordion"));

    rsx! {
        div {
            class: "{container_classes}",
            id: props.id.as_deref(),
            style: props.style.as_deref().unwrap_or(""),

            // Hidden input controls the state
            input {
                r#type: "{input_type}",
                name: "{props.name}",
                class: "peer",
                checked: is_open(),
                disabled: props.disabled.unwrap_or(false),
                aria_expanded: is_open(),
                aria_controls: "{content_id}",
                oninput: move |e| {
                    let new_state = e.value().parse().unwrap_or(false);
                    is_open.set(new_state);
                    if let Some(handler) = &props.on_toggle {
                        handler.call(new_state);
                    }
                },
            }

            // Clickable title section
            div {
                class: "{title_classes}",
                id: "{label_id}",
                role: "button",
                aria_controls: "{content_id}",
                aria_expanded: is_open(),
                "{props.title}"
            }

            // Content section
            div {
                class: "{content_classes}",
                id: "{content_id}",
                role: "region",
                aria_labelledby: "{label_id}",
                hidden: if is_open() { None } else { Some("until-found") },
                {props.children}
            }
        }
    }
}