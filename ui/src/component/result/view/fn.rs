use super::*;

/// A generic status/result page aligned with common component libraries
/// (Ant Design `Result`, Element Plus `Result`): a big status code, a title,
/// a description and an action area.
///
/// Typical uses are 404 / 403 / 500 pages; the action buttons (or links) are
/// passed in as children.
///
/// # Arguments
///
/// - `VirtualNode<EuvResultProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The result page virtual DOM tree.
#[component]
pub fn euv_result(node: VirtualNode<EuvResultProps>) -> VirtualNode {
    let EuvResultProps {
        code,
        title,
        description,
    }: EuvResultProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_result()
            if { !code.is_empty() } {
                div {
                    class: c_euv_result_code()
                    {
                        code
                    }
                }
            }
            if { !title.is_empty() } {
                h1 {
                    class: c_euv_result_title()
                    {
                        title
                    }
                }
            }
            if { !description.is_empty() } {
                p {
                    class: c_euv_result_description()
                    {
                        description
                    }
                }
            }
            div {
                class: c_euv_result_actions()
                children
            }
        }
    }
}
