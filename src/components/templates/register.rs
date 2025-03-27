
use dioxus::prelude::*;

#[component]
pub fn Error() -> Element {
    rsx! {
        section { class: "bg-gray-50 dark:bg-gray-900",
            div { class: "flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0",
                a {
                    href: "/",
                    class: "flex items-center mb-6 text-2xl font-semibold text-gray-900 dark:text-white",
                    img {
                        src: "https://flowbite.s3.amazonaws.com/blocks/marketing-ui/logo.svg",
                        alt: "logo",
                        class: "w-8 h-8 mr-2",
                    }
                    "\n            Mortgage CRM\n        "
                }
                div { class: "w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700",
                    div { class: "p-6 space-y-4 md:space-y-6 sm:p-8",
                        h1 { class: "text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white",
                            "\n                    Create Account\n                "
                        }
                        form {
                            method: "POST",
                            action: "?/signup",
                            class: "space-y-4 md:space-y-6",
                            div {
                                label {
                                    r#for: "firstName",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "First Name"
                                }
                                input {
                                    r#type: "text",
                                    name: "firstName",
                                    required: "false",
                                    placeholder: "First Name",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "firstName",
                                }
                            }
                            div {
                                label {
                                    r#for: "lastName",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Last Name"
                                }
                                input {
                                    name: "lastName",
                                    r#type: "text",
                                    placeholder: "Last Name",
                                    required: "false",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "lastName",
                                }
                            }
                            div {
                                label {
                                    r#for: "email",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Your email"
                                }
                                input {
                                    required: "false",
                                    placeholder: "name@company.com",
                                    r#type: "email",
                                    name: "email",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "email",
                                }
                            }
                            div {
                                label {
                                    r#for: "password",
                                    class: "block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                                    "Password"
                                }
                                input {
                                    required: "false",
                                    placeholder: "••••••••",
                                    r#type: "password",
                                    name: "password",
                                    class: "bg-gray-50 border border-gray-300 text-gray-900 rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                    id: "password",
                                }
                            }
                            button {
                                r#type: "submit",
                                class: "w-full text-white bg-blue-600 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800",
                                "Sign Up"
                            }
                            p { class: "text-sm font-light text-gray-500 dark:text-gray-400",
                                "\n                        Already have an account? "
                                a {
                                    href: "/auth",
                                    class: "font-medium text-primary-600 hover:underline dark:text-primary-500",
                                    "Log In"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}