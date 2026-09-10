use super::*;

/// Renders a branded logo element displaying the "E" letter with a gradient background.
///
/// When `on_click` is provided, renders as a `<button>` element;
/// otherwise renders as a `<span>` element (e.g. for use inside anchor tags).
/// Used both as the navigation sidebar logo and as the vConsole floating action button.
///
/// # Arguments
///
/// - `VirtualNode<EuvLogoProps>` - The props node containing variant, click handler.
///
/// # Returns
///
/// - `VirtualNode` - A styled element with the "E" branding.
#[component]
pub fn euv_logo(node: VirtualNode<EuvLogoProps>) -> VirtualNode {
    let EuvLogoProps { variant, on_click }: EuvLogoProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let class_name: String = match variant {
        LogoButtonVariant::Nav => format!(
            "{} {}",
            c_euv_logo().get_name(),
            c_euv_logo_nav().get_name()
        ),
        LogoButtonVariant::Fab => format!(
            "{} {}",
            c_euv_logo().get_name(),
            c_euv_logo_fab().get_name()
        ),
    };
    if on_click.is_some() {
        html! {
            button {
                class: class_name
                onclick: on_click
                "E"
                children
            }
        }
    } else {
        html! {
            span {
                class: class_name
                "E"
            }
        }
    }
}
