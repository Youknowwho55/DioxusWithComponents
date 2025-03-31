//! input Components Module
//!
//! Provides customizable input components with multiple variants including:
//! - Color schemes (primary, danger, warning, etc.)
//! - Sizes (xs, sm, md, lg)
//! - Types (input, submit, reset)
//! - Icon support
//!
//! # Example
//! ```
//! use crate::components::ui::input::prelude::*;
//!
//! input {
//!     input_scheme: inputScheme::Primary,
//!     input_size: inputSize::Large,
//!     "Click Me"
//! }
//! ```

pub mod input;
pub mod input_showcase;
pub mod types;

// Core Exports
pub use input::Input;
pub use types::{InputProps, InputScheme, InputType, InputSize};
// Showcase/Demos
pub use input_showcase::render_input_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::Input;
    
    // Type Definitions
    pub use super::{
        InputProps,
        InputScheme, 
        InputType,
        InputSize
    };
    
}

