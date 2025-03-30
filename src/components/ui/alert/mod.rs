//! Alert Components Module
//!
//! Provides styled alert components with various types and configurations.

pub mod alert;
pub mod alert_showcase;
pub mod types;

// Core Components
pub use alert::Alert;

// Types
pub use types::{AlertColor, AlertIcon, WithButtons};

// Showcase/Demos
pub use alert_showcase::render_alert_showcase;

// Convenience Prelude
pub mod prelude {
    pub use super::{Alert, AlertColor, AlertIcon, WithButtons};
}