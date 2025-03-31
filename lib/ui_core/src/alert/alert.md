<!-- @format -->

# Alert Component

An alert component to display important messages to users with various semantic styles and interactive options.

## Usage Example

```rust
use your_crate::Alert;
use your_crate::AlertVariant;

fn App(Scope) -> Element {
    rsx! {
        Alert {
            variant: AlertVariant::Success,
            title: Some("Operation Successful".to_string()),
            dismissible: true,
            "Your data has been saved successfully."
        }
    }
}
```

## Types

### AlertVariant

Represents the semantic type of alert:

```
pub enum AlertVariant {
#[default]
Default, // Blue, general information
Info, // Blue, informational content
Warning, // Yellow, warning that needs attention
Error, // Red, error that needs action
Success, // Green, successful operation
}
```

### AlertStatus

Tracks the display state of the alert:

```
pub enum AlertStatus {
    Visible,    // Alert is fully visible
    Dismissing, // Alert is in the process of being dismissed (for animations)
    Dismissed,  // Alert has been dismissed
}
```

### AlertIcon

Defines the icon to display in the alert:

```
pub enum AlertIcon {
    #[default]
    Default,  // Circle-i icon
    Info,     // Information icon
    Warning,  // Warning triangle icon
    Error,    // X icon
    Success,  // Checkmark icon
    None,     // No icon
}
```

### AlertAction

Defines an action button for the alert:

```
pub struct AlertAction {
    pub label: String,  // Button text
    pub on_click: Option<EventHandler<MouseEvent>>,  // Click handler
    pub class: String,  // Additional CSS classes
}
```

| Property       | Type                               | Default                 | Description                           |
| -------------- | ---------------------------------- | ----------------------- | ------------------------------------- |
| `children`     | `Element`                          | Required                | Main content of the alert             |
| `title`        | `Option<String>`                   | `None`                  | Optional title for the alert          |
| `variant`      | `AlertVariant`                     | `AlertVariant::Default` | Semantic style of the alert           |
| `icon`         | `Option<AlertIcon>`                | Matches variant         | Custom icon override                  |
| `class`        | `String`                           | `""`                    | Additional CSS classes                |
| `dismissible`  | `bool`                             | `false`                 | Whether the alert can be dismissed    |
| `on_dismiss`   | `Option<EventHandler<MouseEvent>>` | `None`                  | Event handler when alert is dismissed |
| `actions`      | `Vec<AlertAction>`                 | `vec![]`                | Action buttons for the alert          |
| `role`         | `&'static str`                     | `"alert"`               | ARIA role for accessibility           |
| `auto_dismiss` | `Option<u32>`                      | `None`                  | Auto dismiss timing in milliseconds   |
