//! Card Components Module
//!
//! Provides flexible card components with header, body, and footer sections.
//!
//! # Example
//! ```
//! use crate::components::daisy_ui::prelude::*;
//!
//! Card {
//!     class: "shadow-lg".into(),
//!     children: rsx! {
//!         CardHeader { title: "My Card".into() }
//!         CardBody { "Card content here" }
//!         CardFoot { "Footer content" }
//!     }
//! }
//! ```

pub mod card;
pub mod card_showcase;
pub mod types;

// Core Components
pub use card::{Card, CardHeader, CardBody, CardFoot};

// Type Definitions
pub use types::{
    CardProps, 
    CardHeaderProps, 
    CardBodyProps, 
    CardFootProps
};

// Showcase
pub use card_showcase::render_card_showcase;

// Convenience Prelude
pub mod prelude {
    // Core Components
    pub use super::Card;
    pub use super::{CardHeader, CardBody, CardFoot};
    
    // Type Definitions
    pub use super::{
        CardProps,
        CardHeaderProps,
        CardBodyProps,
        CardFootProps
    };
    
    // Only include showcase in prelude if feature enabled
    pub use super::render_card_showcase;
}