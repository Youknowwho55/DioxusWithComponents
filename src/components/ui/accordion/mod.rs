//! Accordion Components Module
//!
//! Provides customizable accordion components with multiple behavior variants:
//! - **Default**: Standard checkbox behavior with no indicator
//! - **Arrow**: Checkbox behavior with arrow indicator
//! - **Plus**: Checkbox behavior with plus/minus indicator
//! - **Radio**: Radio behavior (single open) with arrow indicator
//! - **RadioPlus**: Radio behavior (single open) with plus/minus indicator
//!
//! # Examples
//! ```rust
//! use crate::components::daisy_ui::accordion::prelude::*;
//!
//! // Basic accordion
//! Accordion {
//!     name: "faq-group".to_string(),
//!     title: "How does this work?".to_string(),
//!     children: rsx! { p { "Detailed explanation here..." } }
//! }
//!
//! // Radio-style accordion group
//! Accordion {
//!     name: "settings-group".to_string(),
//!     title: "Notification Settings".to_string(),
//!     accordion_type: AccordionType::Radio,
//!     children: rsx! { NotificationSettingsForm {} }
//! }
//! ```

pub mod accordion;
pub mod accordion_showcase;
pub mod types;

// Core Components
pub use accordion::Accordion;

// Types
pub use types::{
    AccordionProps,
    AccordionType,
};

// Showcase/Demos
pub use accordion_showcase::render_accordion_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::Accordion;
    
    // Type Definitions
    pub use super::{
        AccordionProps,
        AccordionType,
    };
    

}