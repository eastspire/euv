use super::*;
use std::time::Instant;

/// Quiet period for the debounce row (ms).
pub(crate) const TIMING_DEBOUNCE_MS: u32 = 300;

/// Throttle window for the throttle row (ms).
pub(crate) const TIMING_THROTTLE_MS: u32 = 250;

/// Interval at which the ticks the `App::use_interval` driver
/// pushes the throttle / debounce state machine forward.
pub(crate) const TIMING_TICK_MS: i32 = 50;

/// Placeholder text shared by the live-input boxes.
pub(crate) const TIMING_INPUT_PLACEHOLDER: &str = "Type here…";

/// DOM id for the debounce row's input.
pub(crate) const TIMING_DEBOUNCE_INPUT_ID: &str = "timing-debounce-input";

/// DOM id for the throttle row's input.
pub(crate) const TIMING_THROTTLE_INPUT_ID: &str = "timing-throttle-input";

/// Creates an input handler that updates `live`, schedules a
/// debounce commit on `debounced`, and records `current` in
/// `previous` for the snapshot row to consume.
pub(crate) fn timing_debounce_on_input(
    live: Signal<String>,
    debounced: DebouncedValue<String>,
    current: Signal<String>,
    previous: Previous<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(value) = timing_read_input(&event) {
            live.set(value.clone());
            debounced.set(value.clone(), Instant::now());
            let snapshot: Option<String> = previous_step(previous, value.clone());
            let next_current: String = match snapshot {
                Some(prev) => format!("{prev} → {value}"),
                None => value,
            };
            current.set(next_current);
        }
    }))
}

/// Creates an input handler that drives the throttle row.
pub(crate) fn timing_throttle_on_input(
    live: Signal<String>,
    throttled: ThrottledValue<String>,
    current: Signal<String>,
    previous: Previous<String>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let captured_live: Signal<String> = live;
        let captured_throttled: ThrottledValue<String> = throttled;
        let captured_current: Signal<String> = current;
        let captured_previous: Previous<String> = previous;
        if let Some(value) = timing_read_input(&event) {
            captured_live.set(value.clone());
            captured_throttled.set(value.clone(), Instant::now());
            let snapshot: Option<String> = previous_step(captured_previous, value.clone());
            let next_current: String = match snapshot {
                Some(prev) => format!("{prev} → {value}"),
                None => value,
            };
            captured_current.set(next_current);
        }
    }))
}

/// Reads the current value from an `Event` whose target is an
/// `<input>` element. Returns `None` if the cast failed or the
/// event has no target.
fn timing_read_input(event: &Event) -> Option<String> {
    let target: JsValue = event.target()?.into();
    let input: HtmlInputElement = target.dyn_into::<HtmlInputElement>().ok()?;
    Some(input.value())
}

/// Returns the snapshot string the previous-value row displays.
///
/// `None` (no prior value) renders as `"—"`, `Some(value)` renders
/// as the value verbatim. Calling this also records the current
/// rendering's "current" value so subsequent renders see the value
/// that was on screen just before.
///
/// # Arguments
///
/// - `Previous<String>` - The previous-value tracker.
///
/// # Returns
///
/// - `String` - The display string for the current previous state.
pub(crate) fn timing_previous_snapshot(previous: Previous<String>) -> String {
    match previous.get_previous_snapshot() {
        Some(value) => value,
        None => String::from("—"),
    }
}

/// Reads a `Signal<String>` and returns the underlying
/// `String` value. Coerces to a text node for html! slots.
pub(crate) fn timing_signal_to_string(signal: &Signal<String>) -> String {
    signal.get()
}
