// src/components/ui/card.rs
#![allow(non_snake_case)]

use dioxus::prelude::*;
use super::types::{CardProps, CardHeaderProps, CardBodyProps, CardFootProps};
/// A flexible card container component
///
/// # Example
/// ```rust
/// Card {
///     class: "shadow-lg".into(),
///     drawer_trigger: Some("my-drawer".into()),
///     children: rsx! {
///         CardHeader { title: "My Card" }
///         CardBody { "Card content here" }
///         CardFoot { "Footer content" }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let class = props.class.as_deref().unwrap_or("");
    let class = format!("card {}", class.trim());
    
    rsx! {
        div {
            class: "{class}",
            "data-drawer-target": props.drawer_trigger.as_deref(),
            "data-modal-target": props.modal_trigger.as_deref(),
            {props.children}
        }
    }
}

/// Header section for a card component
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let class = props.class.as_deref().unwrap_or("");
    let class = format!("card-header flex items-center {}", class.trim());
    
    rsx! {
        div { class: "{class}",
            h3 {
                class: "card-title overflow-hidden text-ellipsis",
                title: "{props.title}",
                {props.title.clone()}
            }
            {props.children}
        }
    }
}

/// Main content section for a card component
#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    let class = props.class.as_deref().unwrap_or("");
    let class = format!("card-body {}", class.trim());
    
    rsx! {
        div { class: "{class}", {props.children} }
    }
}

/// Footer section for a card component
#[component]
pub fn CardFoot(props: CardFootProps) -> Element {
    let class = props.class.as_deref().unwrap_or("");
    let class = format!("card-foot {}", class.trim());
    
    rsx! {
        div { class: "{class}", {props.children} }
    }
}

