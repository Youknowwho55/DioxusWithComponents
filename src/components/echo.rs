use dioxus::prelude::*;
use serde::{Serialize, Deserialize};

// Ensure you have these dependencies in Cargo.toml:
// dioxus = { version = "0.4", features = ["fullstack"] }
// serde = { version = "1.0", features = ["derive"] }

#[derive(Serialize, Deserialize, Clone)]
struct EchoPayload {
    message: String,
}

#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    // Simple server-side echo function
    Ok(input)
}

#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());
    let mut error = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "max-w-md mx-auto p-6 bg-white shadow-md rounded-lg",
            h4 { class: "text-2xl font-bold mb-4 text-gray-800", "ServerFn Echo" }
            input {
                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all duration-300",
                placeholder: "Type here to echo...",
                oninput: move |event| {
                    let input_value = event.value();
                    spawn(async move {
                        match echo_server(input_value).await {
                            Ok(data) => {
                                response.set(data);
                                error.set(None);
                            }
                            Err(e) => {
                                error.set(Some(e.to_string()));
                                response.set(String::new());
                            }
                        }
                    });
                },
            }
            if let Some(err) = error() {
                p { class: "text-red-500 mt-2", "Error: {err}" }
            }
            if !response().is_empty() {
                p { class: "mt-2 text-green-600",
                    "Server echoed: "
                    i { class: "font-semibold", "{response}" }
                }
            }
        }
    }
}