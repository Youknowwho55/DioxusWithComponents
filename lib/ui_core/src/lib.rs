#![warn(missing_docs)]

//! UI Components Module
//! 
//! Reusable UI components following the design system.

// =============================================
// Component Module Declarations
// =============================================

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

// =============================================
// Public API Organization
// =============================================

/// Primary components grouped by category
pub mod components {
    // Layout components
    pub use super::app_layout::AppLayout;
    
    // Navigation components
    pub use super::nav_item::{NavGroup, NavItem, NavSubGroup, NavSubItem};
    pub use super::tab_container::{TabContainer, TabPanel};
    
    // Data Display components
    pub use super::avatar::{Avatar, AvatarSize, AvatarType};
    pub use super::card::{Card, CardBody, CardFoot, CardHeader};
    pub use super::time_line::{TimeLine, TimeLineBadge, TimeLineBody};
    
    // Form components
    pub use super::button::{Button, ButtonScheme, ButtonSize, ButtonType};
    pub use super::input::{Input, InputSize, InputType};
    pub use super::range::{Range, RangeColor};
    pub use super::select::{Select, SelectOption, SelectSize};
    pub use super::text_area::{TextArea, TextAreaSize};
    
    // Feedback components
    pub use super::alert::{Alert, AlertColor, AlertIcon, WithButtons};
    pub use super::blank_slate::BlankSlate;
    pub use super::drawer::{Drawer, DrawerBody, DrawerFooter};
    pub use super::modal::{Modal, ModalAction, ModalBody};
    pub use super::tooltip::{ToolTip, ToolTipColor};
    
    // Utility components
    pub use super::relative_time::{RelativeTime, RelativeTimeFormat};
    
    /// Component showcase utilities for documentation/demo purposes
    pub mod showcase {
        pub use super::super::{
            accordion::render_accordion_showcase,
            alert::render_alert_showcase,
            button::render_button_showcase,
            card::render_card_showcase,
            input::render_input_showcase,
        };
    }
}

/// Common types used across components
pub mod types {
    pub use super::accordion::AccordionProps;
    pub use super::alert::{AlertColor, AlertIcon, WithButtons};
    pub use super::avatar::{AvatarSize, AvatarType};
    pub use super::button::{ButtonScheme, ButtonSize, ButtonType};
    pub use super::drop_down::Direction;
    pub use super::input::{InputProps, InputScheme, InputSize, InputType};
    pub use super::label::{LabelRole, LabelSize};
    pub use super::range::RangeColor;
    pub use super::select::SelectSize;
    pub use super::text_area::TextAreaSize;
    pub use super::tooltip::ToolTipColor;
}

/// Convenience prelude for importing commonly used items
pub mod prelude {
    pub use super::components::*;
    pub use super::types::*;
}

// =============================================
// Top-level Re-exports
// =============================================

// Core components
pub use accordion::Accordion;
pub use alert::Alert;
pub use button::Button;
pub use card::Card;
pub use input::Input;

// Commonly used types
pub use input::types::{InputScheme, InputSize, InputType};