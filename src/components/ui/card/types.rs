// src/components/ui/card/types.rs
use dioxus::prelude::*;

/// Properties for the Card component
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Additional CSS classes for the card
    #[props(default)]
    pub class: Option<String>,
    
    /// Target drawer ID for interaction
    #[props(default)]
    pub drawer_trigger: Option<String>,
    
    /// Target modal ID for interaction
    #[props(default)]
    pub modal_trigger: Option<String>,
    
    /// Child elements
    pub children: Element,
}

/// Properties for the CardHeader component
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Header title text
    pub title: String,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    
    /// Child elements
    #[props(default)]
    pub children: Element,
}

/// Properties for the CardBody component
#[derive(Props, Clone, PartialEq)]
pub struct CardBodyProps {
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    
    /// Child elements
    pub children: Element,
}

/// Properties for the CardFoot component
#[derive(Props, Clone, PartialEq)]
pub struct CardFootProps {
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    
    /// Child elements
    #[props(default)]
    pub children: Element,
}