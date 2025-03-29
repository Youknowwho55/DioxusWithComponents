use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "flex flex-col items-center h-screen  py-20 relative overflow-y-auto w-full",
            h1 { class: "text-4xl mb-8 font-bold", "This is the Contact Component" }
            p { class: " mb-8", "Learn how to Contact us!" }
        }
    }
}
