// lib/ui_core/src/modal/types.rs
use dioxus::prelude::*;


#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    trigger_id: String,
    children: Element,
    submit_action: Option<String>,
    class: Option<String>,
}

impl ModalProps {
    pub fn new(trigger_id: String, children: Element, submit_action: Option<String>, class: Option<String>) -> Self {
        Self { trigger_id, children, submit_action, class }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
}

impl ModalBodyProps {
    pub fn new( children: Element,  class: Option<String>) -> Self {
        Self {  children, class }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
}

impl ModalActionProps {
    pub fn new( children: Element,  class: Option<String>) -> Self {
        Self {  children, class }
    }
}