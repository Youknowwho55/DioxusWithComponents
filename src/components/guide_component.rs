
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


// All Props structs in Dioxus need to derive the Properties trait which requires both Clone and PartialEq
/*
#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}
    */

// When Dioxus re-renders your component, it compares the Element returned from the last render 
// against the Element returned in the current render.

// Dioxus will re-render your component in only two circumstances:

// 1. When the Props change as determined by PartialEq
// 2. When a function like signal.set() or signal.write() calls Scope.needs_update()


// Any expression that can be rendered to a String can be included directly in RSX. 
// RSX also accepts Option<Element> and iterators of Elements:

/* 
rsx! {
    // Anything that's `Display`
    {"Something"}

    // Optionals
    {show_title.then(|| rsx! { "title!" } )}

    // And iterators
    ul {
        {(0..5).map(|i| rsx! { "{i}" })}
    }
}
*/