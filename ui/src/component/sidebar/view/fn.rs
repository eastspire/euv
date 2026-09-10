use super::*;

/// A generic collapsible navigation tree aligned with common docs frameworks.
///
/// Renders leaf links and collapsible groups recursively. The collapse state
/// lives in the caller-owned `collapsed` signal so it survives re-renders and
/// can be shared between multiple sidebar instances. All groups start
/// expanded.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The sidebar virtual DOM tree.
#[component]
pub fn euv_sidebar(node: VirtualNode<EuvSidebarProps>) -> VirtualNode {
    let EuvSidebarProps {
        route_signal,
        collapsed,
        items,
        prefix,
        on_navigate,
    }: EuvSidebarProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            for item in items.iter() {
                euv_sidebar_item {
                    route_signal
                    collapsed
                    item: *item
                    prefix: prefix.clone()
                    on_navigate: on_navigate.clone()
                }
            }
        }
    }
}

/// Renders one sidebar node: a leaf link or a collapsible group.
///
/// # Arguments
///
/// - `VirtualNode<EuvSidebarItemProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The item virtual DOM tree.
#[component]
pub fn euv_sidebar_item(node: VirtualNode<EuvSidebarItemProps>) -> VirtualNode {
    let EuvSidebarItemProps {
        route_signal,
        collapsed,
        item,
        prefix,
        on_navigate,
    }: EuvSidebarItemProps = node.try_get_props().unwrap_or_default();
    let route: String = route_signal.get();
    let path: &str = strip_hash_anchor(&route);
    if item.children.is_empty() {
        let Some(link) = item.link else {
            return html! {
                ""
            };
        };
        let active: bool = path == link;
        let link_class: fn() -> &'static Css = if active {
            c_euv_sidebar_link_active
        } else {
            c_euv_sidebar_link
        };
        return html! {
            a {
                class: {
                    link_class()
                }
                // OPT 30: see comment in `euv_navbar` — replace
                // `format!("#{...}")` with a single `#`-prefix concat.
                href: {
                    let mut href: String =
                        String::with_capacity(ROUTE_HASH_PREFIX.len() + link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(link);
                    href
                }
                onclick: navigate_link(on_navigate.clone(), link)
                {
                    item.text
                }
            }
        };
    }
    // OPT 30: `format!("{prefix}/{}", item.text)` becomes a pre-sized
    // concatenation of the constant separator `/` between two borrowed
    // slices.
    let key: String = {
        let mut key: String = String::with_capacity(prefix.len() + 1 + item.text.len());
        key.push_str(&prefix);
        key.push('/');
        key.push_str(item.text);
        key
    };
    let open: bool = !collapsed.get().contains(&key);
    let arrow_class: fn() -> &'static Css = if open {
        c_euv_sidebar_group_arrow_open
    } else {
        c_euv_sidebar_group_arrow
    };
    let title_node: VirtualNode = match item.link {
        Some(link) => html! {
            a {
                // OPT 30: same `#`-prefix concat as above.
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(link);
                    href
                }
                onclick: navigate_group_link(on_navigate.clone(), link)
                {
                    item.text
                }
            }
        },
        None => html! {
            {
                item.text
            }
        },
    };
    html! {
        div {
            class: c_euv_sidebar_group()
            div {
                class: c_euv_sidebar_group_title()
                onclick: toggle_group(collapsed, key.clone())
                span {
                    title_node
                }
                span {
                    class: arrow_class()
                    "▸"
                }
            }
            if open {
                div {
                    class: c_euv_sidebar_children()
                    euv_sidebar {
                        route_signal
                        collapsed
                        items: item.children
                        prefix: key.clone()
                        on_navigate: on_navigate.clone()
                    }
                }
            }
        }
    }
}

/// Builds the leaf-link click handler: the interceptor when set, otherwise
/// the default hash-router navigation.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
/// - `&'static str` - The target link.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn navigate_link(
    on_navigate: Option<Rc<dyn Fn(&'static str)>>,
    link: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    match on_navigate {
        Some(interceptor) => Some(Rc::new(move |event: Event| {
            event.prevent_default();
            interceptor(link);
        })),
        None => Some(Rc::new(move |event: Event| {
            event.prevent_default();
            Router::navigate(link);
        })),
    }
}

/// Navigates to a group index page without toggling the group.
///
/// Stops event propagation so the parent row's toggle handler does not also
/// fire when the link is clicked. Uses the interceptor when set.
///
/// # Arguments
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The navigation interceptor.
/// - `&'static str` - The target route.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn navigate_group_link(
    on_navigate: Option<Rc<dyn Fn(&'static str)>>,
    link: &'static str,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        event.prevent_default();
        event.stop_propagation();
        match &on_navigate {
            Some(interceptor) => interceptor(link),
            None => Router::navigate(link),
        }
    }))
}

/// Toggles a sidebar group's collapsed state.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The collapsed-keys signal.
/// - `String` - The group key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn toggle_group(collapsed: Signal<Vec<String>>, key: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_| {
        let mut keys: Vec<String> = collapsed.get();
        if let Some(index) = keys.iter().position(|k| k == &key) {
            keys.remove(index);
        } else {
            keys.push(key.clone());
        }
        collapsed.set(keys);
    }))
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
