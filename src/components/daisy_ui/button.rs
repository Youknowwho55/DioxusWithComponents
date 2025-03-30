#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Primary,
    Warn,
    Danger,
}

impl ButtonScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonScheme::Default => "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded cursor-pointer active:bg-blue-800 active:transform active:scale-95 shadow-md hover:shadow-lg",
            ButtonScheme::Primary => "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded cursor-pointer active:bg-green-800 active:transform active:scale-95 shadow-md hover:shadow-lg",
            ButtonScheme::Warn => "bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-2 px-4 rounded cursor-pointer active:bg-yellow-800 active:transform active:scale-95 shadow-md hover:shadow-lg",
            ButtonScheme::Danger => "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded cursor-pointer active:bg-red-800 active:transform active:scale-95 shadow-md hover:shadow-lg",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl ButtonType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl ButtonSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonSize::Default => "px-3 py-1.5 text-sm",
            ButtonSize::ExtraSmall => "px-2 py-1 text-xs",
            ButtonSize::Small => "px-3 py-1.5 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-5 py-2.5 text-lg",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
    disabled_text: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = if props.button_scheme.is_some() {
        props.button_scheme.unwrap()
    } else {
        Default::default()
    };

    let button_type = if props.button_type.is_some() {
        props.button_type.unwrap()
    } else {
        Default::default()
    };
    let button_type = button_type.to_string();

    let button_size = if props.button_size.is_some() {
        props.button_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let disabled = if let Some(disabled) = props.disabled {
        if disabled {
            Some(true)
        } else {
            None
        }
    } else {
        None
    };

    let class = format!(
        "btn {} {} {}",
        class,
        button_scheme.to_string(),
        button_size.to_string()
    );

    rsx!(
        button {
            class: "{class}",
            id: props.id,
            disabled,
            "data-drawer-target": props.drawer_trigger,
            "data-modal-target": props.modal_trigger,
            "type": "{button_type}",
            "data-disabled-text": props.disabled_text,
            if let Some(img_src) = props.prefix_image_src {
                img { src: "{img_src}", class: "mr-2", width: "12" }
            }
            {props.children}
            if let Some(img_src) = props.suffix_image_src {
                img { src: "{img_src}", class: "ml-2", width: "12" }
            }
        }
    )
}
