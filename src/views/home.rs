use crate::components::{Echo, Hero, Comp};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
        Comp {}
    }
}
