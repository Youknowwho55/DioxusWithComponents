//learning/src/main.rs
mod components;
mod views;

use dioxus::prelude::*;
use components::Navbar;
use views::{Blog, Home, About, Contact}; // Import your route components
use components::layout::Layout;

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