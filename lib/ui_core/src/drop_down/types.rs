use dioxus::prelude::*;


#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    #[default]
    None,
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::None => "",
            Direction::Top => "dropdown-top",
            Direction::Bottom => "dropdown-bottom",
            Direction::Left => "dropdown-left",
            Direction::Right => "dropdown-right",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownProps {
    children: Element,
    carat: Option<bool>,
    button_text: String,
    class: Option<String>,
    direction: Option<Direction>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownLinkProps {
    href: String,
    target: Option<String>,
    drawer_trigger: Option<String>,
    class: Option<String>,
    children: Element,
}
