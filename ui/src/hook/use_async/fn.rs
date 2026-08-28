use super::*;

/// Obtains a stand-alone (non-reactive) `UseAsyncHandle`.
///
/// Unlike the `HookContext`-bound variant described in the trait
/// doc-comment, this factory deliberately skips the hook slot and
/// falls back to a self-contained handle. That's the same code path
/// that `UseAsyncHandle::default()` uses internally, and it's the
/// only path that compiles today (`HookContext::use_async` is on
/// the roadmap but not yet wired in `euv-core`).
///
/// The returned handle is `Copy`, cheap to pass around, and exposes
/// `state()` / `set_state()` for non-async testing as well as
/// `refetch()` for the real wasm path. Render code that wants
/// reactive updates can still subscribe by reading `state()` inside
/// a render closure.
///
/// # Returns
///
/// - `UseAsyncHandle<T, L>` - The async handle in the `Loading`
///   initial state, ready for an `refetch(...)` from the call site.
pub fn use_async<T, L>() -> UseAsyncHandle<T, L>
where
    T: Clone + PartialEq + 'static,
    L: Clone + PartialEq + HasLoadingHint + 'static,
{
    UseAsyncHandle::new_for_fallback()
}
