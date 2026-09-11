use super::*;

/// Implementation of input functionality.
impl UseEuvInput {
    /// Creates a click event handler that toggles a boolean signal.
    ///
    /// Produces a `NativeEventHandler` that flips the value of the given
    /// boolean signal on each click. Useful for toggle buttons, visibility
    /// switches, and drawer open/close patterns.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The boolean signal to toggle.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the signal.
    pub fn use_toggle(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |_: Event| {
            let current: bool = signal.get();
            signal.set(!current);
        }))
    }

    /// Creates an input event handler that updates a string signal.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The signal to update with the input value.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
    pub fn on_input_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let value: Option<String> = event.target().and_then(|target: EventTarget| {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    return Some(input.value());
                }
                if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                    return Some(textarea.value());
                }
                if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    return Some(select.value());
                }
                None
            });
            if let Some(value) = value {
                signal.set(value);
            }
        }))
    }

    /// Creates a change event handler that updates a string signal.
    ///
    /// # Arguments
    ///
    /// - `Signal<String>` - The signal to update with the change value.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
    pub fn on_change_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let value: Option<String> = event.target().and_then(|target: EventTarget| {
                if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                    return Some(input.value());
                }
                if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                    return Some(select.value());
                }
                if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                    return Some(textarea.value());
                }
                None
            });
            if let Some(value) = value {
                signal.set(value);
            }
        }))
    }

    /// Creates a change event handler that updates a boolean signal from checkbox.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - The boolean signal to update with the checked state.
    ///
    /// # Returns
    ///
    /// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
    pub fn on_change_checked(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            if let Some(target) = event.target()
                && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
            {
                signal.set(input.checked());
            }
        }))
    }

    /// Focus gap (CSS) reserved between the focused input and the on-screen
    /// keyboard. Small enough to feel tight, large enough that the caret does
    /// not graze the IME top edge.
    const FOCUS_GAP_PX: f64 = 12.0;

    /// Time (ms) the browser / WebView is given to bring up the IME and
    /// update the visual viewport before we measure element position.
    const FOCUS_SCROLL_DELAY_MILLIS: i32 = 220;

    /// Creates a focus handler that scrolls the focused input into the
    /// visible area between the safe top and the soft keyboard.
    ///
    /// Reads `--euv-keyboard-height` (set by the native host /
    /// `IMMERSIVE_SAFE_AREA_SCRIPT` page bridge) and the visual viewport.
    /// If the input's bottom edge falls under
    /// `viewport_bottom - keyboard_height - FOCUS_GAP_PX`, the page is
    /// scrolled by the difference. A small inline `padding-bottom` is also
    /// added to `<main>` (when one exists) so the document gains enough
    /// scrollable space for the adjustment — the inline style is cleared
    /// by [`Self::on_blur_restore_height`].
    pub fn on_focus_scroll_into_view() -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<HtmlElement>() else {
                return;
            };
            let Some(window) = window() else {
                return;
            };
            let element_clone: HtmlElement = element.clone();
            let window_clone: Window = window.clone();
            if let Ok(Some(main_el)) = element.closest("main")
                && let Ok(main) = main_el.dyn_into::<HtmlElement>()
            {
                let _: Result<(), JsValue> = main
                    .style()
                    .set_property("padding-bottom", "var(--euv-keyboard-height, 0px)");
            }
            let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let rect: DomRect = element_clone.get_bounding_client_rect();
                let input_bottom: f64 = rect.bottom();
                let viewport_height: f64 = window_clone
                    .visual_viewport()
                    .map(|viewport: VisualViewport| viewport.height())
                    .unwrap_or_else(|| {
                        window_clone
                            .inner_height()
                            .map(|height: JsValue| height.as_f64().unwrap_or_default())
                            .unwrap_or_default()
                    });
                let document_value: Document = match window_clone.document() {
                    Some(doc) => doc,
                    None => return,
                };
                let probe: Element = match document_value.create_element("div") {
                    Ok(el) => el,
                    Err(_) => return,
                };
                let keyboard_height: f64 = window_clone
                    .get_computed_style(&probe)
                    .ok()
                    .flatten()
                    .and_then(|style| style.get_property_value("--euv-keyboard-height").ok())
                    .and_then(|raw| {
                        let trimmed = raw.trim().trim_end_matches("px").to_string();
                        trimmed.parse::<f64>().ok()
                    })
                    .unwrap_or(0.0);
                let visible_bottom: f64 = viewport_height - keyboard_height - Self::FOCUS_GAP_PX;
                if input_bottom > visible_bottom && visible_bottom > 0.0 {
                    let scroll_amount: f64 = input_bottom - visible_bottom;
                    window_clone.scroll_by_with_x_and_y(0.0, scroll_amount);
                }
            }));
            let _: Result<i32, JsValue> = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref::<Function>(),
                    Self::FOCUS_SCROLL_DELAY_MILLIS,
                );
            closure.forget();
        }))
    }

    /// Blur handler that strips the inline `padding-bottom` injected by
    /// [`Self::on_focus_scroll_into_view`] so the page returns to its
    /// native layout once the keyboard closes.
    pub fn on_blur_restore_height() -> Option<Rc<dyn Fn(Event)>> {
        Some(Rc::new(move |event: Event| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(element) = target.dyn_into::<HtmlElement>() else {
                return;
            };
            if let Ok(Some(main_el)) = element.closest("main")
                && let Ok(main) = main_el.dyn_into::<HtmlElement>()
            {
                let _: Result<String, JsValue> = main.style().remove_property("padding-bottom");
            }
        }))
    }
}
