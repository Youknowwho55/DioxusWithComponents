#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    trigger_id: String,
    children: Element,
    submit_action: Option<String>,
    class: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    children: Element,
    class: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalActionProps {
    children: Element,
    class: Option<String>,
}