use dioxus::prelude::*;
// use chrono::Utc;

#[component]
pub fn Layout(children: Element) -> Element {
    // let current_year = Utc::now().year();

    rsx! {
        div { class: "flex flex-col",
            // Navbar
            nav { class: "w-full bg-gray-800 text-white p-4",
                div { class: "container mx-auto flex justify-between items-center",
                    div { class: "text-2xl font-bold", "My Dioxus App" }
                }
            }
            // Main content area
            main { class: "flex-grow container mx-auto py-2 px-4", {children} }
            // Footer
            footer { class: "w-full bg-gray-100 p-4 text-center",
                "© 2025 My Dioxus App. All rights reserved."
            }
        }
    }}
