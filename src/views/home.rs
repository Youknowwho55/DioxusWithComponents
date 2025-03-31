use crate::views::{
    Count,
    component_showcase::Component_showcase,
    DogApp,
};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Component_showcase {}
        DogApp {}
        Count {}
    }
}
