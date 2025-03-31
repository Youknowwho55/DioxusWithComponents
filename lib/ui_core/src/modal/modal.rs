#![allow(non_snake_case)]

use dioxus::prelude::*;
use super::types::{ModalProps, ModalBodyProps, ModalActionProps};


#[component]
pub fn Modal(props: ModalProps) -> Element {
    let class = if let Some(class) = props.class {
        format!("modal {}", class)
    } else {
        "modal".to_string()
    };
    rsx!(
        if let Some(action) = &props.submit_action {
            form { action: "{action}", method: "post",
                dialog { class: "{class}", id: "{props.trigger_id}", {props.children} }
            }
        } else {
            dialog { class: "{class}", id: "{props.trigger_id}", {props.children} }
        }
    )
}



#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("modal-box {}", class);
    rsx!(
        div { class: "{class}", {props.children} }
    )
}



#[component]
pub fn ModalAction(props: ModalActionProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("modal-action {}", class);
    rsx!(
        div { class: "{class}", {props.children} }
    )
}
