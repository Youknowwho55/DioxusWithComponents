use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }

            div { class: "flex flex-col items-center bg-red-700 text-white py-20 relative overflow-y-auto w-full",
                h1 { class: "text-s text-gray-100 mb-8 ", "Hello, With Tailwind Home" }
                p { class: "text-xl font-medium text-black dark:text-white", "This is another test" }
                div { id: "links",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                        "💫 VSCode Extension"
                    }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                }
            }
        }
    }
}