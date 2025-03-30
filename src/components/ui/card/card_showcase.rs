use super::card::{Card, CardBody, CardHeader, CardFoot};
use dioxus::prelude::*;

// Card showcase component
pub fn render_card_showcase() -> Element {
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