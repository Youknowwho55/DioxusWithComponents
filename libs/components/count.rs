#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Count() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| "...".to_string());

    rsx! {
        h1 { "High-Five counter: {count}" }
        div {
            button {
                class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| count += 1,
                "Up high!"
            }
            button {
                class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                onclick: move |_| count -= 1,
                "Down low!"
            }
        }
        button {
            class: "bg-yellow-500 hover:bg-yellow-800 text-white font-bold py-2 px-4 rounded",

            onclick: move |_| async move {
                if let Ok(data) = get_server_data().await {
                    println!("Client received: {}", data);
                    text.set(data.clone());
                    post_server_data(data).await.unwrap();
                }
            },
            "Run a server function"
        }
        "Server said: {text}"
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}