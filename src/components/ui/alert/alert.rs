// src/components/DaisyUI/alert.rs
use dioxus::prelude::*;
use super::types::{AlertColor, AlertIcon, WithButtons, AlertProps};

/// A customizable Alert component with icons and optional action buttons.
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let base_classes = props.alert_color.to_classes();
    let combined_classes = format!("alert {} {}", base_classes, props.class);

    rsx! {
        div { class: "{combined_classes}", role: "alert",
            // Icon section
            div {
                class: "alert-icon",
                dangerous_inner_html: "{props.alert_icon.to_svg()}",
            }
            // Message content
            div { class: "alert-content", {props.children} }
            // Optional buttons
            {
                match props.with_buttons {
                    WithButtons::Buttons => rsx! {
                        div { class: "alert-actions flex gap-2 mt-2",
                            button { class: "btn btn-sm", onclick: move |_| log::info!("Deny clicked"), "Cancel" }
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| log::info!("Accept clicked"),
                                "Confirm"
                            }
                        }
                    },
                    WithButtons::Default => rsx! {
                        Fragment {}
                    },
                }
            }
        }
    }
}