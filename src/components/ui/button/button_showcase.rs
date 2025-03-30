use dioxus::prelude::*;
use super::{Button, ButtonScheme, ButtonType, ButtonSize};

/// Button component showcase demonstrating all variants and configurations
pub fn render_button_showcase() -> Element {
    rsx! {
        div { class: "space-y-8 p-6",
            // Title and description
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold", "Button Component" }
                p { class: "text-gray-600 mt-2",
                    "Fully customizable button with multiple schemes, sizes, and types."
                }
            }

            // Schemes Section
            section { class: "space-y-4",
                h2 { class: "text-xl font-semibold", "Color Schemes" }
                div { class: "flex flex-wrap gap-4",
                    Button { button_scheme: ButtonScheme::Default, "Default" }
                    Button { button_scheme: ButtonScheme::Primary, "Primary" }
                    Button { button_scheme: ButtonScheme::Warn, "Warning" }
                    Button { button_scheme: ButtonScheme::Danger, "Danger" }
                }
            }

            // Sizes Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Size Variants" }
                div { class: "flex flex-wrap items-center gap-4",
                    Button { button_size: ButtonSize::ExtraSmall, "XS" }
                    Button { button_size: ButtonSize::Small, "Small" }
                    Button { button_size: ButtonSize::Medium, "Medium" }
                    Button { button_size: ButtonSize::Large, "Large" }
                }
            }

            // Types Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Button Types" }
                div { class: "flex flex-wrap gap-4",
                    Button { button_type: ButtonType::Button, "Button" }
                    Button { button_type: ButtonType::Submit, "Submit" }
                    Button { button_type: ButtonType::Reset, "Reset" }
                }
            }

            // Interactive Examples
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Interactive Examples" }
                // With icons
                div { class: "flex flex-wrap gap-4",
                    Button {
                        button_scheme: ButtonScheme::Primary,
                        prefix_image_src: Some("/icons/plus.svg".into()),
                        "Add Item"
                    }
                    Button {
                        button_scheme: ButtonScheme::Danger,
                        suffix_image_src: Some("/icons/trash.svg".into()),
                        "Delete"
                    }
                }
                // Disabled state
                div { class: "flex flex-wrap gap-4 mt-4",
                    Button {
                        disabled: true,
                        disabled_text: Some("Processing...".into()),
                        "Disabled Button"
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
                                name: "button_scheme",
                                type_info: "ButtonScheme",
                                description: "Color variant (Default, Primary, Warn, Danger)",
                            }
                            PropRow {
                                name: "button_size",
                                type_info: "ButtonSize",
                                description: "Size variant (ExtraSmall, Small, Medium, Large)",
                            }
                            PropRow {
                                name: "button_type",
                                type_info: "ButtonType",
                                description: "HTML button type (Button, Submit, Reset)",
                            }
                            PropRow {
                                name: "class",
                                type_info: "Option<String>",
                                description: "Additional CSS classes",
                            }
                            PropRow {
                                name: "prefix_image_src",
                                type_info: "Option<String>",
                                description: "Image source to display before text",
                            }
                            PropRow {
                                name: "suffix_image_src",
                                type_info: "Option<String>",
                                description: "Image source to display after text",
                            }
                            PropRow {
                                name: "disabled",
                                type_info: "Option<bool>",
                                description: "Whether the button is disabled",
                            }
                            PropRow {
                                name: "disabled_text",
                                type_info: "Option<String>",
                                description: "Text to show when button is disabled",
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