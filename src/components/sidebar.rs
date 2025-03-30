// src/components/sidebar.rs
use dioxus::prelude::*;
// Import all your components
use crate::components::daisy_ui::{
    button::{Button, ButtonScheme, ButtonSize, ButtonType},
    card::{Card, CardBody, CardFoot, CardHeader},
    accordion::{Accordion, AccordionType},
    Alert, AlertColor, AlertIcon, WithButtons
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ComponentShowcase {
    Card,
    Button,
    Accordion,
    Alert,
    // Add more components as needed
}

#[component]
pub fn Component_showcase() -> Element {
    let mut selected_component = use_signal(|| ComponentShowcase::Card);
    
    // List of all available components for the sidebar
    let components = [
        ComponentShowcase::Card,
        ComponentShowcase::Button,
        ComponentShowcase::Accordion,
        ComponentShowcase::Alert,
    ];

    rsx! {
        div { class: "flex min-h-screen bg-base-200",
            // Sidebar navigation
            div { class: "w-64 p-4 bg-base-300 h-screen sticky top-0",
                h2 { class: "text-xl font-bold mb-4", "Components" }
                ul { class: "mb-3 lg:mb-2 uppercase tracking-wide font-bold text-sm lg:text-xs text-gray-500 cursor-pointer",
                    {
                        components
                            .iter()
                            .map(|&component| {
                                let component_name = format!("{:?}", component);
                                rsx! {
                                    li { class: "py-2", key: "{component_name}",
                                        a {
                                            class: format!(
                                                "block px-4 py-2 rounded transition {}",
                                                if selected_component() == component {
                                                    "bg-red-300 text-blue-400"
                                                } else {
                                                    "hover:bg-black-100 hover:text-black-500"
                                                },
                                            ),
                                            onclick: move |_| selected_component.set(component),
                                            "{component_name}"
                                        }
                                    }
                                }
                            })
                    }
                }
            }
            // Main content area
            div { class: "flex-1 p-8", {render_selected_component(selected_component)} }
        }
    }
}
fn render_selected_component(selected: Signal<ComponentShowcase>) -> Element {
    match selected() {
        ComponentShowcase::Card => render_card_showcase(),
        ComponentShowcase::Button => render_button_showcase(),
        ComponentShowcase::Accordion => render_accordion_showcase(),
        ComponentShowcase::Alert => render_alert_showcase(),
    }

}

/// Start of the component showcase
/// This function renders the selected component based on the state

// Card showcase component
fn render_card_showcase() -> Element {
    rsx! {
        div { class: "space-y-8",
            h2 { class: "text-2xl font-bold", "Card Component" }
            p { class: "text-gray-600 mb-4",
                "A versatile card component with header and body sections."
            }
            // Example card
            div { class: "max-w-2xl",
                Card {
                    class: Some("shadow-xl bg-base-100".to_string()),
                    drawer_trigger: None,
                    modal_trigger: None,
                    CardHeader { class: None, title: "Sample Card".to_string() }
                    CardBody { class: None, "This is a card body with some sample content." }
                    CardFoot { class: None, "This is a card Foot with some sample content." }
                }
            }
            // Component documentation
            div { class: "mt-8 p-4 bg-gray-100 rounded-lg w-full",
                h3 { class: "font-bold", "Props" }
                ul { class: "list-disc pl-5 mt-2",
                    li { "class: Optional styles to apply to the card" }
                    li { "drawer_trigger: Optional drawer ID to trigger" }
                    li { "modal_trigger: Optional modal ID to trigger" }
                    li { "children: Content to display inside the card" }
                }
            }
        }
    }
} // END Card showcase component

// Button showcase component
fn render_button_showcase() -> Element {
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

// accordion showcase component
fn render_accordion_showcase() -> Element {
    rsx! {
        div { class: "space-y-8",
            h2 { class: "text-2xl font-bold", "Accordion Component" }
            // Default type
            Accordion {
                name: "demo-accordion",
                title: "Default Accordion",
                open: None,
                accordion_type: None,
                join: None,
                children: rsx! {
                    p { "This is the content of the default accordion." }
                },
            }
            // Arrow type
            Accordion {
                name: "demo-accordion",
                title: "Arrow Style Accordion",
                open: None,
                accordion_type: Some(AccordionType::Arrow),
                join: None,
                children: rsx! {
                    p { "This accordion has an arrow indicator." }
                },
            }
            // Radio group example
            div { class: "w-full mt-4",
                h4 { class: "text-lg font-bold mb-2", "Radio Group Behavior" }
                Accordion {
                    name: "radio-group",
                    title: "Section 1",
                    open: Some(true),
                    accordion_type: Some(AccordionType::Radio),
                    join: true,
                    children: rsx! {
                        p { "This is section 1 content." }
                    },
                }
                Accordion {
                    name: "radio-group",
                    title: "Section 2",
                    open: None,
                    accordion_type: Some(AccordionType::Radio),
                    join: Some(true),
                    children: rsx! {
                        p { "This is section 2 content." }
                    },
                }
            }
            // Component documentation
            div { class: "mt-8 p-4 bg-gray-100 rounded-lg w-full",
                h3 { class: "font-bold", "Props" }
                ul { class: "list-disc pl-5 mt-2",
                    li { "title: Text displayed in the accordion header" }
                    li { "name: Name attribute for grouping radio buttons" }
                    li { "checked: Whether the accordion is initially open" }
                    li { "radio: Use radio behavior (only one open at a time in a group)" }
                    li { "accordion_type: Visual indicator style (Default, Arrow, Plus)" }
                    li { "join: Whether to join with adjacent accordions visually" }
                    li { "class: Additional classes for the container" }
                    li { "title_class: Additional classes for the title" }
                    li { "content_class: Additional classes for the content" }
                }
            }
        }
    }
}
// END accordion showcase component

fn render_alert_showcase() -> Element {
    rsx! {
        div { class: "space-y-4",
            // Alert examples
            h2 { class: "text-2xl font-bold mb-4", "Alert Examples" }
            Alert { alert_color: AlertColor::Default, "This is a default info alert" }
            Alert { alert_color: AlertColor::Info, "This is an info alert" }
            Alert { alert_color: AlertColor::Success, "This is a success alert" }
            Alert { alert_color: AlertColor::Warn, "This is a warning alert" }
            Alert { alert_color: AlertColor::Error, "This is an error alert" }
            Alert {
                alert_color: AlertColor::Success,
                with_buttons: WithButtons::Buttons,
                "This is a success message with buttons!"
            }
            Alert {
                alert_icon: AlertIcon::Warn,
                alert_color: AlertColor::Warn,
                class: "my-custom-class".to_string(),
                "This is a warning alert with custom icon and class"
            }

            // Component documentation
            div { class: "mt-8 p-4 bg-gray-100 rounded-lg w-full",
                h3 { class: "font-bold text-lg", "Alert Component Props" }
                ul { class: "list-disc pl-5 mt-2 space-y-1",
                    li {
                        strong { "alert_icon: " }
                        "Optional icon to display (AlertIcon::Default, Info, Warn, Error, Success)"
                    }
                    li {
                        strong { "alert_color: " }
                        "Color variant (AlertColor::Default, Info, Warn, Error, Success)"
                    }
                    li {
                        strong { "with_buttons: " }
                        "Whether to show action buttons (WithButtons::Default or Buttons)"
                    }
                    li {
                        strong { "class: " }
                        "Additional CSS classes for the alert container"
                    }
                    li {
                        strong { "children: " }
                        "The content/message to display inside the alert"
                    }
                }
            }
        }
    }
}