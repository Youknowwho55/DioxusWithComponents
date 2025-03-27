use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Component {
    Accordion,
    Alert,
    AppLayout,
    Avatar,
    BlankSlate,
    Button,
    Card,
    CheckBox,
    Drawer,
    DropDown,
    Input,
    Label,
    Modal,
    NavItem,
    Pagination,
    Range,
    RelativeTime,
    Select,
    TabContainer,
    TextArea,
    TimeLine,
    Tooltip,
    // Add more components as needed
}

pub fn comp() -> Element {
    // Use a signal to track the current component index
    let mut current_component_index = use_signal(|| 0);

    // Define the list of components
    let components = vec![
        Component::Accordion,
        Component::Alert,
        Component::AppLayout,
        Component::Avatar,
        Component::BlankSlate,
        Component::Button,
        Component::Card,
        Component::CheckBox,
        Component::Drawer,
        Component::DropDown,
        Component::Input,
        Component::Label,
        Component::Modal,
        Component::NavItem,
        Component::Pagination,
        Component::Range,
        Component::RelativeTime,
        Component::Select,
        Component::TabContainer,
        Component::TextArea,
        Component::TimeLine,
        Component::Tooltip,
    ];



    // Clone `components` for each closure
    let components_next = components.clone();
    let components_prev = components.clone();

    // Get the current component based on the index
    let current_component = &components[current_component_index()];

    rsx! {
        div {
            // Button to cycle to the next component
            button {
                class: "px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors duration-300 ease-in-out shadow-md active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 mx-2",
                onclick: move |_| {
                    current_component_index
                        .set((current_component_index() + 1) % components_next.len());
                },
                "Next"
            }
            // Button to cycle to the previous component
            button {
                class: "px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors duration-300 ease-in-out shadow-md active:scale-95 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 mx-2",
                onclick: move |_| {
                    current_component_index
                        .set(
                            (current_component_index() + components_prev.len() - 1)
                                % components_prev.len(),
                        );
                },
                "Previous"
            }

            // Render the current component
            div {
                match current_component {
                    Component::Accordion => rsx! {
                        div { "Accordion Component" }
                    },
                    Component::Alert => rsx! {
                        div { "Alert Component" }
                    },
                    Component::AppLayout => rsx! {
                        div { "App Layout Component" }
                    },
                    Component::Avatar => rsx! {
                        div { "Avatar Component" }
                    },
                    Component::BlankSlate => rsx! {
                        div { "Blank Slate Component" }
                    },
                    Component::Button => rsx! {
                        div { "Button Component" }
                    },
                    Component::Card => rsx! {
                        div { "Card Component" }
                    },
                    Component::CheckBox => rsx! {
                        div { "CheckBox Component" }
                    },
                    Component::Drawer => rsx! {
                        div { "Drawer Component" }
                    },
                    Component::DropDown => rsx! {
                        div { "DropDown Component" }
                    },
                    Component::Input => rsx! {
                        div { "Input Component" }
                    },
                    Component::Label => rsx! {
                        div { "Label Component" }
                    },
                    Component::Modal => rsx! {
                        div { "Modal Component" }
                    },
                    Component::NavItem => rsx! {
                        div { "NavItem Component" }
                    },
                    Component::Pagination => rsx! {
                        div { "Pagination Component" }
                    },
                    Component::Range => rsx! {
                        div { "Range Component" }
                    },
                    Component::RelativeTime => rsx! {
                        div { "RelativeTime Component" }
                    },
                    Component::Select => rsx! {
                        div { "Select Component" }
                    },
                    Component::TabContainer => rsx! {
                        div { "TabContainer Component" }
                    },
                    Component::TextArea => rsx! {
                        div { "TextArea Component" }
                    },
                    Component::TimeLine => rsx! {
                        div { "TimeLine Component" }
                    },
                    Component::Tooltip => rsx! {
                        div { "Tooltip Component" }
                    },
                }
            }
        }
    }
}