<!-- @format -->

How to Add New Components to the Showcase
I've created a comprehensive implementation that includes both Card and Button components in the showcase. Here's a step-by-step guide on how to add new components:
Steps to Add a New Component to the Showcase

Update the ComponentShowcase enum:

```

pub enum ComponentShowcase {
Card,
Button,
// Add your new component here, e.g., Modal,
}
```

Add the component to the components array:

```

let components = [
    ComponentShowcase::Card,
    ComponentShowcase::Button,
    // Add your new component here, e.g., ComponentShowcase::Modal,
];
```

Import the component:

```

use crate::components::daisy_ui::your_component::{YourComponent, YourComponentProps};
```

Create a render function for the new component:

```
fn render_your_component_showcase() -> Element {
    rsx! {
        div { class: "space-y-8",
            h2 { class: "text-2xl font-bold", "Your Component Name" }
            p { class: "text-gray-600 mb-4", "Description of your component." }

            // Example usage of your component
            YourComponent {
                // Set props here
                "Your component content"
            }

            // Component documentation
            div { class: "mt-8 p-4 bg-base-300 rounded-lg",
                h3 { class: "font-bold", "Props" }
                ul { class: "list-disc pl-5 mt-2",
                    li { "prop1: Description of prop1" }
                    li { "prop2: Description of prop2" }
                }
            }
        }
    }
}
```

Add the new component to the render_selected_component function:

```
fn render_selected_component(selected: &Signal<ComponentShowcase>) -> Element {
    match *selected.read() {
        ComponentShowcase::Card => render_card_showcase(),
        ComponentShowcase::Button => render_button_showcase(),
        ComponentShowcase::YourComponent => render_your_component_showcase(),
        // Add more cases for other components
    }
}
```

Organization Tips

1. **Group related components:** As your showcase grows, consider organizing components into categories.
2. **Use consistent layouts:** Follow the pattern of title, description, examples, and documentation for each component.
3. **Document thoroughly:** Include all props, their types, and what they do for each component.
4. **Show variations:** Display different configurations of each component (like I did with Button sizes and schemes).
5. **Consider splitting into files:** If your showcase grows very large, you might want to move each component's showcase function to a separate file:

````

// src/showcase/button_showcase.rs
pub fn render_button_showcase() -> Element {
    // Button showcase code here
}

// Then in your main file:
use crate::showcase::button_showcase::render_button_showcase;
```


// Then in your main file:
```

use crate::showcase::button_showcase::render_button_showcase;
```

The implementation I've provided showcases different aspects of the Button component, including different schemes, sizes, and special states like disabled buttons and buttons with icons. The same approach can be applied to any component in your UI library.
````
