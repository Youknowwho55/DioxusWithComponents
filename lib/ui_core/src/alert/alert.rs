// src/components/ui/alert/component.rs
use dioxus::prelude::*;
use super::types::{AlertProps, AlertVariant, AlertIcon, AlertAction, AlertStatus};
use std::time::Duration;

/// A customizable Alert component for displaying messages with various semantic styles.
///
/// # Features
/// - Multiple semantic variants (info, warning, error, success)
/// - Customizable icons
/// - Optional title
/// - Optional dismiss button
/// - Action buttons
/// - Auto-dismiss functionality
/// - Accessibility support
///
/// # Example
/// ```
/// use your_crate::Alert;
/// use your_crate::AlertVariant;
///
/// fn App(cx: Scope) -> Element {
///     render! {
///         Alert {
///             variant: AlertVariant::Success,
///             title: Some("Operation Successful".to_string()),
///             dismissible: true,
///             "Your data has been saved successfully."
///         }
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    // State for managing visibility
    let is_visible = use_state(cx, || true);
    let status = use_state(cx, || AlertStatus::Visible);
    
    // Set up auto-dismiss timer if specified
    if let Some(dismiss_time) = props.auto_dismiss {
        use_effect(cx, dismiss_time, |time| {
            async move {
                tokio::time::sleep(Duration::from_millis(time as u64)).await;
                status.set(AlertStatus::Dismissing);
                // Add timeout for animation before fully dismissing
                tokio::time::sleep(Duration::from_millis(300)).await;
                is_visible.set(false);
            }
        });
    }
    
    // Handle dismiss action
    let on_dismiss = {
        let status = status.clone();
        let is_visible = is_visible.clone();
        let on_dismiss = props.on_dismiss.clone();
        
        move |evt: MouseEvent| {
            status.set(AlertStatus::Dismissing);
            // Trigger user's dismiss handler if provided
            if let Some(handler) = &on_dismiss {
                handler.call(evt);
            }
            // Set state to dismiss after animation
            set_timeout(move || is_visible.set(false), Duration::from_millis(300));
        }
    };
    
    // Don't render if dismissed
    if !is_visible.get() {
        return None;
    }
    
    // Determine appropriate icon
    let icon_to_use = props.icon.unwrap_or_else(|| props.variant.get_icon());
    
    // Classes for alert container
    let base_classes = props.variant.to_classes();
    let animation_class = match *status.get() {
        AlertStatus::Visible => "",
        AlertStatus::Dismissing => "alert-dismissing",
        AlertStatus::Dismissed => "alert-dismissed",
    };
    let combined_classes = format!("alert {} {} {}", base_classes, animation_class, props.class);
    
    rsx! {
        div { class: "{combined_classes}", role: "{props.role}",
            // Icon section (if icon is not None)
            {
                match icon_to_use.to_svg() {
                    Some(svg) => rsx! {
                        div { class: "alert-icon", dangerous_inner_html: "{svg}" }
                    },
                    None => None,
                }
            }
            // Content section
            div { class: "alert-content",
                // Optional title
                {
                    match &props.title {
                        Some(title) => rsx! {
                            div { class: "alert-title font-bold mb-1", "{title}" }
                        },
                        None => None,
                    }
                }
                // Main content
                div { class: "alert-message", {props.children} }
            }
            // Dismiss button (if dismissible)
            {
                if props.dismissible {
                    rsx! {
                        button {
                            class: "alert-dismiss-btn",
                            "aria-label": "Dismiss",
                            onclick: on_dismiss,
                            // X icon
                            dangerous_inner_html: r#"<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>"#,
                        }
                    }
                } else {
                    None
                }
            }
            // Action buttons (if any)
            {
                if !props.actions.is_empty() {
                    rsx! {
                        div { class: "alert-actions flex gap-2 mt-2",
                            {
                                props
                                    .actions
                                    .iter()
                                    .map(|action| {
                                        let action_class = format!("btn btn-sm {}", action.class);
                                        let on_click = action.on_click.clone();
                                        let label = action.label.clone();
                                        rsx! {
                                            button {
                                                class: "{action_class}",
                                                onclick: move |evt| {
                                                    if let Some(handler) = &on_click {
                                                        handler.call(evt);
                                                    }
                                                },
                                                "{label}"
                                            }
                                        }
                                    })
                            }
                        }
                    }
                } else {
                    None
                }
            }
        }
    }
}