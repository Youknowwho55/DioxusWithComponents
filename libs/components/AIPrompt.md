<!-- @format -->

Here's an AI prompt template that you can use to get consistent component showcase implementations for your Dioxus UI library:
CopyI'm building a component showcase for my Dioxus UI library. Please create a component showcase implementation for a {COMPONENT_NAME} component.

Here's the code for my {COMPONENT_NAME} component:

```rust
{PASTE_YOUR_COMPONENT_CODE_HERE}
Please create a showcase function called render_{component_name_lowercase}_showcase() that demonstrates all the different variants, properties, and states of this component.
The showcase should follow this structure:

1. A main heading with the component name
2. A short description of the component
3. Examples of different variants/configurations grouped by category (like schemes, sizes, states)
4. A documentation section listing all props with descriptions

My showcase follows a modular pattern where each component has its own showcase function that's called from a main match statement. Please follow the format in this existing code:
```

fn render_selected_component(selected: &Signal<ComponentShowcase>) -> Element {
match \*selected.read() {
ComponentShowcase::Card => render_card_showcase(),
ComponentShowcase::Button => render_button_showcase(),
// My new component will go here
}
}

```
Also, please explain how I should update my ComponentShowcase enum and components array to include this new component.
Copy

To use this template:

1. Replace `{COMPONENT_NAME}` with your component name (e.g., "Modal", "Dropdown", "Tabs")
2. Replace `{PASTE_YOUR_COMPONENT_CODE_HERE}` with the code for the component you want to add to the showcase
3. The `{component_name_lowercase}` will be automatically derived from your component name

This prompt provides all the necessary context and instructions to ensure you get consistent implementations that follow your established pattern. The AI will create a showcase function that displays all the variants of your component with proper documentation, and will explain how to integrate it into your existing showcase system.
```
