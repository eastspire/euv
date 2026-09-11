use super::*;

/// Implementation of touch point extraction from DOM touch events.
impl NativeTouchPoint {
    /// Extracts all active touch points from a `TouchEvent`.
    ///
    /// Iterates over the `touches` list of the given `TouchEvent` and
    /// builds a `Vec<NativeTouchPoint>` with each touch point's
    /// identifier, viewport coordinates, screen coordinates, page
    /// coordinates, and offset coordinates relative to the target element.
    ///
    /// The offset coordinates (`offset_x`, `offset_y`) are computed by
    /// subtracting the target element's bounding rect from the touch's
    /// client coordinates, since the browser `Touch` object does not
    /// provide `offsetX`/`offsetY` directly.
    ///
    /// Uses web-sys typed getters (`TouchEvent::touches()`,
    /// `TouchList::get`, `Touch::client_x()`) instead of
    /// `Reflect::get(event, "clientX")`. The Reflect path allocates a
    /// `JsValue::from_str` per field per touch (7 fields × N touches per
    /// event) on every `touchmove` (60-120Hz); the typed getters skip
    /// the string lookup and the per-field JS string allocation.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - All currently active touch points.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPoint> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: i32 = touch.client_x();
                let client_y: i32 = touch.client_y();
                let screen_x: i32 = touch.screen_x();
                let screen_y: i32 = touch.screen_y();
                let page_x: i32 = touch.page_x();
                let page_y: i32 = touch.page_y();
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }

    /// Extracts the changed touch points from a `TouchEvent`.
    ///
    /// The `changedTouches` list contains touch points that have changed
    /// since the last touch event:
    /// - For `touchstart` - newly added touch points.
    /// - For `touchmove` - touch points that have moved.
    /// - For `touchend` / `touchcancel` - removed touch points.
    ///
    /// This is useful for determining which specific fingers were lifted
    /// in a `touchend` event, since the `touches` list no longer contains
    /// them.
    ///
    /// Uses web-sys typed getters (`TouchEvent::changed_touches()`,
    /// `TouchList::get`, `Touch::client_x()`) to avoid the per-field
    /// `Reflect::get` + `JsValue::from_str` allocation cost on the hot
    /// `touchmove` path.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - The touch points that changed in this event.
    pub fn extract_changed(event: &Event) -> Vec<NativeTouchPoint> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.changed_touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: i32 = touch.client_x();
                let client_y: i32 = touch.client_y();
                let screen_x: i32 = touch.screen_x();
                let screen_y: i32 = touch.screen_y();
                let page_x: i32 = touch.page_x();
                let page_y: i32 = touch.page_y();
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}

/// Implementation of high-precision touch point extraction from DOM touch events.
impl NativeTouchPointF64 {
    /// Extracts all active touch points with high-precision `f64` offset coordinates
    /// from a `TouchEvent`.
    ///
    /// Similar to `NativeTouchPoint::extract_all`, but returns `f64` precision for
    /// offset/client coordinates, which is essential for canvas drawing
    /// and other pixel-precise interactions.
    ///
    /// Uses web-sys typed getters (`TouchEvent::touches()`,
    /// `TouchList::get`, `Touch::client_x()`) instead of
    /// `Reflect::get(event, "clientX")`. web-sys `Touch` exposes
    /// `client_x`/`page_x`/etc. as `i32`; we widen to `f64` to preserve
    /// the `NativeTouchPointF64` high-precision contract without losing
    /// sub-pixel information on the offset computation.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPointF64>` - All currently active touch points with `f64` coordinates.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPointF64> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: f64 = touch.client_x() as f64;
                let client_y: f64 = touch.client_y() as f64;
                let screen_x: f64 = touch.screen_x() as f64;
                let screen_y: f64 = touch.screen_y() as f64;
                let page_x: f64 = touch.page_x() as f64;
                let page_y: f64 = touch.page_y() as f64;
                let offset_x: f64 = client_x - rect_left;
                let offset_y: f64 = client_y - rect_top;
                NativeTouchPointF64 {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}
