use dioxus::prelude::*;
use super::{Alert, AlertColor, AlertIcon, WithButtons};
use crate::alert::types::{AlertProps, AlertVariant};

/// Showcase for the Alert component demonstrating all variants and configurations
pub fn render_alert_showcase() -> Element {
    rsx! {
        div { class: "space-y-6",
            // Title and description
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold", "Alert Component" }
                p { class: "text-gray-600 mt-2",
                    "Demonstration of all available Alert component variations and configurations."
                }
            }

            // Basic examples section
            section { class: "space-y-4",
                h2 { class: "text-xl font-semibold", "Basic Examples" }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Alert { alert_color: AlertColor::Default, "Default alert" }
                    Alert { alert_color: AlertColor::Info, "Info alert" }
                    Alert { alert_color: AlertColor::Success, "Success alert" }
                    Alert { alert_color: AlertColor::Warn, "Warning alert" }
                    Alert { alert_color: AlertColor::Error, "Error alert" }
                }
            }

            // Advanced examples section
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Advanced Examples" }
                Alert {
                    alert_color: AlertColor::Success,
                    with_buttons: WithButtons::Buttons,
                    "Success message with action buttons"
                }
                Alert {
                    alert_icon: AlertIcon::Warn,
                    alert_color: AlertColor::Warn,
                    class: "border-l-4 border-yellow-500".to_string(),
                    "Custom styled warning alert with icon"
                }
            }

            // Props documentation
            section { class: "mt-12 p-6 bg-gray-50 rounded-lg",
                h2 { class: "text-xl font-semibold mb-4", "Component API" }
                // Props table
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
                                name: "alert_color",
                                type_info: "AlertColor",
                                description: "Color variant (Default, Info, Success, Warn, Error)",
                            }
                            PropRow {
                                name: "alert_icon",
                                type_info: "AlertIcon",
                                description: "Optional icon (Default, Info, Success, Warn, Error)",
                            }
                            PropRow {
                                name: "with_buttons",
                                type_info: "WithButtons",
                                description: "Show action buttons (Default or Buttons)",
                            }
                            PropRow {
                                name: "class",
                                type_info: "String",
                                description: "Additional CSS classes",
                            }
                            PropRow {
                                name: "children",
                                type_info: "Element",
                                description: "Content to display inside the alert",
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Helper component for rendering prop documentation rows
#[component]
fn PropRow(name: String, type_info: String, description: String) -> Element {
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