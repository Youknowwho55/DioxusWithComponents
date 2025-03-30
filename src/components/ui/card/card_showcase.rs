use dioxus::prelude::*;
use super::{Card, CardBody, CardHeader, CardFoot};

pub fn render_card_showcase() -> Element {
    rsx! {
        div { class: "space-y-8 p-4",
            header { class: "mb-6",
                h1 { class: "text-3xl font-bold", "Card Component" }
                p { class: "text-gray-600 mt-2",
                    "A flexible container component with header, body, and footer sections."
                }
            }

            // Basic Example - using explicit conversion
            section { class: "space-y-4",
                h2 { class: "text-xl font-semibold", "Basic Card" }
                Card {
                    class: Some("shadow-md bg-white rounded-lg".to_string()),
                    drawer_trigger: None,
                    modal_trigger: None,
                    CardHeader {
                        title: "Basic Card Title".to_string(),
                        class: Some("border-b p-4".to_string()),
                    }
                    CardBody { class: Some("p-4".to_string()),
                        "This card demonstrates the basic structure with all sections."
                    }
                    CardFoot { class: Some("border-t p-4 bg-gray-50".to_string()), "Footer content" }
                }
            }

            // Interactive Example
            section { class: "space-y-4 mt-8",
                h2 { class: "text-xl font-semibold", "Interactive Card" }
                Card {
                    class: Some("shadow-lg hover:shadow-xl transition-shadow cursor-pointer".to_string()),
                    drawer_trigger: None,
                    modal_trigger: Some("example-modal".to_string()),
                    CardHeader {
                        title: "Clickable Card".to_string(),
                        class: Some("bg-blue-500 text-white p-4 rounded-t-lg".to_string()),
                    }
                    CardBody { class: Some("p-4".to_string()),
                        "This card triggers a modal when clicked. Try it!"
                    }
                }
            }
        
        // ... rest of your showcase code
        }
    }
}