// src/components/daisy_ui/accordion_showcase.rs
use dioxus::prelude::*;
use super::{Accordion, AccordionType};

/// Comprehensive showcase for the Accordion component
pub fn render_accordion_showcase() -> Element {
    rsx! {
        div { class: "space-y-8 p-6",
            // Title and description
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold", "Accordion Component" }
                p { class: "text-gray-600 mt-2",
                    "Interactive collapsible sections with multiple style variants and behaviors."
                }
            }

            // Basic Examples Section
            section { class: "space-y-4",
                h2 { class: "text-xl font-semibold", "Basic Variants" }
                // Default Accordion
                div { class: "w-full max-w-2xl mx-auto",
                    Accordion {
                        name: "basic-accordion".to_string(),
                        title: "Default Accordion".to_string(),
                        children: rsx! {
                            p { "This is the default accordion style with no indicator." }
                        },
                    }
                }

                // Arrow Style
                div { class: "w-full max-w-2xl mx-auto mt-4",
                    Accordion {
                        name: "arrow-accordion".to_string(),
                        title: "Arrow Indicator".to_string(),
                        accordion_type: Some(AccordionType::Arrow),
                        children: rsx! {
                            p { "This accordion shows an arrow indicator that rotates when open." }
                        },
                    }
                }

                // Plus Style
                div { class: "w-full max-w-2xl mx-auto mt-4",
                    Accordion {
                        name: "plus-accordion".to_string(),
                        title: "Plus/Minus Indicator".to_string(),
                        accordion_type: Some(AccordionType::Plus),
                        children: rsx! {
                            p { "This accordion toggles between plus and minus indicators." }
                        },
                    }
                }
            }

            // Advanced Examples Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Advanced Usage" }
                // Joined Accordion Group
                div { class: "w-full max-w-2xl mx-auto",
                    h3 { class: "text-lg font-medium mb-2", "Joined Accordions" }
                    Accordion {
                        name: "joined-group".to_string(),
                        title: "First Item".to_string(),
                        join: Some(true),
                        children: rsx! {
                            p { "First content section" }
                        },
                    }
                    Accordion {
                        name: "joined-group".to_string(),
                        title: "Second Item".to_string(),
                        join: Some(true),
                        children: rsx! {
                            p { "Second content section" }
                        },
                    }
                    Accordion {
                        name: "joined-group".to_string(),
                        title: "Third Item".to_string(),
                        join: Some(true),
                        children: rsx! {
                            p { "Third content section" }
                        },
                    }
                }

                // Radio Behavior Group
                div { class: "w-full max-w-2xl mx-auto mt-6",
                    h3 { class: "text-lg font-medium mb-2", "Radio Behavior (Single Open)" }
                    Accordion {
                        name: "radio-group".to_string(),
                        title: "Option 1".to_string(),
                        accordion_type: Some(AccordionType::Radio),
                        join: Some(true),
                        children: rsx! {
                            p { "Only one item can be open at a time" }
                        },
                    }
                    Accordion {
                        name: "radio-group".to_string(),
                        title: "Option 2".to_string(),
                        accordion_type: Some(AccordionType::Radio),
                        join: Some(true),
                        children: rsx! {
                            p { "Selecting one closes others" }
                        },
                    }
                }
            }

            // Props Documentation
            section { class: "mt-12 p-6 bg-gray-50 rounded-lg",
                h2 { class: "text-xl font-semibold mb-4", "Component API" }
                // Props Table
                div { class: "overflow-x-auto",
                    table { class: "min-w-full divide-y divide-gray-200",
                        thead { class: "bg-gray-100",
                            tr {
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Prop"
                                }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Type"
                                }
                                th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                    "Description"
                                }
                            }
                        }
                        tbody { class: "bg-white divide-y divide-gray-200",
                            PropRow {
                                name: "name",
                                type_info: "String",
                                description: "Group identifier (required for radio behavior)",
                            }
                            PropRow {
                                name: "title",
                                type_info: "String",
                                description: "Header text content",
                            }
                            PropRow {
                                name: "accordion_type",
                                type_info: "AccordionType",
                                description: "Visual style and behavior variant",
                            }
                            PropRow {
                                name: "join",
                                type_info: "Option<bool>",
                                description: "Whether to visually join with adjacent accordions",
                            }
                            PropRow {
                                name: "open",
                                type_info: "Option<bool>",
                                description: "Initial open state",
                            }
                            PropRow {
                                name: "class",
                                type_info: "Option<String>",
                                description: "Additional container classes",
                            }
                            PropRow {
                                name: "on_toggle",
                                type_info: "Option<EventHandler<bool>>",
                                description: "Callback when open state changes",
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Reusable prop documentation component
#[component]
fn PropRow(name: &'static str, type_info: &'static str, description: &'static str) -> Element {
    rsx! {
        tr {
            td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900",
                "{name}"
            }
            td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-blue-600",
                "{type_info}"
            }
            td { class: "px-6 py-4 text-sm text-gray-500", "{description}" }
        }
    }
}