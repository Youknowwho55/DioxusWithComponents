// src/components/ui/accordion/types.rs
use dioxus::prelude::*;

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