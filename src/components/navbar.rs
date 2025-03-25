use crate::Route;
use dioxus::prelude::*;


#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet" }

        div { class: "text-4xl px-8 font-bold", id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
