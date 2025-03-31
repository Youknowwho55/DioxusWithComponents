// src/components/ui/input.rs
use dioxus::prelude::*;
use super::types::{InputProps, InputScheme, InputSize, InputType};

/// A customizable input component with multiple variants
///
/// # Example
/// ```rust
/// Input {
///     name: "username",
///     r#type: InputType::Text,
///     placeholder: "Enter your username",
///     label: "Username",
///     required: true
/// }
/// ```
#[component]
pub fn Input(props: InputProps) -> Element {
    // Use unwrap_or_default for cleaner default handling
    let scheme = props.scheme.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let input_type = props.r#type.unwrap_or_default();

    // Combine classes more efficiently with better spacing
    let mut input_class = format!(
        "block w-full rounded-md shadow-sm {} {} {}",
        scheme.to_classes(),
        size.to_classes(),
        props.class.as_deref().unwrap_or("")
    ).trim().to_string();

    // Add state-based classes
    if props.disabled {
        input_class.push_str(" opacity-75 cursor-not-allowed bg-gray-50");
    }
    if props.readonly {
        input_class.push_str(" bg-gray-50");
    }

    // Generate unique ID if none provided
    let input_id = props.id.as_deref().unwrap_or(props.name.as_str());

    // Determine if we need special spacing for prefix/suffix
    let has_prefix = props.prefix.is_some();
    let has_suffix = props.suffix.is_some();

    rsx! {
        div { class: "input-container space-y-2",
            // Label if provided with size-appropriate styling
            {props.label.as_ref().map(|label| rsx! {
                label {
                    class: "block font-medium {size.label_classes()} {scheme.text_color()} {props.label_class.as_deref().unwrap_or(\"\")}",
                    r#for: input_id,
                    "{label}"
                    {props.required.then(|| rsx! {
                        span { class: "text-red-500", " *" }
                    })}
                }
            })}

            // Input container with potential prefix/suffix
            div { class: "relative rounded-md shadow-sm",
                // Prefix element
                {props.prefix.as_ref().map(|prefix| rsx! {
                    div { class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                        span { class: "text-gray-500 sm:text-sm", {prefix} }
                    }
                })}

                // Main input element
                input {
                    class: "{input_class} \
                           {has_prefix.then(|| \"pl-7\").unwrap_or(\"\")} \
                           {has_suffix.then(|| \"pr-7\").unwrap_or(\"\")}",
                    id: input_id,
                    name: props.name,
                    r#type: input_type.as_str(),
                    value: props.value.as_deref(),
                    placeholder: props.placeholder.as_deref(),
                    disabled: props.disabled,
                    readonly: props.readonly,
                    required: props.required,
                    autofocus: props.autofocus,
                    min: props.min.as_deref(),
                    max: props.max.as_deref(),
                    step: props.step.as_deref(),
                    pattern: props.pattern.as_deref(),
                    maxlength: props.maxlength.map(|v| v.to_string()).as_deref(),
                    minlength: props.minlength.map(|v| v.to_string()).as_deref(),
                    autocomplete: props.autocomplete.as_deref(),
                    input {
                        // Input/Change events (for FormEvent)
                        oninput: move |e: FormEvent| {
                            let value = e.value();
                            if let Some(handler) = &props.oninput {
                                handler.call(value);
                            }
                        },
                        onchange: move |e: FormEvent| {
                            let value = e.value();
                            if let Some(handler) = &props.onchange {
                                handler.call(value);
                            }
                        },
                        // Keyboard events
                        onkeydown: move |e: KeyboardEvent| {
                            if let Some(handler) = &props.onkeydown {
                                handler.call(e);
                            }
                        },
                        onkeyup: move |e: KeyboardEvent| {
                            if let Some(handler) = &props.onkeyup {
                                handler.call(e);
                            }
                        },
                        // Focus events
                        onfocus: move |e: FocusEvent| {
                            if let Some(handler) = &props.onfocus {
                                handler.call(e);
                            }
                        },
                        onblur: move |e: FocusEvent| {
                            if let Some(handler) = &props.onblur {
                                handler.call(e);
                            }
                        },
                    }
                }

                // Suffix element
                {props.suffix.as_ref().map(|suffix| rsx! {
                    div { class: "absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none",
                        span { class: "text-gray-500 sm:text-sm", {suffix} }
                    }
                })}
            }

            // Help text if provided
            {props.help_text.as_ref().map(|help| rsx! {
                p { class: "text-sm {scheme.text_color()}", "{help}" }
            })}

            // Error message if provided
            {props.error.as_ref().map(|error| rsx! {
                p { class: "text-sm text-red-600", "{error}" }
            })}
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::types::{InputProps, InputScheme, InputSize, InputType};

    // Basic rendering test
    #[test]
    fn renders_basic_input() {
        let props = InputProps {
            name: "test_input".to_string(),
            label: Some("Test Label".to_string()),
            ..Default::default()
        };
        
        // This just verifies the component compiles and renders
        let _ = Input(props);
    }

    // Test all input types
    #[test]
    fn handles_all_input_types() {
        let types = [
            InputType::Text,
            InputType::Email,
            InputType::Password,
            // Add all other variants...
        ];
        
        for input_type in types {
            let props = InputProps {
                name: "type_test".to_string(),
                r#type: Some(input_type),
                ..Default::default()
            };
            let _ = Input(props);
        }
    }

    // Test class generation
    #[test]
    fn generates_correct_classes() {
        let props = InputProps {
            name: "class_test".to_string(),
            scheme: Some(InputScheme::Primary),
            size: Some(InputSize::Large),
            class: Some("extra-class".to_string()),
            ..Default::default()
        };
        
        let mut dom = VirtualDom::new_with_props(Input, props);
        let rendered = dioxus_ssr::render(&dom);
        assert!(rendered.contains("border-blue-500"));
        assert!(rendered.contains("px-4 py-2.5"));
        assert!(rendered.contains("extra-class"));
    }
}