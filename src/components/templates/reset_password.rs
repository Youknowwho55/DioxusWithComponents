use dioxus::prelude::*;

#[component]
pub fn Reset() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full",
            h2 { class: "mt-2 text-center text-3xl font-bold tracking-tight text-base-content",
                "Reset Your Password"
            }
            p { class: "text-center mt-1",
                "We'll send you an email with a link to reset your password."
            }
            form {
                action: "?/resetPassword",
                method: "POST",
                class: "flex flex-col items-center space-y-2 w-full max-w-md pt-4",
                div { class: "form-control w-full",
                    label { r#for: "email", class: "label font-medium pb-1",
                        span { class: "label-text", "Email" }
                    }
                    input {
                        r#type: "email",
                        name: "email",
                        class: "input input-bordered w-full",
                        placeholder: "Enter your email",
                    }
                }
                div { class: "w-full pt-2",
                    button { class: "btn btn-primary w-full", "Request Password Reset" }
                }
            }
        }
    }
}