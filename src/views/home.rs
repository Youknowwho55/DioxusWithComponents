use crate::components::{Echo, Hero, ComponentShowcase};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        ComponentShowcase {}
        Echo {}
    }
}
