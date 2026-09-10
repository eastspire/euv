use super::*;

/// A card component that wraps children with a styled container.
///
/// # Arguments
///
/// - `VirtualNode<EuvCardProps>` - The props node containing title.
///
/// # Returns
///
/// - `VirtualNode` - A styled card element.
#[component]
pub fn euv_card(node: VirtualNode<EuvCardProps>) -> VirtualNode {
    let EuvCardProps { title, .. }: EuvCardProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_card()
            h3 {
                class: c_card_title()
                title
            }
            children
        }
    }
}
