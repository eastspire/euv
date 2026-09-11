use super::*;

/// An alert component that displays contextual messages with semantic styling.
///
/// Renders a styled message box using variant-specific classes.
/// Supports Success, Error, Warning, and Info variants.
/// The message content is passed as children.
///
/// # Arguments
///
/// - `VirtualNode<EuvAlertProps>` - The props node containing the variant.
///
/// # Returns
///
/// - `VirtualNode` - A styled alert element.
#[component]
pub fn euv_alert(node: VirtualNode<EuvAlertProps>) -> VirtualNode {
    let EuvAlertProps { variant }: EuvAlertProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    match variant {
        AlertVariant::Error => html! {
            div {
                class: c_error_box()
                children
            }
        },
        AlertVariant::Success => html! {
            div {
                class: c_success_box()
                children
            }
        },
    }
}
