use super::*;

/// The string value the async page resolves into on `Resolve`.
pub(crate) const HOOKS_ASYNC_RESOLVED_VALUE: &str = "hello from use_async";

/// The error message the async page fails with on `Fail`.
pub(crate) const HOOKS_ASYNC_FAIL_MESSAGE: &str = "demo failure";

/// Returns a click handler that triggers an `Ok("...")` future
/// and writes it into the supplied `UseAsyncHandle`.
pub(crate) fn hooks_async_refetch(handle: UseAsyncHandle<String, ()>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.set_state(AsyncState::<String, ()>::Ok(String::from(
            HOOKS_ASYNC_RESOLVED_VALUE,
        )));
    }))
}

/// Reads the current `AsyncState` and shapes it into a readable
/// string for the demo card.
pub(crate) fn hooks_async_state_label(handle: UseAsyncHandle<String, ()>) -> String {
    match handle.state() {
        AsyncState::<String, ()>::Loading(_) => String::from("Loading"),
        AsyncState::<String, ()>::Ok(value) => format!("Ok({value:?})"),
        AsyncState::<String, ()>::Err(err) => format!("Err({err:?})"),
    }
}

/// Returns `true` when the underlying `LazyComponent`'s factory
/// has not yet produced a value.
///
/// Coerces to a `bool` so the call site can use the result
/// directly inside a span / VirtualNode `{}` slot.
pub(crate) fn hooks_async_lazy_is_pending(lazy: &LazyComponent<u32>) -> bool {
    lazy.get().is_none()
}

/// Returns the loaded `LazyComponent` value as a `String` for the
/// demo card. Falls back to `"pending"` when the factory has not
/// fired yet.
pub(crate) fn hooks_async_lazy_loaded_label(lazy: &LazyComponent<u32>) -> String {
    lazy.loaded()
        .map(|value: u32| value.to_string())
        .unwrap_or_else(|| String::from("pending"))
}

/// Reads `SuspenseHandle`'s phase and shapes it into a readable
/// string for the demo card.
pub(crate) fn hooks_async_suspense_phase_label(handle: &SuspenseHandle<String>) -> String {
    match handle.get_phase().get() {
        SuspensePhase::Pending => String::from("Pending"),
        SuspensePhase::Resolved(value) => format!("Resolved({value})"),
        SuspensePhase::Failed(message) => format!("Failed({message})"),
    }
}

/// Builds the click handler that flips the suspense handle to
/// `Resolved`.
pub(crate) fn hooks_async_resolve(
    handle: SuspenseHandle<String>,
    value: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.resolve_sync(value.clone());
    }))
}

/// Builds the click handler that flips the suspense handle to
/// `Failed`.
pub(crate) fn hooks_async_fail(
    handle: SuspenseHandle<String>,
    message: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.fail(message.clone());
    }))
}

/// Builds the click handler that resets the suspense handle
/// back to `Pending`.
pub(crate) fn hooks_async_reset(handle: SuspenseHandle<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        handle.reset();
    }))
}
