use dioxus::prelude::*;

#[component]
pub fn Error() -> Element {
    rsx! {
        div { class: "flex flex-col items-center h-full w-full",
            h2 { class: "mt-2 text-center text-3xl font-bold tracking-tight text-base-content",
                "\n\t\tLogin to your account\n\t"
            }
            p { class: "text-center mt-1",
                "\n\t\tOr "
                a {
                    href: "/register",
                    class: "text-primary font-medium hover:cursor-pointer hover:underline",
                    "register"
                }
                " if you don't already have an account.\n\t"
            }
            form {
                "use:enhance": "{submitLogin}",
                method: "POST",
                action: "?/login",
                class: "flex flex-col items-center space-y-2 w-full pt-4",
                "\n\t\t<Input\n\t\t\ttype=\"email\"\n\t\t\tid=\"email\"\n\t\t\tlabel=\"Email\"\n\t\t\tvalue={form?.data?.email ?? ''}\n\t\t\terrors={form?.errors?.email}\n\t\t\tdisabled={loading}\n\t\t/>\n\t\t"
                input {
                    "errors": "{form?.errors?.password}",
                    label: "Password",
                    r#type: "password",
                    disabled: "{loading}",
                    id: "password",
                }
                div { class: "w-full max-w-lg",
                    a {
                        href: "/reset-password",
                        class: "font-medium text-primary hover:cursor-pointer hover:underline",
                        "\n\t\t\t\tForgot Password?"
                    }
                }
                div { class: "w-full max-w-lg pt-2",
                    button {
                        r#type: "submit",
                        disabled: "{loading}",
                        class: "btn btn-primary w-full",
                        "Login"
                    }
                }
                div { class: "alert alert-error shadow-lg w-full max-w-lg",
                    div {
                        svg {
                            "viewBox": "0 0 24 24",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "stroke-current flex-shrink-0 h-6 w-6",
                            path {
                                d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                            }
                        }
                        span { "You must verify your email before you can login." }
                    }
                }
            }
        }}
}