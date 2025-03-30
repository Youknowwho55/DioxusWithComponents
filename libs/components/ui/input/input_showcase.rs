use dioxus::prelude::*;
use super::{Input, InputScheme, InputType, InputSize};

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
                        name: "text-input",
                        input_type: InputType::Text,
                        placeholder: "Text input",
                    }
                    Input {
                        name: "email-input",
                        input_type: InputType::Email,
                        placeholder: "Email input",
                    }
                    Input {
                        name: "password-input",
                        input_type: InputType::Password,
                        placeholder: "Password input",
                    }
                    Input {
                        name: "number-input",
                        input_type: InputType::Number,
                        placeholder: "Number input",
                    }
                    Input {
                        name: "date-input",
                        input_type: InputType::Date,
                        placeholder: "Date input",
                    }
                    Input { name: "color-input", input_type: InputType::Color }
                    Input {
                        name: "file-input",
                        input_type: InputType::File,
                        class: "block w-full",
                    }
                }
            }

            // Schemes Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Color Schemes" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "default-input",
                        input_scheme: InputScheme::Default,
                        placeholder: "Default scheme",
                    }
                    Input {
                        name: "primary-input",
                        input_scheme: InputScheme::Primary,
                        placeholder: "Primary scheme",
                    }
                    Input {
                        name: "success-input",
                        input_scheme: InputScheme::Success,
                        placeholder: "Success scheme",
                    }
                    Input {
                        name: "warn-input",
                        input_scheme: InputScheme::Warning,
                        placeholder: "Warning scheme",
                    }
                    Input {
                        name: "danger-input",
                        input_scheme: InputScheme::Danger,
                        placeholder: "Danger scheme",
                    }
                }
            }

            // Sizes Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Size Variants" }
                div { class: "grid grid-cols-1 gap-4",
                    Input {
                        name: "xs-input",
                        input_size: InputSize::ExtraSmall,
                        placeholder: "Extra small input",
                    }
                    Input {
                        name: "small-input",
                        input_size: InputSize::Small,
                        placeholder: "Small input",
                    }
                    Input {
                        name: "medium-input",
                        input_size: InputSize::Medium,
                        placeholder: "Medium input",
                    }
                    Input {
                        name: "large-input",
                        input_size: InputSize::Large,
                        placeholder: "Large input",
                    }
                }
            }

            // States Section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Input States" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "disabled-input",
                        disabled: true,
                        value: "Disabled input",
                    }
                    Input {
                        name: "readonly-input",
                        readonly: true,
                        value: "Readonly input",
                    }
                    Input {
                        name: "required-input",
                        required: true,
                        placeholder: "Required input",
                    }
                    Input {
                        name: "autofocus-input",
                        autofocus: true,
                        placeholder: "Autofocused input",
                    }
                }
            }

            // Validation Examples
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Validation Examples" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "minmax-input",
                        input_type: InputType::Number,
                        min: "5",
                        max: "10",
                        placeholder: "Number between 5-10",
                    }
                    Input {
                        name: "pattern-input",
                        pattern: "[A-Za-z]{3}",
                        placeholder: "Exactly 3 letters",
                    }
                    Input {
                        name: "length-input",
                        minlength: "3",
                        maxlength: "6",
                        placeholder: "3-6 characters",
                    }
                }
            }

            // With Labels and Helpers
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "With Labels & Helpers" }
                div { class: "grid grid-cols-1 gap-4",
                    Input {
                        name: "labeled-input",
                        label: "Username",
                        placeholder: "Enter your username",
                    }
                    Input {
                        name: "help-text-input",
                        label: "Password",
                        help_text: "Must be at least 8 characters",
                        input_type: InputType::Password,
                    }
                    Input {
                        name: "error-input",
                        label: "Email",
                        error: "Please enter a valid email address",
                        input_scheme: InputScheme::Danger,
                    }
                }
            }

            // With Prefix/Suffix
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "With Prefix/Suffix" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Input {
                        name: "prefix-input",
                        prefix: rsx! {
                            span { class: "px-3 text-gray-500", "$" }
                        },
                        placeholder: "Amount",
                    }
                    Input {
                        name: "suffix-input",
                        suffix: rsx! {
                            span { class: "px-3 text-gray-500", "kg" }
                        },
                        placeholder: "Weight",
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
                                name: "input_type",
                                type_info: "Option<InputType>",
                                description: "Input type (Text, Email, Password, etc.)",
                            }
                            PropRow {
                                name: "input_scheme",
                                type_info: "Option<InputScheme>",
                                description: "Color variant (Default, Primary, Success, etc.)",
                            }
                            PropRow {
                                name: "input_size",
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
                            PropRow {
                                name: "prefix",
                                type_info: "Option<Element>",
                                description: "Prefix element (icon or text)",
                            }
                            PropRow {
                                name: "suffix",
                                type_info: "Option<Element>",
                                description: "Suffix element (icon or text)",
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