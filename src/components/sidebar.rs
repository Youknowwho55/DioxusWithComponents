use dioxus::prelude::*;
// Import all your components
use crate::components::daisy_ui::card::{Card, CardHeader, CardBody};
use crate::components::daisy_ui::button::{Button, ButtonScheme, ButtonSize, ButtonType};

#[derive(PartialEq, Clone, Debug)]
pub enum ComponentShowcase {
    Card,
    Button,
    // Add more components as needed
}

#[component]
pub fn Component_showcase() -> Element {
    let mut selected_component = use_signal(|| ComponentShowcase::Card);
    
    // List of all available components for the sidebar
    let components = [
        ComponentShowcase::Card,
        ComponentShowcase::Button,
        // Add more components as they're implemented
    ];
    
    rsx! {
        div { class: "flex min-h-screen bg-base-200",
            // Sidebar navigation
            div { class: "w-64 p-4 bg-base-300 h-screen sticky top-0",
                h2 { class: "text-xl font-bold mb-4", "Components" }
                ul { class: "menu",
                    {
                        components
                            .iter()
                            .map(|component| {
                                let is_active = *selected_component.read() == *component;
                                let component_clone = component.clone();
                                rsx! {
                                    li { key: "{component:?}",
                                        a {
                                            class: if is_active { "active" } else { "" },
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
}

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
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
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
                        "Primary Button"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Outline),
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
                        "Outline Button"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Danger),
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
                        "Danger Button"
                    }
                }
            }
            // Button sizes section
            div { class: "mb-8",
                h3 { class: "text-xl font-bold mb-3", "Button Sizes" }
                div { class: "flex flex-wrap gap-4 items-center",
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: Some(ButtonSize::ExtraSmall),
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Extra Small"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: Some(ButtonSize::Small),
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Small"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: Some(ButtonSize::Medium),
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Medium"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: Some(ButtonSize::Large),
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Large"
                    }
                }
            }
            // Special states section
            div { class: "mb-8",
                h3 { class: "text-xl font-bold mb-3", "Special States" }
                div { class: "flex flex-wrap gap-4",
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: None,
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: Some(true),
                        prefix_image_src: None,
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: Some("Loading...".to_string()),
                        "Disabled Button"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: None,
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: Some("/icons/plus.svg".to_string()),
                        suffix_image_src: None,
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Button with Prefix Icon"
                    }
                    Button {
                        button_scheme: Some(ButtonScheme::Primary),
                        button_size: None,
                        button_type: None,
                        class: None,
                        id: None,
                        disabled: None,
                        prefix_image_src: None,
                        suffix_image_src: Some("/icons/arrow.svg".to_string()),
                        drawer_trigger: None,
                        modal_trigger: None,
                        disabled_text: None,
                        "Button with Suffix Icon"
                    }
                }
            }
            // Component documentation
            div { class: "mt-8 p-4 bg-base-300 rounded-lg",
                h3 { class: "font-bold", "Props" }
                ul { class: "list-disc pl-5 mt-2",
                    li { "button_scheme: Visual style of the button (Default, Primary, Outline, Danger)" }
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