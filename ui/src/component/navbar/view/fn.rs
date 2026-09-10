use super::*;

/// A generic fixed top navigation bar aligned with common site frameworks.
///
/// Renders the mobile hamburger button (when `drawer_open` is provided), the
/// brand block, the link list and a trailing actions area taken from the
/// children.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The navbar virtual DOM tree.
#[component]
pub fn euv_navbar(node: VirtualNode<EuvNavbarProps>) -> VirtualNode {
    let EuvNavbarProps {
        route_signal,
        brand_logo,
        brand_title,
        brand_href,
        items,
        drawer_open,
    }: EuvNavbarProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let menu_button: VirtualNode = match drawer_open {
        Some(signal) => html! {
            button {
                class: if { signal } {
                    c_euv_navbar_menu_button_active()
                } else {
                    c_euv_navbar_menu_button()
                }
                onclick: UseEuvLayout::use_drawer_toggle(signal)
                "≡"
            }
        },
        None => html! {
            ""
        },
    };
    html! {
        nav {
            class: c_euv_navbar()
            menu_button
            a {
                class: c_euv_navbar_brand()
                href: format!("#{brand_href}")
                onclick: Router::link_handler(brand_href)
                span {
                    class: c_euv_navbar_logo()
                    {
                        brand_logo
                    }
                }
                span {
                    {
                        brand_title
                    }
                }
            }
            div {
                class: c_euv_navbar_links()
                for item in items.iter() {
                    euv_navbar_link {
                        route_signal
                        item: *item
                    }
                }
            }
            div {
                class: c_euv_navbar_actions()
                children
            }
        }
    }
}

/// Renders a single navbar link (internal hash route or external URL).
///
/// # Arguments
///
/// - `VirtualNode<EuvNavbarLinkProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The link virtual DOM tree.
#[component]
pub fn euv_navbar_link(node: VirtualNode<EuvNavbarLinkProps>) -> VirtualNode {
    let EuvNavbarLinkProps { route_signal, item }: EuvNavbarLinkProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = item.link.starts_with("http");
    let route: String = route_signal.get();
    let path: &str = strip_hash_anchor(&route);
    let active: bool = !external && item.link != "/" && path.starts_with(item.link);
    let link_class: fn() -> &'static Css = if active {
        c_euv_navbar_link_active
    } else {
        c_euv_navbar_link
    };
    if external {
        html! {
            a {
                class: c_euv_navbar_link()
                href: item.link
                target: "_blank"
                onclick: Router::external_link_handler(item.link)
                {
                    item.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    link_class()
                }
                href: format!("#{}", item.link)
                onclick: Router::link_handler(item.link)
                {
                    item.text
                }
            }
        }
    }
}

/// Strips the `#anchor` suffix from a hash route.
///
/// # Arguments
///
/// - `&str` - The raw route string.
///
/// # Returns
///
/// - `&str` - The route path without the anchor.
fn strip_hash_anchor(route: &str) -> &str {
    match route.split_once('#') {
        Some((path, _)) => path,
        None => route,
    }
}
