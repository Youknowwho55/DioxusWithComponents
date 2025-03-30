use dioxus::prelude::*;
// Import all your components
use crate::components::daisy_ui::card::{Card, CardHeader, CardBody};
use crate::components::daisy_ui::button::{Button, ButtonScheme, ButtonSize, ButtonType};
use crate::components::daisy_ui::accordion::{Accordion, AccordionType};


#[derive(PartialEq, Clone, Debug)]
pub enum ComponentShowcase {
    Card,
    Button,
    Accordion,
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

        // Add more components as they're implemented
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
                            .map(|component| {
                                let is_active = *selected_component.read() == *component;
                                let component_clone = component.clone();
                                rsx! {
                                    li { class: "py-2", key: "{component:?}",
                                        a {
                                            class: if is_active { "active bg-black-200" } else { "hover:bg-black-700" },
                                            onclick: move |_| selected_component.set(component_clone.clone()),
                                            {format!("{:?}", component)}
                                        }
                                    }
                                }
                            })
                    }
                }
            }
            // Main content area - render the selected component
            div { class: "flex-1 p-8", {render_selected_component(&selected_component)} }
        }
    }
}

// Separate function to render the selected component
fn render_selected_component(selected: &Signal<ComponentShowcase>) -> Element {
    match *selected.read() {
        ComponentShowcase::Card => render_card_showcase(),
        ComponentShowcase::Button => render_button_showcase(),
        ComponentShowcase::Accordion => render_accordion_showcase(),
        // Add cases for other components
    }
}

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
                    CardHeader { class: None, title: "Sample Card".to_string(), "" }
                    CardBody { class: None, "This is a card body with some sample content." }
                }
            }
            // Component documentation
            div { class: "mt-8 p-4 bg-base-300 rounded-lg",
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
                    join: Some(true),
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
