//! UI Components Module
//!
//! Reusable UI components following the design system.

// Component modules
pub mod accordion;
pub mod alert;
pub mod app_layout;
pub mod avatar;
pub mod blank_slate;
pub mod button;
pub mod card;
pub mod check_box;
pub mod drawer;
pub mod drop_down;
pub mod input;
pub mod label;
pub mod marketing;
pub mod modal;
pub mod nav_item;
pub mod pagination;
pub mod range;
pub mod relative_time;
pub mod select;
pub mod tab_container;
pub mod text_area;
pub mod time_line;
pub mod tooltip;

// Component exports grouped by category
pub mod components {
    // Layout
    pub use super::app_layout::AppLayout;
    
    // Navigation
    pub use super::nav_item::{NavGroup, NavItem, NavSubGroup, NavSubItem};
    pub use super::tab_container::{TabContainer, TabPanel};
    
    // Data Display
    pub use super::avatar::{Avatar, AvatarSize, AvatarType};
    pub use super::card::{Card, CardBody, CardFoot, CardHeader};
    pub use super::time_line::{TimeLine, TimeLineBadge, TimeLineBody};
    
    // Form Elements
    pub use super::button::{Button, ButtonScheme, ButtonSize, ButtonType};
    pub use super::check_box::{CheckBox, CheckBoxScheme, CheckBoxSize};
    pub use super::input::{Input, InputSize, InputType};
    pub use super::range::{Range, RangeColor};
    pub use super::select::{Select, SelectOption, SelectSize};
    pub use super::text_area::{TextArea, TextAreaSize};
    
    // Feedback
    pub use super::alert::{Alert, AlertColor, AlertIcon, WithButtons};
    pub use super::blank_slate::BlankSlate;
    pub use super::drawer::{Drawer, DrawerBody, DrawerFooter};
    pub use super::modal::{Modal, ModalAction, ModalBody};
    pub use super::tooltip::{ToolTip, ToolTipColor};
    
    // Utility
    pub use super::relative_time::{RelativeTime, RelativeTimeFormat};
}

// Type exports
pub mod types {
    pub use super::accordion::{Accordion, AccordionProps};
    pub use super::alert::{AlertColor, AlertIcon, WithButtons};
    pub use super::avatar::{AvatarSize, AvatarType};
    pub use super::button::{ButtonScheme, ButtonSize, ButtonType};
    pub use super::check_box::{CheckBoxScheme, CheckBoxSize};
    pub use super::drop_down::Direction;
    pub use super::input::{InputSize, InputType};
    pub use super::label::{LabelRole, LabelSize};
    pub use super::range::RangeColor;
    pub use super::select::SelectSize;
    pub use super::text_area::TextAreaSize;
    pub use super::tooltip::ToolTipColor;
}

// Convenience prelude
pub mod prelude {
    pub use super::components::*;
    pub use super::types::*;
}