use super::*;

/// A high-performance virtual list component for rendering large datasets.
///
/// Only renders the visible items plus an overscan buffer, keeping DOM node count
/// constant regardless of total list size. Supports multiple instances on the same page
/// through unique container ids.
///
/// # Arguments
///
/// - `VirtualNode<EuvVirtualListProps>` - The props node containing configuration, item renderer, and optional callbacks.
///
/// # Returns
///
/// - `VirtualNode` - The virtual list container with windowed rendering.
#[component]
pub fn euv_virtual_list(node: VirtualNode<EuvVirtualListProps>) -> VirtualNode {
    let EuvVirtualListProps {
        config,
        item_renderer,
        on_scroll,
        on_visible_range_change,
    } = node.try_get_props().unwrap_or_default();
    let state: UseVirtualList = UseVirtualList::use_scroll_state();
    let container_id: String = config.id.clone();
    let total_count: usize = config.total_count;
    let item_height: i32 = config.item_height;
    let overscan_count: usize = config.overscan_count;
    let viewport_height: i32 = state.get_viewport_height().get();
    if viewport_height == 0 {
        state.schedule_measure_by_id(&container_id);
    }
    // OPT-24: cache the container Element via NodeRef so the scroll
    // handler avoids `get_element_by_id` + `dyn_into<HtmlElement>` on
    // every scroll event. The `ref:` attribute on the container div
    // populates this once the renderer mounts the node.
    let container_ref: NodeRef<Element> = App::use_node_ref();
    let scroll_handler: Option<Rc<dyn Fn(Event)>> = {
        let state: UseVirtualList = state;
        let container_ref: NodeRef<Element> = container_ref.clone();
        let on_scroll: Option<VirtualListScrollHandler> = on_scroll;
        Some(Rc::new(move |_: Event| {
            if let Some(element_value) = container_ref.get() {
                let html_element: HtmlElement = element_value.unchecked_into();
                let scroll_offset: i32 = html_element.scroll_top();
                state.get_scroll_offset().set(scroll_offset);
                if let Some(ref callback) = on_scroll {
                    callback(scroll_offset);
                }
            }
        }))
    };
    let scroll_offset: i32 = state.get_scroll_offset().get();
    let (_, _, render_start, render_end): (usize, usize, usize, usize) =
        UseVirtualList::compute_visible_range(
            scroll_offset,
            viewport_height,
            total_count,
            item_height,
            overscan_count,
        );
    let range_callback: Option<VirtualListRangeHandler> = on_visible_range_change.clone();
    let range_watch_initialized: Signal<bool> = App::use_signal(|| false);
    if !range_watch_initialized.get() {
        let scroll_offset_signal: Signal<i32> = state.get_scroll_offset();
        let viewport_height_signal: Signal<i32> = state.get_viewport_height();
        let fire_handle: FireHandle = (move || {
            let scroll_offset: i32 = scroll_offset_signal.get();
            let viewport_height: i32 = viewport_height_signal.get();
            if let Some(ref callback) = range_callback {
                let (visible_start, visible_end, _, _): (usize, usize, usize, usize) =
                    UseVirtualList::compute_visible_range(
                        scroll_offset,
                        viewport_height,
                        total_count,
                        item_height,
                        overscan_count,
                    );
                callback((visible_start, visible_end));
            }
        })
        .into();
        App::batch(|| {
            scroll_offset_signal.subscribe(move || {
                App::batch(|| unsafe { fire_handle.fire() });
            });
            viewport_height_signal.subscribe(move || {
                App::batch(|| unsafe { fire_handle.fire() });
            });
            unsafe { fire_handle.fire() }
            range_watch_initialized.set(true);
        });
    }
    let total_height: i32 = total_count as i32 * item_height;
    let top_padding: i32 = render_start as i32 * item_height;
    // OPT-24: build the per-render style strings once (not once per
    // visible item). The previous version called `format!()` three
    // times per visible item per scroll frame, allocating 150 Strings
    // per frame at 50 visible items. Now we build 3 Strings per render
    // and reuse their `&str` slices via `style` interpolation.
    let item_height_style: String = format!("height: {item_height}px; box-sizing: border-box;");
    let item_height_style_ref: &str = item_height_style.as_str();
    let row_wrapper_style: String =
        format!("position: absolute; top: {top_padding}px; left: 0; right: 0;");
    let row_wrapper_style_ref: &str = row_wrapper_style.as_str();
    let scroll_spacer_style: String = format!("position: relative; height: {total_height}px;");
    let scroll_spacer_style_ref: &str = scroll_spacer_style.as_str();
    let children: Vec<VirtualNode> = (render_start..render_end)
        .map(|index: usize| {
            let item_node: VirtualNode = (item_renderer)(index);
            html! {
                div {
                    key: index.to_string()
                    style: item_height_style_ref
                    item_node
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_virtual_list_container()
            id: container_id
            ref: container_ref
            onscroll: scroll_handler
            div {
                style: scroll_spacer_style_ref
                div {
                    style: row_wrapper_style_ref
                    children
                }
            }
        }
    }
}
