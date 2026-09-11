use super::*;

/// A generic documentation page layout aligned with common site frameworks
/// (VuePress default theme `Page`, Docusaurus doc item): a fluid content
/// column with the page body as children, `euv_pagination` prev/next links
/// and an optional footer, plus a sticky right `euv_toc` anchor column that
/// collapses on narrow viewports.
///
/// # Arguments
///
/// - `VirtualNode<EuvDocLayoutProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The doc page layout virtual DOM tree.
#[component]
pub fn euv_doc_layout(node: VirtualNode<EuvDocLayoutProps>) -> VirtualNode {
    let EuvDocLayoutProps {
        toc_title,
        toc_items,
        prev_label,
        next_label,
        prev,
        next,
        footer,
    }: EuvDocLayoutProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_doc_layout()
            div {
                class: c_euv_doc_content()
                children
                euv_pagination {
                    prev_label
                    next_label
                    prev
                    next
                }
                if { !footer.is_empty() } {
                    footer {
                        class: c_euv_footer()
                        {
                            footer
                        }
                    }
                }
            }
            if { !toc_items.is_empty() } {
                div {
                    class: c_euv_doc_toc()
                    euv_toc {
                        title: toc_title
                        items: toc_items
                    }
                }
            }
        }
    }
}
