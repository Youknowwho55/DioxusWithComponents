//learning/src/main.rs
mod example; // This line includes the example.rs file
mod components;
mod views;

use dioxus::prelude::*;
use components::Navbar;
use views::{Blog, Home};
use example::comp; // Import the `comp` function from example.rs

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind-output.css"); // Updated path

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}

        // Call the `comp` function directly
        {comp()}
    }
}