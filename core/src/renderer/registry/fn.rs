use super::*;

/// Walks the DOM ancestor chain of an event entirely in JS, invoking a
/// Rust-side callback for every marked ancestor.
///
/// This replaces the previous Rust-side loop in `dispatch_delegated_event`
/// that walked the ancestor chain one layer per round-trip (`get_attribute`
/// + `parent_element` = 2 JS crossings per layer; a depth-10 click cost 20
/// crossings). Moving the walk into JS collapses the crossing count to one
/// `#[wasm_bindgen]` call per event, regardless of DOM depth.
///
/// The JS body is provided inline via `#[wasm_bindgen(inline_js = "...")]`
/// so no separate `.js` file needs to be shipped alongside the wasm artifact.
/// The function accepts a `max_depth` cap so high-frequency events
/// (`mousemove`, `touchmove`, `wheel`, …) can keep their existing bounded
/// walk; passing `0` (per the call-site convention in
/// `dispatch_delegated_event`) means "walk until `<html>`".
///
/// Returns `true` when the callback returns `true`, indicating the handler
/// was found and the walk should stop. The Rust callback is responsible for
/// invoking the matching handler if it finds one in the registry.
///
/// # Arguments
///
/// - `event: &JsValue` - The DOM event whose target chain should be walked.
/// - `max_depth: usize` - Upper bound on hops; `0` means unbounded.
/// - `callback: &js_sys::Function` - JS function invoked as
///   `callback(euv_id)` for each marked ancestor. Receives the parsed
///   `usize` euv-id from the `data-euv-id` attribute.
///
/// # Returns
///
/// - `bool` - The callback's last return value (`true` = handler found).
#[wasm_bindgen(inline_js = r#"
export function euv_event_walk_ancestors(event, max_depth, callback) {
    let node = event.target;
    let depth = 0;
    // Unbounded when max_depth is 0 (per the call site convention in
    // dispatch_delegated_event — passing 0 means "walk until <html>").
    // Otherwise count `event.target` itself as depth 1.
    while (node) {
        if (max_depth !== 0 && depth >= max_depth) {
            return false;
        }
        // Only DOM Elements carry data-euv-id; skip text nodes cheaply.
        if (node.nodeType === 1) {
            const id = node.getAttribute && node.getAttribute("data-euv-id");
            if (id !== null && id !== undefined && id !== "") {
                // parseInt is native (no string alloc on the WASM side);
                // NaN check guards against malformed attribute values.
                const parsed = parseInt(id, 10);
                if (!isNaN(parsed)) {
                    if (callback(parsed) === true) {
                        return true;
                    }
                }
            }
        }
        node = node.parentElement;
        depth += 1;
    }
    return false;
}
"#)]
extern "C" {
    pub(crate) fn euv_event_walk_ancestors(
        event: &JsValue,
        max_depth: usize,
        callback: &js_sys::Function,
    ) -> bool;
}
