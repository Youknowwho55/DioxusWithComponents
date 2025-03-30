// tests/components/input.rs
// Import your crate (your package name in Cargo.toml)
use my_dioxus_app::components::ui::input::{Input, InputProps};
use my_dioxus_app::components::ui::input::types::{InputScheme, InputSize, InputType};
use dioxus::prelude::*;
use dioxus_ssr::render;

// Basic rendering test
#[test]
fn renders_with_required_props() {
    let props = InputProps {
        name: "test_input".to_string(),  // Required prop
        ..Default::default()
    };
    
    let rendered = render(Input(props));
    assert!(rendered.contains("name=\"test_input\""));
}

// Test all variants systematically
#[test]
fn handles_all_scheme_variants() {
    let schemes = [
        InputScheme::Default,
        InputScheme::Primary,
        InputScheme::Success,
        InputScheme::Warning,
        InputScheme::Danger,
        InputScheme::Info,
    ];
    
    for scheme in schemes {
        let props = InputProps {
            name: "scheme_test".to_string(),
            scheme: Some(scheme),
            ..Default::default()
        };
        
        let rendered = render(Input(props));
        assert!(rendered.contains(scheme.to_classes().split(' ').next().unwrap()));
    }
}

// Test prop combinations
#[test]
fn disabled_input_renders_correctly() {
    let props = InputProps {
        name: "disabled_test".to_string(),
        disabled: true,
        ..Default::default()
    };
    
    let rendered = render(Input(props));
    assert!(rendered.contains("disabled"));
    assert!(rendered.contains("cursor-not-allowed"));
}

// Test event handlers (mock approach)
#[test]
fn handles_event_callbacks() {
    let mut clicked = false;
    let props = InputProps {
        name: "event_test".to_string(),
        onchange: Some(EventHandler::new(move |_| {
            clicked = true;
        })),
        ..Default::default()
    };
    
    // In a real test, you'd simulate an event
    // This just verifies the handler is attached
    let rendered = render(Input(props));
    assert!(rendered.contains("onchange"));
}

// Test DOM structure
#[test]
fn includes_label_when_provided() {
    let props = InputProps {
        name: "label_test".to_string(),
        label: Some("Test Label".to_string()),
        ..Default::default()
    };
    
    let rendered = render(Input(props));
    assert!(rendered.contains("Test Label"));
    assert!(rendered.contains("<label"));
}

// Test accessibility attributes
#[test]
fn includes_aria_attributes() {
    let props = InputProps {
        name: "aria_test".to_string(),
        required: true,
        ..Default::default()
    };
    
    let rendered = render(Input(props));
    assert!(rendered.contains("required"));
    assert!(rendered.contains("aria-required=\"true\""));
}