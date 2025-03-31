// use dioxus::prelude::*;
// use ui_core::Input;  // Import from your UI core library
// use ui_core::types::InputType;  // For InputType enum

// #[component]
// pub fn Login(
//     form: Option<LoginForm>,  // Your form data type
//     loading: bool,
//     onsubmit: EventHandler<LoginData>,  // Your submit handler
// ) -> Element {
//     rsx! {
//         div { class: "flex flex-col items-center h-full w-full",
//             h2 { class: "mt-2 text-center text-3xl font-bold tracking-tight text-base-content",
//                 "Login to your account"
//             }
//             p { class: "text-center mt-1",
//                 "Or "
//                 a {
//                     href: "/register",
//                     class: "text-primary font-medium hover:cursor-pointer hover:underline",
//                     "register"
//                 }
//                 " if you don't already have an account."
//             }
//             form {
//                 class: "flex flex-col items-center space-y-2 w-full pt-4",
//                 prevent_default: "onsubmit",
//                 onsubmit: move |evt| {
//                     let email = evt.data.values().get("email").map(|v| v.as_str().to_owned());
//                     let password = evt.data.values().get("password").map(|v| v.as_str().to_owned());
//                     onsubmit.call(LoginData { email, password });
//                 },
//                 Input {
//                     name: "email".to_string(),
//                     r#type: InputType::Email,
//                     label: Some("Email".to_string()),
//                     value: form.as_ref().and_then(|f| f.data.email.clone()),
//                     error: form.as_ref().and_then(|f| f.errors.email.clone()),
//                     disabled: loading,
//                 }
//                 Input {
//                     name: "password".to_string(),
//                     r#type: InputType::Password,
//                     label: Some("Password".to_string()),
//                     value: form.as_ref().and_then(|f| f.data.password.clone()),
//                     error: form.as_ref().and_then(|f| f.errors.password.clone()),
//                     disabled: loading,
//                 }
//                 div { class: "w-full max-w-lg",
//                     a {
//                         href: "/reset-password",
//                         class: "font-medium text-primary hover:cursor-pointer hover:underline",
//                         "Forgot Password?"
//                     }
//                 }
//                 div { class: "w-full max-w-lg pt-2",
//                     button {
//                         r#type: "submit",
//                         disabled: loading,
//                         class: "btn btn-primary w-full",
//                         "Login"
//                     }
//                 }
//                 // Error message (conditionally shown)
//                 if let Some(form) = &form {
//                     if form.show_verification_error {
//                         div { class: "alert alert-error shadow-lg w-full max-w-lg",
//                             div {
//                                 svg {
//                                     view_box: "0 0 24 24",
//                                     fill: "none",
//                                     xmlns: "http://www.w3.org/2000/svg",
//                                     class: "stroke-current flex-shrink-0 h-6 w-6",
//                                     path {
//                                         d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
//                                         stroke_linejoin: "round",
//                                         stroke_width: "2",
//                                         stroke_linecap: "round",
//                                     }
//                                 }
//                                 span { "You must verify your email before you can login." }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// // Example form data structures (customize as needed)
// #[derive(Clone, PartialEq)]
// pub struct LoginForm {
//     pub data: LoginData,
//     pub errors: LoginErrors,
//     pub show_verification_error: bool,
// }

// #[derive(Clone, PartialEq)]
// pub struct LoginData {
//     pub email: Option<String>,
//     pub password: Option<String>,
// }

// #[derive(Clone, PartialEq)]
// pub struct LoginErrors {
//     pub email: Option<String>,
//     pub password: Option<String>,
// }