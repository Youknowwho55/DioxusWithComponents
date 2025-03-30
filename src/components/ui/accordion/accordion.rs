// src/components/daisy_ui/accordion.rs
#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccordionType {
    Default,    // Checkbox behavior, no indicator
    Arrow,      // Checkbox behavior with arrow indicator
    Plus,       // Checkbox behavior with plus/minus indicator
    Radio,      // Radio behavior with arrow indicator
    RadioPlus,  // Radio behavior with plus/minus indicator
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
    /// Disable the accordion
    disabled: Option<bool>,
    /// Callback when accordion is toggled
    on_toggle: Option<EventHandler<bool>>,
    /// Background color when hovered
    hover_bg: Option<String>,
    /// Border color
    border_color: Option<String>,
    /// Whether to add animation when opening/closing
    animated: Option<bool>,
    /// Additional styles for the container
    style: Option<String>,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    // Get accordion type and determine behavior
    let accordion_type = props.accordion_type.unwrap_or(AccordionType::Default);
    let type_classes = accordion_type.get_classes();
    let input_type = if accordion_type.is_radio() { "radio" } else { "checkbox" };
    
    // Determine if we should join with adjacent accordions
    let join_class = if props.join.unwrap_or(false) { 
        "join-item border-t-0 first:rounded-t-lg last:rounded-b-lg" 
    } else { 
        "" 
    };
    
    // Animation class
    let animated_class = if props.animated.unwrap_or(true) { 
        "transition-all duration-300 ease-in-out" 
    } else { 
        "" 
    };
    
    // Disabled state
    let disabled_class = if props.disabled.unwrap_or(false) { 
        "opacity-60 cursor-not-allowed" 
    } else { 
        "" 
    };
    
    // Hover background
    let hover_bg = props.hover_bg.as_deref().unwrap_or("hover:bg-gray-50");
    
    // Border color
    let border_color = props.border_color.as_deref().unwrap_or("border-gray-200");
    
    // Build container classes
    let container_class = format!(
        "collapse bg-base-100 border {border_color} rounded-lg shadow-sm {} {} {} {} {}",
        type_classes,
        join_class,
        disabled_class,
        animated_class,
        props.class.clone().unwrap_or_default()
    );
    
    // Title classes
    let title_class = format!(
        "collapse-title text-lg font-medium p-4 min-h-0 {hover_bg} {}",
        props.title_class.clone().unwrap_or_default()
    );
    
    // Content classes
    let content_class = format!(
        "collapse-content p-4 {}",
        props.content_class.clone().unwrap_or_default()
    );

    let mut is_open = use_signal(|| props.open.unwrap_or(false));
    
    rsx!(
        div {
            class: "{container_class}",
            id: props.id.clone(),
            style: props.style.unwrap_or_default(),
            input {
                checked: is_open(),
                "type": "{input_type}",
                name: "{props.name}",
                class: "peer",
                disabled: props.disabled.unwrap_or(false),
                oninput: move |e| {
                    let new_state = e.value().parse().unwrap_or(false);
                    is_open.set(new_state);
                    if let Some(handler) = &props.on_toggle {
                        handler.call(new_state);
                    }
                },
                // Accessibility attributes
                aria_expanded: is_open(),
                aria_controls: format!("{}-content", props.id.as_deref().unwrap_or("accordion")),
            }
            div {
                class: "{title_class}",
                id: format!("{}-label", props.id.as_deref().unwrap_or("accordion")),
                aria_label: "Toggle accordion",
                "{props.title}"
            }
            div {
                class: "{content_class}",
                id: format!("{}-content", props.id.as_deref().unwrap_or("accordion")),
                role: "region",
                aria_labelledby: format!("{}-label", props.id.as_deref().unwrap_or("accordion")),
                hidden: if is_open() { None } else { Some("until-found") },
                {props.children}
            }
        }
    )
}