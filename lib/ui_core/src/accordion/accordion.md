<!-- @format -->

markdown
Copy

# DaisyUI Accordion Component

## Basic Usage

```rust
use your_crate::components::daisy_ui::accordion::Accordion;

Accordion {
    name: "faq-1".into(),
    title: "Question".into(),
    children: rsx! { p { "Answer" } }
}
Key Features

Behaviors: Single (radio), Multiple (checkbox), or Default
Accessibility: Full ARIA attributes & keyboard nav
Styling: Custom borders, hover states, animations
Grouping: Visually join accordions with join: true
Essential Props

Prop	Type	Default	Description
name	String	Required	Group identifier
title	String	Required	Header text
children	Element	Required	Content body
open	bool	false	Initial expanded state
accordion_type	AccordionType	Default	Behavior variant
Styling Quick Reference

rust
Copy
Accordion {
    // ...
    border_color: Some("border-blue-300".into()),
    hover_bg: Some("hover:bg-blue-50".into()),
    animated: false, // Disable animations
    class: Some("custom-class".into()) // Full override
}
Best Practices

Use for: FAQs, settings panels, content disclosure
Avoid: Nesting accordions, overloading content
Accessibility: Always provide name for groups
Example Group

rust
Copy
rsx! {
    Accordion {
        name: "group".into(),
        title: "Item 1".into(),
        join: true,
        children: rsx! { /* ... */ }
    }
    Accordion {
        name: "group".into(),
        title: "Item 2".into(),
        join: true,
        children: rsx! { /* ... */ }
    }
}
Tip: Use on_toggle handler for state management in complex UIs.
rust
Copy
on_toggle: move |is_open| {
    println!("Accordion is now {}", if is_open { "open" } else { "closed" });
}
Copy
```
