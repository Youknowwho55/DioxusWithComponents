#![allow(non_snake_case)]
use dioxus::prelude::*;



#[component]
pub fn DropDown(props: DropDownProps) -> Element {
    let direction = if let Some(direction) = props.direction {
        direction.to_string()
    } else {
        Direction::default().to_string()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    rsx!(
        div { class: "dropdown {class} {direction}",
            label {
                tabindex: "0",
                class: "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", class: "mr-2", width: "16" }
                }
                span { class: "truncate", "{props.button_text}" }
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", class: "ml-2", width: "12" }
                } else if props.carat.is_some() && props.carat.unwrap() {
                    div { class: "dropdown-caret" }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
                {props.children}
            }
        }
    )
}


#[component]
pub fn DropDownLink(props: DropDownLinkProps) -> Element {
    let class = if let Some(class) = props.class {
        format!("dropdown-item {} ", class)
    } else {
        "dropdown-item".to_string()
    };

    if let Some(trigger) = &props.drawer_trigger {
        rsx!(
            li {
                a {
                    class: "{class}",
                    "data-drawer-target": "{trigger}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    } else {
        rsx!(
            li {
                a {
                    class: "{class}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    }
}
