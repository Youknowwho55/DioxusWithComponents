use crate::components::{Count, DogApp, Component_showcase};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Component_showcase {}
        DogApp {}
        Count {}
    }
}
