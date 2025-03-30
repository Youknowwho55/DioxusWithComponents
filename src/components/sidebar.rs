// src/components/sidebar.rs
use dioxus::prelude::*;
// Import all your components
use crate::components::ui::alert::render_alert_showcase;
use crate::components::ui::card::render_card_showcase;


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ComponentShowcase {
    Card,
    // Button,
    // Accordion,
    Alert,
    // Add more components as needed
}

#[component]
pub fn Component_showcase() -> Element {
    let mut selected_component = use_signal(|| ComponentShowcase::Card);
    
    // List of all available components for the sidebar
    let components = [
        ComponentShowcase::Card,
        // ComponentShowcase::Button,
        // ComponentShowcase::Accordion,
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
        // ComponentShowcase::Button => render_button_showcase(),
        // ComponentShowcase::Accordion => render_accordion_showcase(),
        ComponentShowcase::Alert => render_alert_showcase(),
    }

}




