// Button showcase component
use dioxus::prelude::*;
use super::{
    Button,
    ButtonScheme,
    ButtonType,
    ButtonSize,
};
pub fn render_button_showcase() -> Element {
    rsx! {
        div { class: "space-y-8",
            h2 { class: "text-2xl font-bold", "Button Component" }
            p { class: "text-gray-600 mb-4",
                "A customizable button component with various schemes, sizes and types."
            }
            // Button schemes section
            div { class: "mb-8",
                h3 { class: "text-xl font-bold mb-3", "Button Schemes" }
                div { class: "flex flex-wrap gap-4",
                    Button {
                        button_scheme: Some(ButtonScheme::Default),
                        button_size: None,
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Default Button"
                    }
                    // Component documentation
                    div { class: "mt-8 p-4 bg-gray-100 rounded-lg w-full",
                        h3 { class: "font-bold", "Props" }
                        ul { class: "list-disc pl-5 mt-2",
                            li {
                                "button_scheme: Visual style of the button (Default, Primary, Outline, Danger)"
                            }
                            li { "button_size: Size of the button (ExtraSmall, Small, Medium, Large)" }
                            li { "button_type: HTML button type (Button, Submit, Reset)" }
                            li { "class: Optional additional CSS classes" }
                            li { "id: Optional unique identifier" }
                            li { "disabled: Whether the button is disabled" }
                            li { "prefix_image_src: Optional image to show before the button text" }
                            li { "suffix_image_src: Optional image to show after the button text" }
                            li { "drawer_trigger: Optional drawer ID to trigger" }
                            li { "modal_trigger: Optional modal ID to trigger" }
                            li { "disabled_text: Text to show when button is disabled" }
                        }
                    }
                }
            }
        }
    }
}
// END Button showcase component
