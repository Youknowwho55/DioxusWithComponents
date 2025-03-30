// src/components/ui/check_box/types.rs
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxScheme {
    #[default]
    Default,
    Primary,
    Outline,
    Danger,
}

impl CheckBoxScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            CheckBoxScheme::Default => "checkbox-default",
            CheckBoxScheme::Primary => "checkbox-primary",
            CheckBoxScheme::Outline => "checkbox-outline",
            CheckBoxScheme::Danger => "checkbox-warning",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl CheckBoxSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            CheckBoxSize::Default => "checkbox-sm",
            CheckBoxSize::ExtraSmall => "checkbox-xs",
            CheckBoxSize::Small => "checkbox-sm",
            CheckBoxSize::Medium => "checkbox-md",
            CheckBoxSize::Large => "checkbox-lg",
        }
    }
}
