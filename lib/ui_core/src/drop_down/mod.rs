//! Drop Down Components Module
//!
//! Provides customizable drop sown components with multiple variants including:
//! - Color schemes (primary, danger, warning, etc.)
//! - Sizes (xs, sm, md, lg)
//! )
//! - Icon support
//!
//! # Example
//! ```
//! use crate::components::ui::drop_down::prelude::*;
//!
//! DropDown {
//!     drop_down_scheme: DropDownScheme::Primary,
//!     drop_down_size: DropDownSize::Large,
//!     "Click Me"
//!
//! }
//! ```

pub mod drop_down;
pub mod drop_down_showcase;
pub mod types;

// Core Exports
pub use drop_down::DropDown;
pub use types::{
    DropDownProps,
    DropDownScheme,
    DropDownSize,
    Direction,
    DropDownLinkProps,
};

// Showcase/Demos
pub use drop_down_showcase::render_drop_down_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::DropDown;
    
    // Type Definitions
    pub use super::{
        DropDownProps,
        DropDownScheme, 
        DropDownSize
    };
    
}