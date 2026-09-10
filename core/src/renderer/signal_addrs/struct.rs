use super::*;

/// Rust-side registry of bridge signal addresses per element.
///
/// Replaces the per-element `data-euv-signal-addrs` DOM attribute round-trip:
/// the mount path pushes directly into a global `HashMap<euv_id, Vec<usize>>`,
/// and the cleanup path reads it back to drive `Signal::clear_listeners`.
/// No JS-boundary crossings are required for signal address bookkeeping.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct SignalAddrs;

/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct SignalAddrMapCell(
    /// Interior-mutable storage for the signal-address registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<HashMap<usize, Vec<usize>>>,
);
