//! Button Components Module
//!
//! Provides styled button components with various types and configurations.

pub mod button;
pub mod button_showcase;
pub mod types;

// Core Components
pub use button::Button;

// Types
pub use types::{
    ButtonScheme,
    ButtonType,
    ButtonSize,
};

// Showcase/Demos
pub use button_showcase::render_button_showcase;

// Convenience Prelude
pub mod prelude {
    pub use super::{
        Button,
        ButtonProps,
        ButtonScheme,
        ButtonType,
        ButtonSize,
    };
}