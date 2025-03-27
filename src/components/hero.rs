use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { class: "flex flex-col items-center relative overflow-y-auto w-full",
                h1 { class: "text-s text-red-700 mb-8 ", "Hello, With Tailwind Home" }
                p { class: "text-xl font-medium text-black dark:text-white", "This is another test" }
                div {
                    id: "links",
                    class: "flex flex-col items-center justify-center space-y-2 p-4",
                    a {
                        href: "https://dioxuslabs.com/learn/0.6/",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "📚 Learn Dioxus"
                    }
                    a {
                        href: "https://dioxuslabs.com/awesome",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "🚀 Awesome Dioxus"
                    }
                    a {
                        href: "https://github.com/dioxus-community/",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "📡 Community Libraries"
                    }
                    a {
                        href: "https://github.com/DioxusLabs/sdk",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "⚙️ Dioxus Development Kit"
                    }
                    a {
                        href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "💫 VSCode Extension"
                    }
                    a {
                        href: "https://discord.gg/XgGxMSkvUM",
                        class: "text-blue-600 hover:text-blue-800 transition-colors text-lg text-center",
                        "👋 Community Discord"
                    }
                }
            }
        }
    }
}