// src/components/ui/button/types.rs
use dioxus::prelude::*;

/// Visual style variants for buttons
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Primary,
    Warn,
    Danger,
}

impl ButtonScheme {
    /// Returns Tailwind classes for the button scheme
    pub fn to_classes(&self) -> &'static str {
        match self {
            ButtonScheme::Default => "bg-blue-500 hover:bg-blue-700 text-white",
            ButtonScheme::Primary => "bg-green-500 hover:bg-green-700 text-white",
            ButtonScheme::Warn => "bg-yellow-500 hover:bg-yellow-700 text-white",
            ButtonScheme::Danger => "bg-red-500 hover:bg-red-700 text-white",
        }
    }
}

/// HTML button type attributes
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl ButtonType {
    /// Returns the HTML button type string
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        }
    }
}

/// Size variants for buttons
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    /// Returns Tailwind classes for the button size
    pub fn to_classes(&self) -> &'static str {
        match self {
            ButtonSize::Default => "px-3 py-1.5 text-sm",
            ButtonSize::ExtraSmall => "px-2 py-1 text-xs",
            ButtonSize::Small => "px-3 py-1.5 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-5 py-2.5 text-lg",
        }
    }
}

/// Properties for the Button component
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Child elements (button content)
    pub children: Element,
    
    /// HTML id attribute
    #[props(default)]
    pub id: Option<String>,
    
    /// Disabled state
    #[props(default)]
    pub disabled: Option<bool>,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    
    /// Image source to display before text
    #[props(default)]
    pub prefix_image_src: Option<String>,
    
    /// Image source to display after text
    #[props(default)]
    pub suffix_image_src: Option<String>,
    
    /// Button type variant
    #[props(default)]
    pub button_type: Option<ButtonType>,
    
    /// Size variant
    #[props(default)]
    pub button_size: Option<ButtonSize>,
    
    /// Color scheme variant
    #[props(default)]
    pub button_scheme: Option<ButtonScheme>,
    
    /// Target drawer ID for interaction
    #[props(default)]
    pub drawer_trigger: Option<String>,
    
    /// Target modal ID for interaction
    #[props(default)]
    pub modal_trigger: Option<String>,
    
    /// Text to show when button is disabled
    #[props(default)]
    pub disabled_text: Option<String>,
}