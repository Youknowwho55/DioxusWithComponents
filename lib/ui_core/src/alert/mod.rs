//! Alert Components Module
//!
//! Provides styled alert components with various types and configurations.

pub mod alert;
pub mod types;

// Core Components
pub use alert::Alert;

// Types
pub use types::{
    AlertProps, AlertVariant, AlertStatus, AlertIcon,
};

pub mod prelude {
    pub use super::{Alert, AlertIcon, AlertProps, AlertStatus, AlertVariant};
}

