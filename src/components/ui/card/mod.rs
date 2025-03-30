// Main component
pub mod card;
// Showcase
pub mod card_showcase;

// Convenience re-exports
pub use card::{Card, CardBody, CardHeader, CardFoot};
pub use card_showcase::render_card_showcase;