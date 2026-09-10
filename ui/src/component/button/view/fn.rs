use super::*;

/// A versatile button component supporting primary and outline variants.
///
/// Renders a styled `<button>` element whose appearance is determined by the
/// `variant` and `disabled` props. When children are provided they replace
/// the label text.
///
/// # Arguments
///
/// - `VirtualNode<EuvButtonProps>` - The props node containing variant, label, onclick, disabled.
///
/// # Returns
///
/// - `VirtualNode` - A styled button element.
#[component]
pub fn euv_button(node: VirtualNode<EuvButtonProps>) -> VirtualNode {
    let EuvButtonProps {
        variant,
        label,
        onclick: click_handler,
        disabled,
    }: EuvButtonProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let content: VirtualNode = match children {
        VirtualNode::Empty => VirtualNode::Text(TextNode::new(Cow::Owned(label.to_string()), None)),
        other => other,
    };
    match variant {
        EuvButtonVariant::Primary => html! {
            button {
                class: c_euv_button_primary_md()
                disabled: disabled.get()
                onclick: click_handler
                content
            }
        },
        EuvButtonVariant::Outline => html! {
            button {
                class: c_euv_button_outline_md()
                disabled: disabled.get()
                onclick: click_handler
                content
            }
        },
    }
}
