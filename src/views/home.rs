use crate::components::{Count, Hero, DogApp, Component_showcase};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Component_showcase {}
        DogApp {}
        Count {}
    }
}
