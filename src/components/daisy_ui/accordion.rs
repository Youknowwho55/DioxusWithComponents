#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccordionType {
    Default,  // Checkbox behavior, no indicator
    Arrow,    // Checkbox behavior with arrow indicator
    Plus,     // Checkbox behavior with plus/minus indicator
    Radio,    // Radio behavior with arrow indicator
    RadioPlus, // Radio behavior with plus/minus indicator
}

impl AccordionType {
    pub fn get_classes(&self) -> &'static str {
        match self {
            AccordionType::Default => "",
            AccordionType::Arrow => "collapse-arrow",
            AccordionType::Plus => "collapse-plus",
            AccordionType::Radio => "collapse-arrow",
            AccordionType::RadioPlus => "collapse-plus",
        }
    }
    
    pub fn is_radio(&self) -> bool {
        matches!(self, AccordionType::Radio | AccordionType::RadioPlus)
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    children: Element,
    title: String,
    id: Option<String>,
    /// Name attribute for grouping accordions
    name: String,
    /// Initial open state
    open: Option<bool>,
    /// Visual style and behavior
    accordion_type: Option<AccordionType>,
    /// Join with adjacent accordions
    join: Option<bool>,
    /// Custom classes for container
    class: Option<String>,
    /// Custom classes for title
    title_class: Option<String>,
    /// Custom classes for content
    content_class: Option<String>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    // Get accordion type and determine behavior
    let accordion_type = props.accordion_type.unwrap_or(AccordionType::Default);
    let type_classes = accordion_type.get_classes();
    let input_type = if accordion_type.is_radio() { "radio" } else { "checkbox" };
    
    // Determine if we should join with adjacent accordions
    let join_class = if props.join.unwrap_or(false) { "join-item" } else { "" };
    
    // Build container classes
    let container_class = format!(
        "collapse bg-white border border-gray-200 rounded-lg shadow-sm {} {} {}",
        type_classes,
        join_class,
        props.class.clone().unwrap_or_default()
    );
    
    // Title classes
    let title_class = format!(
        "collapse-title text-lg font-medium p-4 hover:bg-gray-50 {}",
        props.title_class.clone().unwrap_or_default()
    );
    
    // Content classes
    let content_class = format!(
        "collapse-content p-4 {} ",
        props.content_class.clone().unwrap_or_default()
    );

    rsx!(
        div { class: "{container_class}", id: props.id,
            input {
                checked: props.open,
                "type": "{input_type}",
                name: "{props.name}",
                class: "peer",
            }
            div { class: "{title_class}", "{props.title}" }
            div { class: "{content_class}", {props.children} }
        }
    )
}

