use super::*;

/// Global typed signal slab. Single instance, lives for the program's
/// lifetime.
///
/// SAFETY: must only be accessed from the main thread (WASM single-threaded
/// context). Mirrors the contract on the prior `SIGNAL_INNER_REGISTRY`
/// static — every existing call site already relies on single-threaded
/// access because `Signal<T>::inner` was a raw pointer deref'd without
/// synchronization.
pub(crate) static mut SIGNAL_SLAB: LazyLock<UnsafeCell<SignalSlab>> =
    LazyLock::new(|| UnsafeCell::new(SignalSlab::new()));

/// Global reverse-index of `bridge_addr -> HashSet<source_addr>`.
///
/// Tracks which source signals currently hold a `subscribe` closure that
/// captures a given bridge signal's address. The bridge signal's slab slot
/// can be safely freed only when the entry for that bridge is empty AND
/// `clear_listeners` has been called on the bridge; in any other state, a
/// stale closure could dereference the freed slot index. The bridge index
/// here is a slab slot index (the same value carried by `Signal<T>::inner`
/// and round-tripped through `data-euv-signal-addrs`).
///
/// This is set when a bridge is created (`Signal::track_bridge_dependency`)
/// and consulted from `clear_listeners` and `Signal::deactivate`. Entries
/// are removed when the bridge is fully reclaimed.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded
/// context).
pub(crate) static mut BRIDGE_REFS: LazyLock<BridgeRefsCell> =
    LazyLock::new(|| BridgeRefsCell(UnsafeCell::new(HashMap::new())));
