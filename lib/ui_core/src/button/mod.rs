//! Button Components Module
//!
//! Provides customizable button components with multiple variants including:
//! - Color schemes (primary, danger, warning, etc.)
//! - Sizes (xs, sm, md, lg)
//! - Icon support
//!
//! # Example
//! ```
//! use crate::components::ui::button::prelude::*;
//!
//! Button {
//!     button_scheme: ButtonScheme::Primary,
//!     button_size: ButtonSize::Large,
//!     "Click Me"
//! }
//! ```

pub mod button;
pub mod button_showcase;
pub mod types;

// Core Exports
pub use button::Button;
pub use types::{ButtonProps, ButtonScheme, ButtonType, ButtonSize};

// Showcase/Demos
pub use button_showcase::render_button_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::Button;
    
    // Type Definitions
    pub use super::{
        ButtonProps,
        ButtonScheme, 
        ButtonType,
        ButtonSize
    };
    
}