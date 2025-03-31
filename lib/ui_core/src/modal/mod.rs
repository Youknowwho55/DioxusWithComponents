//! Modal Components Module
//!
//! Provides customizable modal components with multiple variants including:
//! - Color schemes (primary, danger, warning, etc.)
//! - Sizes (xs, sm, md, lg)
//! - Types (input, submit, reset)
//! - Icon support
//!
//! # Example
//! ```
//! use crate::components::ui::modal::prelude::*;
//!
//! input {
//!     input_scheme: inputScheme::Primary,
//!     input_size: inputSize::Large,
//!     "Click Me"
//! }
//! ```

pub mod modal;
pub mod modal_showcase;
pub mod types;

// Core Exports
pub use modal::Modal;
pub use types::{
    ModalProps,
    ModalActionProps,
    ModalBodyProps,

};
// Showcase/Demos
pub use modal_showcase::render_modal_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Component
    pub use super::Modal;
    
    // Type Definitions
    pub use super::{
        ModalProps,
        ModalScheme, 
        ModalSize,
        ModalType,
    };
    
}

