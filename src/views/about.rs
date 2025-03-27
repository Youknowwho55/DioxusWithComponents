use crate::components::{Echo, Hero};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "flex flex-col items-center h-screen  py-20 relative overflow-y-auto w-full",
            h1 { class: "text-4xl mb-8 font-bold", "This is the About Component" }
            p { class: " mb-8 ", "Learn more About us" }
        }
    }
}