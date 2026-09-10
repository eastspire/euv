use super::*;

/// A declarative router component that renders different content based on the current route.
///
/// Matches the current route against the provided route configurations and renders
/// the corresponding component. Falls back to the fallback component if no route matches.
///
/// # Arguments
///
/// - `VirtualNode<EuvRoutesProps>` - The props node containing route configurations.
///
/// # Returns
///
/// - `VirtualNode` - The matched route's component or fallback.
#[component]
pub fn euv_routes(node: VirtualNode<EuvRoutesProps>) -> VirtualNode {
    let EuvRoutesProps {
        route_signal,
        routes,
        fallback,
    }: EuvRoutesProps = node.try_get_props().unwrap_or_default();
    let current: String = route_signal.get();
    let matched: Option<Rc<dyn Fn() -> VirtualNode>> = routes
        .into_iter()
        .find(|config: &EuvRouteConfig| config.path == current)
        .map(|config: EuvRouteConfig| config.component);
    match matched {
        Some(component) => component(),
        None => fallback.map_or(VirtualNode::Empty, |f: Rc<dyn Fn() -> VirtualNode>| f()),
    }
}

/// A simple page router wrapper that provides a container for routed content.
///
/// # Arguments
///
/// - `VirtualNode<EuvPageRouterProps>` - The props node containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The page router container with the children content.
#[component]
pub fn euv_page_router(node: VirtualNode<EuvPageRouterProps>) -> VirtualNode {
    let EuvPageRouterProps {
        route_signal: _,
        fallback: _,
    }: EuvPageRouterProps = node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_page_router()
            children
        }
    }
}
