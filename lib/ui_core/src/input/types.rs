// src/components/ui/input/types.rs
use dioxus::prelude::*;

/// Visual style variants for inputs with improved accessibility contrast
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputScheme {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl InputScheme {
    /// Returns Tailwind classes for the input scheme with focus states
    pub fn to_classes(&self) -> &'static str {
        match self {
            InputScheme::Default => "border-gray-300 focus:border-blue-500 focus:ring-1 focus:ring-blue-500",
            InputScheme::Primary => "border-blue-500 focus:border-blue-600 focus:ring-1 focus:ring-blue-600",
            InputScheme::Success => "border-green-500 focus:border-green-600 focus:ring-1 focus:ring-green-600",
            InputScheme::Warning => "border-yellow-500 focus:border-yellow-600 focus:ring-1 focus:ring-yellow-600",
            InputScheme::Danger => "border-red-500 focus:border-red-600 focus:ring-1 focus:ring-red-600",
            InputScheme::Info => "border-cyan-500 focus:border-cyan-600 focus:ring-1 focus:ring-cyan-600",
        }
    }

    /// Returns appropriate text color for error/help messages
    pub fn text_color(&self) -> &'static str {
        match self {
            InputScheme::Default | InputScheme::Primary | InputScheme::Info => "text-gray-600",
            InputScheme::Success => "text-green-600",
            InputScheme::Warning => "text-yellow-600",
            InputScheme::Danger => "text-red-600",
        }
    }
}

/// Size variants for inputs with consistent padding
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Medium,
    ExtraSmall,
    Small,
    Large,
    ExtraLarge,
}

impl InputSize {
    /// Returns Tailwind classes for the input size with line-height consideration
    pub fn to_classes(&self) -> &'static str {
        match self {
            InputSize::ExtraSmall => "px-2 py-1 text-xs leading-tight",
            InputSize::Small => "px-2.5 py-1.5 text-sm leading-normal",
            InputSize::Medium => "px-3 py-2 text-base leading-normal",
            InputSize::Large => "px-4 py-2.5 text-lg leading-normal",
            InputSize::ExtraLarge => "px-5 py-3 text-xl leading-normal",
        }
    }

    /// Returns corresponding label size classes
    pub fn label_classes(&self) -> &'static str {
        match self {
            InputSize::ExtraSmall => "text-xs",
            InputSize::Small => "text-sm",
            InputSize::Medium => "text-base",
            InputSize::Large => "text-lg",
            InputSize::ExtraLarge => "text-xl",
        }
    }
}

/// HTML input type attributes with modern web input types
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTimeLocal,
    Month,
    Week,
    Color,
    File,
    Hidden,
    Range,
    Checkbox,
    Radio,
    Textarea,
}

impl InputType {
    /// Returns the HTML input type string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DateTimeLocal => "datetime-local",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Color => "color",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Range => "range",
            InputType::Checkbox => "checkbox",
            InputType::Radio => "radio",
            InputType::Textarea => "textarea",
        }
    }

    /// Returns whether this input type should use a different base element
    pub fn is_special_element(&self) -> bool {
        matches!(self, InputType::Textarea | InputType::Checkbox | InputType::Radio)
    }
}

/// Properties for the Input component with better organization
#[derive(Props, Clone, PartialEq, Default)]
pub struct InputProps {
    // Core attributes
    /// HTML name attribute (required for form submission)
    pub name: String,
    
    /// Current value of the input
    #[props(default)]
    pub value: Option<String>,
    
    /// Input type variant
    #[props(default)]
    pub r#type: Option<InputType>,
    
    /// HTML id attribute (defaults to name if not provided)
    #[props(default)]
    pub id: Option<String>,
    
    // Visual customization
    /// Color scheme variant
    #[props(default)]
    pub scheme: Option<InputScheme>,
    
    /// Size variant
    #[props(default)]
    pub size: Option<InputSize>,
    
    /// Additional CSS classes for the input element
    #[props(default)]
    pub class: Option<String>,
    
    /// Additional CSS classes for the label element
    #[props(default)]
    pub label_class: Option<String>,
    
    // Content
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    
    /// Help text
    #[props(default)]
    pub help_text: Option<String>,
    
    /// Error message
    #[props(default)]
    pub error: Option<String>,
    
    /// Prefix element (icon or text)
    #[props(default)]
    pub prefix: Option<Element>,
    
    /// Suffix element (icon or text)
    #[props(default)]
    pub suffix: Option<Element>,
    
    // States
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    
    /// Readonly state
    #[props(default = false)]
    pub readonly: bool,
    
    /// Required state
    #[props(default = false)]
    pub required: bool,
    
    /// Autofocus state
    #[props(default = false)]
    pub autofocus: bool,
    
    // Validation attributes
    /// Minimum value (for number/date/range inputs)
    #[props(default)]
    pub min: Option<String>,
    
    /// Maximum value (for number/date/range inputs)
    #[props(default)]
    pub max: Option<String>,
    
    /// Step value (for number/date/range inputs)
    #[props(default)]
    pub step: Option<String>,
    
    /// Pattern for validation
    #[props(default)]
    pub pattern: Option<String>,
    
    /// Maximum length
    #[props(default)]
    pub maxlength: Option<usize>,
    
    /// Minimum length
    #[props(default)]
    pub minlength: Option<usize>,
    
    /// Autocomplete behavior
    #[props(default)]
    pub autocomplete: Option<String>,
    
    // Event handlers
   // Event handlers
   #[props(default)]
   pub oninput: Option<EventHandler<String>>,
   
   #[props(default)]
   pub onchange: Option<EventHandler<String>>,
   
   #[props(default)]
   pub onkeydown: Option<EventHandler<KeyboardEvent>>,
   
   #[props(default)]
   pub onkeyup: Option<EventHandler<KeyboardEvent>>,
   
   #[props(default)]
   pub onfocus: Option<EventHandler<FocusEvent>>,
   
   #[props(default)]
   pub onblur: Option<EventHandler<FocusEvent>>,
}
