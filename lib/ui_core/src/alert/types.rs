// src/components/ui/alert/types.rs
use dioxus::prelude::*;

Need to ADD use AlertStatus;

/*Enum: AlertVariant, AlertIcon*/
/* AlertAction, AlertProps */


/// Represents the semantic type of alert
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Info,
    Warning, // Changed from "Warn" for consistency with standard naming
    Error,
    Success,
}

/// Represents the visual status of the alert
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlertStatus {
    Visible,
    Dismissing, // For animation states
    Dismissed,
}

impl AlertVariant {
    /// Returns Tailwind classes for the alert variant
    pub fn to_classes(&self) -> &'static str {
        match self {
            AlertVariant::Default => "alert bg-blue-500 text-white",
            AlertVariant::Info => "alert bg-blue-400 text-white",
            AlertVariant::Warning => "alert bg-yellow-500 text-white",
            AlertVariant::Error => "alert bg-red-500 text-white",
            AlertVariant::Success => "alert bg-green-500 text-white",
        }
    }
    
    /// Get the matching icon for this variant
    pub fn get_icon(&self) -> AlertIcon {
        match self {
            AlertVariant::Default => AlertIcon::Default,
            AlertVariant::Info => AlertIcon::Info,
            AlertVariant::Warning => AlertIcon::Warning,
            AlertVariant::Error => AlertIcon::Error,
            AlertVariant::Success => AlertIcon::Success,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertIcon {
    #[default]
    Default,
    Info,
    Warning, // Renamed for consistency
    Error,
    Success,
    None, // Added option to hide icon
}

impl AlertIcon {
    /// Returns SVG icon markup for the alert
    pub fn to_svg(&self) -> Option<&'static str> {
        match self {
            AlertIcon::Default => Some(DEFAULT_ICON),
            AlertIcon::Info => Some(INFO_ICON),
            AlertIcon::Warning => Some(WARN_ICON),
            AlertIcon::Error => Some(ERROR_ICON),
            AlertIcon::Success => Some(SUCCESS_ICON),
            AlertIcon::None => None,
        }
    }
}

/// Represents an action button for the alert
#[derive(Clone, PartialEq)]
pub struct AlertAction {
    pub label: String,
    pub on_click: Option<EventHandler<MouseEvent>>,
    pub class: String,
}

// Icon constants
const DEFAULT_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#;
const INFO_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M12 20a8 8 0 10-8-8 8 8 0 008 8z"/></svg>"#;
const WARN_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>"#;
const ERROR_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#;
const SUCCESS_ICON: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#;

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    /// Main content of the alert
    pub children: Element,
    
    /// Optional title for the alert
    #[props(default)]
    pub title: Option<String>,
    
    /// The semantic variant of the alert
    #[props(default)]
    pub variant: AlertVariant,
    
    /// Custom icon override, defaults to matching the variant
    #[props(default)]
    pub icon: Option<AlertIcon>,
    
    /// Additional CSS classes
    #[props(default)]
    pub class: String,
    
    /// Whether the alert can be dismissed
    #[props(default = false)]
    pub dismissible: bool,
    
    /// Event triggered when alert is dismissed
    #[props(default)]
    pub on_dismiss: Option<EventHandler<MouseEvent>>,
    
    /// Action buttons for the alert
    #[props(default)]
    pub actions: Vec<AlertAction>,
    
    /// Accessibility role
    #[props(default = "alert")]
    pub role: &'static str,
    
    /// Auto dismiss timing in milliseconds, None for no auto-dismiss
    #[props(default)]
    pub auto_dismiss: Option<u32>,
}