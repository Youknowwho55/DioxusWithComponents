// use dioxus::prelude::*;
// use dioxus::prelude::*;

// #[derive(Debug, Clone, Routable, PartialEq)]
// #[rustfmt::skip]

// enum Route {
//     #[route("/")]
//     Home {},
//     #[route("/blog/:id")]
//     Blog { id: i32 },
//     #[route("/about")]
//     About {},
//     #[route("/contact")]
//     Contact {}
// }


// #[component]
// pub fn Navbar() -> Element {
//     rsx! {
//         document::Link { rel: "stylesheet" }

//         div { class: "text-xl font-bold flex items-center gap-4", id: "navbar",
//             Link { to: Route::Home {}, class: "hover:text-gray-300", "Home" }
//             Link { to: Route::Blog { id: 1 }, class: "hover:text-gray-300", "Blog" }
//             Link { to: Route::Contact {}, class: "hover:text-gray-300", "Contact" }
//             Link { to: Route::About {}, class: "hover:text-gray-300", "About" }
//         }

//         Outlet::<Route> {}
//     }
// }
