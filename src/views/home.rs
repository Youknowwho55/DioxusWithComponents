use crate::components::{Echo, Hero, DogApp};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        // ComponentShowcase {}
        DogApp {}
        Echo {}
    }
}
