use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet" }

        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            div { class: "flex flex-col items-center h-screen  py-20 relative overflow-y-auto w-full",
                h1 { class: "text-4xl mb-8 font-bold", "Hello, With Tailwind Blog" }
                p {
                    "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
                }

                // Navigation links
                Link { to: Route::Blog { id: id - 1 }, "Previous" }
                span { " <---> " }
                Link { to: Route::Blog { id: id + 1 }, "Next" }
            }
        }
    }}
