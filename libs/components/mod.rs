// src/components/mod.rs


mod navbar;
pub use navbar::Navbar;

mod count;
pub use count::Count;

mod loading;
pub use loading::*;

pub mod layout;

pub mod sidebar;
pub use sidebar::Component_showcase;

// components/mod.rs
pub mod ui;

mod guide_component;
pub use guide_component::DogApp;