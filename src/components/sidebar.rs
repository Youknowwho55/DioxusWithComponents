// use dioxus::prelude::*;
// use crate::components::daisy_ui::{Card, CardHeader, CardBody};
// #[derive(PartialEq, Clone, Debug)]
// pub enum ComponentShowcase {
//     Card,
//     // Add more components here
// }
//     // Add more components here


// #[component]
// pub fn component_showcase() -> Element {
//     let selected_component = use_signal(|| ComponentShowcase::Card);

//     rsx! {
//         div { class: "flex min-h-screen bg-base-200",
//             // Sidebar
//             div { class: "w-64 p-4 bg-base-300 h-screen sticky top-0",
//                 h2 { class: "text-xl font-bold mb-4", "Components" }
//                 ul { class: "menu",
//                     ComponentShowcase::iter().map(|component|) {
//                         rsx! {
//                             li {
//                                 key: "{component:?}",
//                                 a { 
//                                     class: if *selected_component.read() == component {
//                                         "active"
//                                     } else { "" },
//                                     onclick: move |_| selected_component.set(component.clone()),
//                                     "{component:?}"
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             // Main content area
//             div { class: "flex-1 p-8",
//                 match *selected_component.read() {
//                     ComponentShowcase::Card => rsx! {
//                         div { class: "max-w-2xl",
//                             Card {
//                                 CardHeader { title: "Sample Card" }
//                                 CardBody { "This is a card body." }
//                             }
//                         }
//                     },
//                 }
//             }
//         }
//     }
// }


