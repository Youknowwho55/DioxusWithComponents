//! Check Box Components Module
//!
//! Provides customizable check box components with multiple variants including:
//! - Color schemes (primary, danger, warning, etc.)
//! - Sizes (xs, sm, md, lg)
//! )
//! - Icon support
//!
//! # Example
//! ```
//! use crate::components::ui::button::prelude::*;
//!
//! CheckBox {
//!     check_box_scheme: CheckBoxScheme::Primary,
//!     check_box_size: CheckBoxSize::Large,
//!     "Click Me"
//! }
//! ```

pub mod check_box;
pub mod check_box_showcase;
pub mod types;

// Core Exports
pub use check_box::CheckBox;
pub use types::{
    CheckBoxProps,
    CheckBoxScheme,
    CheckBoxSize,
};

// Showcase/Demos
pub use check_box_showcase::render_check_box_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::CheckBox;
    
    // Type Definitions
    pub use super::{
        CheckBoxProps,
        CheckBoxScheme, 
        CheckBoxSize
    };
    
}