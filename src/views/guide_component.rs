
use dioxus::prelude::*;


#[component]
pub fn DogApp() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            p { id: "dogview", "Doggo" }
        }
        div { id: "buttons",
            button { class: "px-3 mx-2", id: "skip", "skip" }
            button { class: "px-3 mx-2", id: "save", "save!" }
        }
    }
}


