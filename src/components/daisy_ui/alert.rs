// src/components/DaisyUI/alert.rs
#![allow(non_snake_case)]
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
    pub fn to_string(&self) -> &'static str {
        match self {
            AlertColor::Default => "alert bg-blue-500 text-white font-bold py-2 px-4",
            AlertColor::Info => "alert bg-blue-500 text-white font-bold py-2 px-4",
            AlertColor::Warn => "alert bg-yellow-500 text-white font-bold py-2 px-4",
            AlertColor::Error => "alert bg-red-500 text-white font-bold py-2 px-4",
            AlertColor::Success => "alert bg-green-500 text-white font-bold py-2 px-4",
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
    pub fn to_string(&self) -> &'static str {
        match self {
            AlertIcon::Default => r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>"#,
            AlertIcon::Info => r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-6 w-6 shrink-0 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M12 20a8 8 0 10-8-8 8 8 0 008 8z"></path></svg>"#,
            AlertIcon::Warn => r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>"#,
            AlertIcon::Error => r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>"#,
            AlertIcon::Success => r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>"#,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    #[props(default)]
    alert_icon: AlertIcon,
    children: Element,
    #[props(default)]
    class: String,
    #[props(default)]
    alert_color: AlertColor,
    #[props(default)]
    with_buttons: WithButtons,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let class = format!("{} {}", props.alert_color.to_string(), props.class);

    rsx! {
        div { class: "{class}",
            div {
                class: "icon",
                dangerous_inner_html: props.alert_icon.to_string(),
            }
            p { class: "message", {props.children} }
            if props.with_buttons == WithButtons::Buttons {
                div { class: "alert-buttons flex gap-2 mt-2",
                    button {
                        class: "btn btn-sm",
                        onclick: move |_| {
                            log::info!("Deny clicked");
                        },
                    }
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| {
                            log::info!("Accept clicked");
                        },
                    }
                }
            }
        }
    }
}
