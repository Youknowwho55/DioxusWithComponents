//learning/src/main.rs

mod views;

use dioxus::prelude::*;
use views::{Blog, Home, About, Contact}; // Import your route components

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/about")]
    About {},
    #[route("/contact")]
    Contact {}
}


#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet" }

        div { class: "text-xl font-bold flex items-center gap-4", id: "navbar",
            Link { to: Route::Home {}, class: "hover:text-gray-300", "Home" }
            Link { to: Route::Blog { id: 1 }, class: "hover:text-gray-300", "Blog" }
            Link { to: Route::Contact {}, class: "hover:text-gray-300", "Contact" }
            Link { to: Route::About {}, class: "hover:text-gray-300", "About" }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn Layout(children: Element) -> Element {
    // let current_year = Utc::now().year();

    rsx! {
        div { class: "flex flex-col",
            // Navbar
            nav { class: "w-full bg-gray-800 text-white p-4",
                div { class: "container mx-auto flex justify-between items-center",
                    div { class: "text-2xl font-bold", "My Dioxus App" }
                }
            }
            // Main content area
            main { class: "flex-grow py-2 px-4", {children} }
            // Footer
            footer { class: "w-full bg-gray-100 p-4 text-center",
                "© 2025 My Dioxus App. All rights reserved."
            }
        }
    }}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind-output.css"); // Updated path

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8080");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Layout { Router::<Route> {} }
    }
}