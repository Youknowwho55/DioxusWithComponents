use dioxus::prelude::*;

use crate::components::daisy_ui::{
    Accordian, Alert, AlertColor, 
    AppLayout, 
    Avatar, AvatarSize, AvatarType, 
    BlankSlate, 
    Button, ButtonScheme, ButtonSize, ButtonType,
    Card, CardHeader, CardBody,
    CheckBox, CheckBoxSize, CheckBoxScheme,
    Drawer, DrawerBody,
    DropDown, DropDownLink,
    Input, InputSize, InputType,
    Label, LabelRole, LabelSize,
    Modal, ModalBody,
    NavItem,
    Pagination,
    Range, RangeColor,
    RelativeTime, RelativeTimeFormat,
    Select, SelectSize, SelectOption,
    TabContainer, TabPanel,
    TextArea, TextAreaSize,
    TimeLine, TimeLineBadge, TimeLineBody,
    ToolTip, ToolTipColor
};
#[derive(PartialEq, Clone)]
pub enum Componentshowcase {

    Card,

}


#[component]
pub fn ComponentShowcase() -> Element {
    let current_component_index = use_signal(|| 0);

    let components = vec![ComponentShowcase::Card];

    let current_component = components[current_component_index()]; // Copying the enum instead of borrowing

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-base-200 p-4",

            // Navigation Buttons
            div { class: "flex space-x-4 mb-8",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        current_component_index.set((current_component_index() + 1) % components.len());
                    },
                    "Next Component ➡️"
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| {
                        current_component_index
                            .set((current_component_index() + components.len() - 1) % components.len());
                    },
                    "⬅️ Previous Component"
                }
            }

            // Component Rendering
            match current_component {
                ComponentShowcase::Card => rsx! {
                    Card {
                        CardHeader { title: "Sample Card" }
                        CardBody { "This is a card body." }
                    }
                },
            }
        }
    }
}