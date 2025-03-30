// src/components/ui/button.rs
use dioxus::prelude::*;
use super::types::{ButtonProps, ButtonScheme, ButtonType, ButtonSize};

/// A customizable button component with multiple variants
///
/// # Example
/// ```rust
/// Button {
///     button_scheme: ButtonScheme::Primary,
///     button_size: ButtonSize::Large,
///     "Click Me"
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    // Use unwrap_or_default for cleaner default handling
    let scheme = props.button_scheme.unwrap_or_default();
    let size = props.button_size.unwrap_or_default();
    let button_type = props.button_type.unwrap_or_default();

    // Combine classes more efficiently
    let mut class = format!(
        "btn {} {} {}",
        scheme.to_classes(),
        size.to_classes(),
        props.class.as_deref().unwrap_or("")
    ).trim().to_string();

    // Add disabled styles if needed
    if props.disabled.unwrap_or(false) {
        class.push_str(" opacity-75 cursor-not-allowed");
    }

    rsx! {
        button {
            class: "{class}",
            id: props.id.as_deref(),
            disabled: props.disabled.unwrap_or(false),
            "data-drawer-target": props.drawer_trigger.as_deref(),
            "data-modal-target": props.modal_trigger.as_deref(),
            r#type: "{button_type.as_str()}",
            "data-disabled-text": props.disabled_text.as_deref(),
            // Prefix icon
            {props.prefix_image_src.as_ref().map(|src| rsx! {
                img {
                    src: "{src}",
                    class: "mr-2",
                    width: "12",
                    // Better accessibility
                    alt: "",
                    aria_hidden: "true",
                    loading: "lazy",
                }
            })}
            // Button content
            {props.children}
            // Suffix icon
            {props.suffix_image_src.as_ref().map(|src| rsx! {
                img {
                    src: "{src}",
                    class: "ml-2",
                    width: "12",
                    alt: "",
                    aria_hidden: "true",
                    loading: "lazy",
                }
            })}
        }
    }
}