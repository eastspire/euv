use super::*;

/// A key-value information row component with label and value display.
///
/// Renders a horizontal row with a label on the left and a value on the right,
/// using `c_info_row`, `c_info_label`, and `c_info_value` styling.
/// The value is passed as children content.
///
/// # Arguments
///
/// - `VirtualNode<EuvInfoProps>` - The props node containing the label.
///
/// # Returns
///
/// - `VirtualNode` - A styled information row element.
#[component]
pub fn euv_info(node: VirtualNode<EuvInfoProps>) -> VirtualNode {
    let EuvInfoProps { label: label_text }: EuvInfoProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_info_row()
            span {
                class: c_info_label()
                label_text
            }
            span {
                class: c_info_value()
                children
            }
        }
    }
}
