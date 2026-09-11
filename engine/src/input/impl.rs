use super::*;

/// Implements static event extraction methods on the `Input` namespace struct.
impl Input {
    /// Extracts the key code string from a keyboard event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The keyboard event.
    ///
    /// # Returns
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`, `"ArrowLeft"`).
    pub fn extract_key_code(event: &Event) -> String {
        // OPT 39: typed web-sys getter avoids the JS Reflect::get crossing.
        event.unchecked_ref::<KeyboardEvent>().code()
    }

    /// Extracts the mouse button enum from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `MouseButton` - The mouse button that was pressed or released.
    pub fn extract_mouse_button(event: &Event) -> MouseButton {
        // OPT 39: typed web-sys `MouseEvent::button` getter instead of
        // Reflect::get + as_f64 cast.
        let button_value: i16 = event.unchecked_ref::<MouseEvent>().button();
        match button_value {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Button4,
            4 => MouseButton::Button5,
            _ => MouseButton::Left,
        }
    }

    /// Extracts the client (viewport) coordinates from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The `(x, y)` client coordinates.
    pub fn extract_mouse_position(event: &Event) -> Vector2D {
        // OPT 39: typed web-sys `MouseEvent::client_x` / `client_y` getters
        // instead of two Reflect::get + as_f64 casts per mouse event.
        let mouse_event: &MouseEvent = event.unchecked_ref::<MouseEvent>();
        let client_x: f64 = f64::from(mouse_event.client_x());
        let client_y: f64 = f64::from(mouse_event.client_y());
        Vector2D::new(client_x, client_y)
    }
}

/// Implements DOM event listener registration on the `Input` namespace struct.
///
/// This is the wiring layer that routes DOM events into [`InputState`]:
/// keyboard events bind to `window` (a `<canvas>` is not focusable by
/// default), while mouse and touch events bind to the canvas element so
/// hit-testing coordinates stay canvas-local. Registered closures are
/// `.forget()`-ed and stay alive for the lifetime of the document,
/// matching the engine's mount-only convention.
impl Input {
    /// Attaches all input listeners and returns the shared state cell.
    ///
    /// # Arguments
    ///
    /// - `InputStateCell` - The shared input state to mutate from event handlers.
    /// - `&Window` - The global window, receiving keyboard events.
    /// - `&EventTarget` - The pointer target (typically the canvas element).
    ///
    /// # Returns
    ///
    /// - `InputStateCell` - The same cell passed in, for convenient chaining.
    pub fn attach(
        state_cell: InputStateCell,
        window: &Window,
        pointer_target: &EventTarget,
    ) -> InputStateCell {
        Self::attach_keyboard(&state_cell, window);
        Self::attach_pointer(&state_cell, pointer_target);
        state_cell
    }

    /// Binds `keydown` / `keyup` listeners to `window`.
    ///
    /// Keyboard events must bind to `window` rather than the canvas: a
    /// `<canvas>` element is not focusable unless `tabindex` is set and the
    /// user clicks it, so canvas-bound key listeners would never fire.
    ///
    /// # Arguments
    ///
    /// - `&InputStateCell` - The shared input state.
    /// - `&Window` - The global window.
    pub fn attach_keyboard(state_cell: &InputStateCell, window: &Window) {
        let state_keydown: InputStateCell = state_cell.clone();
        let keydown_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let code: String = Input::extract_key_code(&event);
                if code.is_empty() {
                    return;
                }
                let state: &mut InputState = state_keydown.get_mut();
                state.press_key(code);
            }));
        Self::register_listener(window, INPUT_EVENT_KEYDOWN, keydown_closure);
        let state_keyup: InputStateCell = state_cell.clone();
        let keyup_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let code: String = Input::extract_key_code(&event);
                if code.is_empty() {
                    return;
                }
                let state: &mut InputState = state_keyup.get_mut();
                state.release_key(code);
            }));
        Self::register_listener(window, INPUT_EVENT_KEYUP, keyup_closure);
    }

    /// Binds mouse / touch / context-menu listeners to the pointer target.
    ///
    /// `touchstart` and `touchmove` call `prevent_default()` so the browser
    /// does not interpret touches as scroll/zoom gestures before the engine
    /// sees them. `contextmenu` is suppressed so right-click reaches the
    /// engine as `MouseButton::Right` instead of opening the browser menu.
    ///
    /// # Arguments
    ///
    /// - `&InputStateCell` - The shared input state.
    /// - `&EventTarget` - The pointer target (typically the canvas element).
    pub fn attach_pointer(state_cell: &InputStateCell, target: &EventTarget) {
        let state_mousedown: InputStateCell = state_cell.clone();
        let mousedown_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let button: MouseButton = Input::extract_mouse_button(&event);
                let position: Vector2D = Input::extract_mouse_position(&event);
                let state: &mut InputState = state_mousedown.get_mut();
                state.press_mouse_button(button, position);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEDOWN, mousedown_closure);
        let state_mouseup: InputStateCell = state_cell.clone();
        let mouseup_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let button: MouseButton = Input::extract_mouse_button(&event);
                let state: &mut InputState = state_mouseup.get_mut();
                state.release_mouse_button(button);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEUP, mouseup_closure);
        let state_mousemove: InputStateCell = state_cell.clone();
        let mousemove_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let position: Vector2D = Input::extract_mouse_position(&event);
                let state: &mut InputState = state_mousemove.get_mut();
                state.update_mouse_position(position);
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSEMOVE, mousemove_closure);
        let state_mouseleave: InputStateCell = state_cell.clone();
        let mouseleave_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |_: Event| {
                let state: &mut InputState = state_mouseleave.get_mut();
                state.set_mouse_moved(false);
                state.set_mouse_delta(Vector2D::zero());
            }));
        Self::register_listener(target, INPUT_EVENT_MOUSELEAVE, mouseleave_closure);
        let state_touchstart: InputStateCell = state_cell.clone();
        let touchstart_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
                let state: &mut InputState = state_touchstart.get_mut();
                for (identifier, position) in Input::extract_touch_positions(&event) {
                    state.start_touch(identifier, position);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHSTART, touchstart_closure);
        let state_touchmove: InputStateCell = state_cell.clone();
        let touchmove_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
                let state: &mut InputState = state_touchmove.get_mut();
                for (identifier, position) in Input::extract_touch_positions(&event) {
                    state.update_touch(identifier, position);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHMOVE, touchmove_closure);
        let state_touchend: InputStateCell = state_cell.clone();
        let touchend_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                let state: &mut InputState = state_touchend.get_mut();
                for identifier in Input::extract_touch_identifiers(&event) {
                    state.end_touch(identifier);
                }
            }));
        Self::register_listener(target, INPUT_EVENT_TOUCHEND, touchend_closure);
        let contextmenu_closure: Closure<dyn FnMut(Event)> =
            Closure::wrap(Box::new(move |event: Event| {
                event.prevent_default();
            }));
        Self::register_listener(target, INPUT_EVENT_CONTEXTMENU, contextmenu_closure);
    }

    /// Extracts `(identifier, position)` pairs for every changed touch.
    ///
    /// A single touch event can carry multiple changed touches, so this
    /// iterates the whole `changedTouches` list rather than reading index 0.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<(i32, Vector2D)>` - The identifier and client position of each changed touch.
    fn extract_touch_positions(event: &Event) -> Vec<(i32, Vector2D)> {
        let touch_event: &TouchEvent = event.unchecked_ref();
        let touches: TouchList = touch_event.changed_touches();
        let length: u32 = touches.length();
        let mut out: Vec<(i32, Vector2D)> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let Some(touch) = touches.get(index) else {
                continue;
            };
            let identifier: i32 = touch.identifier();
            let position: Vector2D =
                Vector2D::new(f64::from(touch.client_x()), f64::from(touch.client_y()));
            out.push((identifier, position));
        }
        out
    }

    /// Extracts the identifier of every changed touch (for `touchend`).
    ///
    /// # Arguments
    ///
    /// - `&Event` - The touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<i32>` - The identifier of each changed touch.
    fn extract_touch_identifiers(event: &Event) -> Vec<i32> {
        let touch_event: &TouchEvent = event.unchecked_ref();
        let touches: TouchList = touch_event.changed_touches();
        let length: u32 = touches.length();
        let mut out: Vec<i32> = Vec::with_capacity(length as usize);
        for index in 0..length {
            let Some(touch) = touches.get(index) else {
                continue;
            };
            out.push(touch.identifier());
        }
        out
    }

    /// Registers a closure on the target and leaks it for the document's lifetime.
    ///
    /// The engine follows the wasm single-page-mount convention: listeners
    /// are never detached, so the closure is `.forget()`-ed immediately
    /// after registration (its clone of the state cell keeps the cell
    /// reachable through the listeners even if the caller drops its `Rc`).
    ///
    /// # Arguments
    ///
    /// - `&EventTarget` - The DOM target to listen on.
    /// - `&str` - The DOM event name.
    /// - `Closure<dyn FnMut(Event)>` - The handler to register.
    fn register_listener(
        target: &EventTarget,
        event_name: &str,
        closure: Closure<dyn FnMut(Event)>,
    ) {
        let _: Result<(), JsValue> =
            target.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }
}

/// Implements input state management for `InputState`.
impl InputState {
    /// Records a key press event, adding to `keys_pressed` and `keys_held`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`).
    pub fn press_key(&mut self, key_code: String) {
        if !self.get_keys_held().contains(&key_code) {
            self.get_mut_keys_pressed().insert(key_code.clone());
        }
        self.get_mut_keys_held().insert(key_code);
    }

    /// Records a key release event, moving from `keys_held` to `keys_released`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string.
    pub fn release_key(&mut self, key_code: String) {
        self.get_mut_keys_held().remove(&key_code);
        self.get_mut_keys_released().insert(key_code);
    }

    /// Tests whether a key was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `&str` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was pressed this frame.
    pub fn is_key_pressed<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_pressed().contains(key_code.as_ref())
    }

    /// Tests whether a key is currently held down.
    ///
    /// # Arguments
    ///
    /// - `K: AsRef<str>` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key is held.
    pub fn is_key_held<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_held().contains(key_code.as_ref())
    }

    /// Tests whether a key was released during this frame.
    ///
    /// # Arguments
    ///
    /// - `K: AsRef<str>` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was released this frame.
    pub fn is_key_released<K>(&self, key_code: K) -> bool
    where
        K: AsRef<str>,
    {
        self.get_keys_released().contains(key_code.as_ref())
    }

    /// Records a mouse button press at the given position.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was pressed.
    /// - `Vector2D` - The mouse position.
    pub fn press_mouse_button(&mut self, button: MouseButton, position: Vector2D) {
        if !self.get_mouse_buttons_held().contains(&button) {
            self.get_mut_mouse_buttons_pressed().insert(button);
        }
        self.get_mut_mouse_buttons_held().insert(button);
        self.set_mouse_position(position);
    }

    /// Records a mouse button release.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was released.
    pub fn release_mouse_button(&mut self, button: MouseButton) {
        self.get_mut_mouse_buttons_held().remove(&button);
        self.get_mut_mouse_buttons_released().insert(button);
    }

    /// Updates the mouse position and computes the delta from the previous position.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The new mouse position.
    pub fn update_mouse_position(&mut self, position: Vector2D) {
        self.set_mouse_delta(position - self.get_mouse_position());
        self.set_mouse_position(position);
        self.set_mouse_moved(true);
    }

    /// Tests whether a mouse button was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was pressed this frame.
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_pressed().contains(&button)
    }

    /// Tests whether a mouse button is currently held down.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button is held.
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_held().contains(&button)
    }

    /// Adds or updates a touch point.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn update_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
    }

    /// Records a new touch point that started this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn start_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
        self.get_mut_touch_started().insert(identifier);
    }

    /// Removes a touch point and marks it as ended this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    pub fn end_touch(&mut self, identifier: i32) {
        self.get_mut_touch_points().remove(&identifier);
        self.get_mut_touch_ended().insert(identifier);
    }

    /// Returns the position of the lowest-identifier active touch point.
    ///
    /// Pointer-style consumers (the example pages' interactive demos) treat
    /// the primary touch like a mouse cursor: touch events never update
    /// `mouse_position`, so this accessor is the only public way to read a
    /// touch position.
    ///
    /// # Returns
    ///
    /// - `Option<Vector2D>` - The client-space position of the primary
    ///   touch, or `None` when no touch is active.
    pub fn primary_touch_position(&self) -> Option<Vector2D> {
        self.get_touch_points()
            .iter()
            .min_by_key(|(identifier, _)| **identifier)
            .map(|(_, position)| *position)
    }

    /// Clears all per-frame input data (pressed, released, deltas).
    ///
    /// Should be called at the end of each game frame after all input has been processed.
    pub fn end_frame(&mut self) {
        self.get_mut_keys_pressed().clear();
        self.get_mut_keys_released().clear();
        self.get_mut_mouse_buttons_pressed().clear();
        self.get_mut_mouse_buttons_released().clear();
        self.set_mouse_delta(Vector2D::zero());
        self.set_mouse_moved(false);
        self.get_mut_touch_started().clear();
        self.get_mut_touch_ended().clear();
    }
}

/// Implements `Default` for `InputState` as a fresh empty state.
impl Default for InputState {
    /// Constructs a default [`InputState`] value.
    ///
    /// # Returns
    ///
    /// - `InputState` - A default-constructed instance with the documented initial state.
    fn default() -> InputState {
        InputState::new()
    }
}
