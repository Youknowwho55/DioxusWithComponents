#![allow(non_snake_case)]
use dioxus::prelude::*;
use super::types::{CheckBoxScheme, CheckBoxSize};

#[derive(Props, Clone, PartialEq)]
pub struct CheckBoxProps {
    children: Element,
    id: Option<String>,
    checked: Option<bool>,
    class: Option<String>,
    name: String,
    value: String,
    checkbox_size: Option<CheckBoxSize>,
    checkbox_scheme: Option<CheckBoxScheme>,
}


#[component]
pub fn CheckBox(props: CheckBoxProps) -> Element {
    let checkbox_scheme = if props.checkbox_scheme.is_some() {
        props.checkbox_scheme.unwrap()
    } else {
        Default::default()
    };

    let checkbox_size = if props.checkbox_size.is_some() {
        props.checkbox_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let checked = if let Some(checked) = props.checked {
        if checked {
            Some("checked")
        } else {
            None
        }
    } else {
        None
    };

    let class = format!(
        "checkbox {} {} {}",
        class,
        checkbox_scheme.to_string(),
        checkbox_size.to_string()
    );

    rsx!(
        input {
            "type": "checkbox",
            class: "{class}",
            id: props.id,
            name: props.name,
            value: props.value,
            checked,
            {props.children}
        }
    )
}
