// accordion showcase component
pub fn render_accordion_showcase() -> Element {
    rsx! {
        div { class: "space-y-8 p-4",

            h2 { class: "text-2xl font-bold", "Accordion Component" }
            // Default type
            div { class: "w-full max-w-2xl mx-auto",
                Accordion {
                    name: "example-group".to_string(),
                    title: "Example Accordion".to_string(),
                    open: Some(true),
                    accordion_type: Some(AccordionType::Arrow),
                    join: Some(false),
                    class: Some("mb-4".to_string()),
                    children: rsx! {
                        p { "This is the content of the accordion." }
                    },
                    on_toggle: move |is_open| {
                        println!("Accordion is now {}", if is_open { "open" } else { "closed" });
                    },
                }
                // Arrow type
                Accordion {
                    name: "demo-accordion-2",
                    title: "Arrow Style Accordion",
                    open: Some(false),
                    accordion_type: Some(AccordionType::Arrow),
                    join: None,
                    children: rsx! {
                        p { "This accordion has an arrow indicator." }
                    },
                }
                // Plus type
                Accordion {
                    name: "demo-accordion-3",
                    title: "Plus Style Accordion",
                    open: Some(false),
                    accordion_type: Some(AccordionType::Plus),
                    join: None,
                    children: rsx! {
                        p { "This accordion has a plus/minus indicator." }
                    },
                }
            }
            // Radio group example
            div { class: "w-full max-w-2xl mx-auto mt-4",
                h4 { class: "text-lg font-bold mb-2", "Radio Group Behavior" }
                Accordion {
                    name: "radio-group-1",
                    title: "Section 1",
                    open: Some(true),
                    accordion_type: Some(AccordionType::Radio),
                    join: Some(true),
                    children: rsx! {
                        p { "This is section 1 content." }
                    },
                }
                Accordion {
                    name: "radio-group-1",
                    title: "Section 2",
                    open: Some(false),
                    accordion_type: Some(AccordionType::Radio),
                    join: Some(true),
                    children: rsx! {
                        p { "This is section 2 content." }
                    },
                }
            }
            // Component documentation
            div { class: "mt-8 p-4 bg-gray-100 rounded-lg w-full max-w-2xl mx-auto",
                h3 { class: "font-bold", "Props" }
                ul { class: "list-disc pl-5 mt-2",
                    li { "title: Text displayed in the accordion header" }
                    li { "name: Name attribute for grouping radio buttons" }
                    li { "open: Whether the accordion is initially open" }
                    li {
                        "accordion_type: Visual indicator style (Default, Arrow, Plus, Radio, RadioPlus)"
                    }
                    li { "join: Whether to join with adjacent accordions visually" }
                    li { "class: Additional classes for the container" }
                    li { "title_class: Additional classes for the title" }
                    li { "content_class: Additional classes for the content" }
                }
            }
        }
    }
}// END accordion showcase component

