use dioxus::prelude::*;
use super::types::{InputProps, InputScheme, InputSize, InputType};
use super::Input;

/// Input component showcase demonstrating all variants and configurations
pub fn render_input_showcase() -> Element {
    rsx! {
        div { class: "space-y-8 p-6",
            // Title and description
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold", "Input Component" }
                p { class: "text-gray-600 mt-2",
                    "Fully customizable input with multiple types, schemes, sizes, and validation."
                }
            }

            // Input Types Section
            section { class: "space-y-4",
                h2 { class: "text-xl font-semibold", "Input Types" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "text-input".to_string(),
                        r#type: Some(InputType::Text),
                        placeholder: Some("Text input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "email-input".to_string(),
                        r#type: Some(InputType::Email),
                        placeholder: Some("Email input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "password-input".to_string(),
                        r#type: Some(InputType::Password),
                        placeholder: Some("Password input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "number-input".to_string(),
                        r#type: Some(InputType::Number),
                        placeholder: Some("Number input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "date-input".to_string(),
                        r#type: Some(InputType::Date),
                        placeholder: Some("Date input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "color-input".to_string(),
                        r#type: Some(InputType::Color),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "file-input".to_string(),
                        r#type: Some(InputType::File),
                        class: Some("block w-full".to_string()),
                        ..InputProps::default(),
                    }
                }
            }

            // Schemes Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Color Schemes" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "default-input".to_string(),
                        scheme: Some(InputScheme::Default),
                        placeholder: Some("Default scheme".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "primary-input".to_string(),
                        scheme: Some(InputScheme::Primary),
                        placeholder: Some("Primary scheme".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "success-input".to_string(),
                        scheme: Some(InputScheme::Success),
                        placeholder: Some("Success scheme".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "warn-input".to_string(),
                        scheme: Some(InputScheme::Warning),
                        placeholder: Some("Warning scheme".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "danger-input".to_string(),
                        scheme: Some(InputScheme::Danger),
                        placeholder: Some("Danger scheme".to_string()),
                        ..InputProps::default(),
                    }
                }
            }

            // Sizes Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Size Variants" }
                div { class: "grid grid-cols-1 gap-4",
                    Input {
                        name: "xs-input".to_string(),
                        size: Some(InputSize::ExtraSmall),
                        placeholder: Some("Extra small input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "small-input".to_string(),
                        size: Some(InputSize::Small),
                        placeholder: Some("Small input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "medium-input".to_string(),
                        size: Some(InputSize::Medium),
                        placeholder: Some("Medium input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "large-input".to_string(),
                        size: Some(InputSize::Large),
                        placeholder: Some("Large input".to_string()),
                        ..InputProps::default(),
                    }
                }
            }

            // States Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Input States" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "disabled-input".to_string(),
                        disabled: true,
                        value: Some("Disabled input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "readonly-input".to_string(),
                        readonly: true,
                        value: Some("Readonly input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "required-input".to_string(),
                        required: true,
                        placeholder: Some("Required input".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "autofocus-input".to_string(),
                        autofocus: true,
                        placeholder: Some("Autofocused input".to_string()),
                        ..InputProps::default(),
                    }
                }
            }

            // Validation Examples
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Validation Examples" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "minmax-input".to_string(),
                        r#type: Some(InputType::Number),
                        min: Some("5".to_string()),
                        max: Some("10".to_string()),
                        placeholder: Some("Number between 5-10".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "pattern-input".to_string(),
                        pattern: Some("[A-Za-z]{3}".to_string()),
                        placeholder: Some("Exactly 3 letters".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "length-input".to_string(),
                        minlength: Some(3),
                        maxlength: Some(6),
                        placeholder: Some("3-6 characters".to_string()),
                        ..InputProps::default(),
                    }
                }
            }

            // With Labels and Helpers
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "With Labels & Helpers" }
                div { class: "grid grid-cols-1 gap-4",
                    Input {
                        name: "labeled-input".to_string(),
                        label: Some("Username".to_string()),
                        placeholder: Some("Enter your username".to_string()),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "help-text-input".to_string(),
                        label: Some("Password".to_string()),
                        help_text: Some("Must be at least 8 characters".to_string()),
                        r#type: Some(InputType::Password),
                        ..InputProps::default(),
                    }
                    Input {
                        name: "error-input".to_string(),
                        label: Some("Email".to_string()),
                        error: Some("Please enter a valid email address".to_string()),
                        scheme: Some(InputScheme::Danger),
                        ..InputProps::default(),
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
                                description: "HTML name attribute (required)",
                            }
                            PropRow {
                                name: "r#type",
                                type_info: "Option<InputType>",
                                description: "Input type (Text, Email, Password, etc.)",
                            }
                            PropRow {
                                name: "scheme",
                                type_info: "Option<InputScheme>",
                                description: "Color variant (Default, Primary, Success, etc.)",
                            }
                            PropRow {
                                name: "size",
                                type_info: "Option<InputSize>",
                                description: "Size variant (ExtraSmall, Small, Medium, Large)",
                            }
                            PropRow {
                                name: "value",
                                type_info: "Option<String>",
                                description: "Current input value",
                            }
                            PropRow {
                                name: "placeholder",
                                type_info: "Option<String>",
                                description: "Placeholder text",
                            }
                            PropRow {
                                name: "disabled",
                                type_info: "bool",
                                description: "Whether the input is disabled",
                            }
                            PropRow {
                                name: "readonly",
                                type_info: "bool",
                                description: "Whether the input is readonly",
                            }
                            PropRow {
                                name: "required",
                                type_info: "bool",
                                description: "Whether the input is required",
                            }
                            PropRow {
                                name: "label",
                                type_info: "Option<String>",
                                description: "Label text",
                            }
                            PropRow {
                                name: "help_text",
                                type_info: "Option<String>",
                                description: "Help text",
                            }
                            PropRow {
                                name: "error",
                                type_info: "Option<String>",
                                description: "Error message",
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