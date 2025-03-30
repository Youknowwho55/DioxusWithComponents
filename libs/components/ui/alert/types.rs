// src/components/ui/alert/types.rs
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertColor {
    #[default]
    Default,
    Info,
    Warn,
    Error,
    Success,
}

impl AlertColor {
    /// Returns Tailwind classes for the alert color variant
    pub fn to_classes(&self) -> &'static str {
        match self {
            AlertColor::Default => "alert bg-blue-500 text-white",
            AlertColor::Info => "alert bg-blue-400 text-white",
            AlertColor::Warn => "alert bg-yellow-500 text-white",
            AlertColor::Error => "alert bg-red-500 text-white",
            AlertColor::Success => "alert bg-green-500 text-white",
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WithButtons {
    #[default]
    Default,
    Buttons,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertIcon {
    #[default]
    Default,
    Info,
    Warn,
    Error,
    Success,
}

impl AlertIcon {
    /// Returns SVG icon markup for the alert
    pub fn to_svg(&self) -> &'static str {
        match self {
            AlertIcon::Default => DEFAULT_ICON,
            AlertIcon::Info => INFO_ICON,
            AlertIcon::Warn => WARN_ICON,
            AlertIcon::Error => ERROR_ICON,
            AlertIcon::Success => SUCCESS_ICON,
        }
    }
}

// Icon constants for better readability
const DEFAULT_ICON: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>"#;

const INFO_ICON: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M12 20a8 8 0 10-8-8 8 8 0 008 8z"/>
</svg>"#;

const WARN_ICON: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
</svg>"#;

const ERROR_ICON: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>"#;

const SUCCESS_ICON: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
</svg>"#;

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    #[props(default)]
    pub alert_icon: AlertIcon,
    pub children: Element,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub alert_color: AlertColor,
    #[props(default)]
    pub with_buttons: WithButtons,
}