

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
