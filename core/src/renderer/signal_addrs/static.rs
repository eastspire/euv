use super::*;

/// Global registry mapping each mounted element's `euv_id` to the
/// `Vec<usize>` of bridge signal inner addresses bound to it.
///
/// Populated by `ElementExt::track_signal_addr` (Rust-side push) and
/// drained by `Renderer::cleanup_subtree` (Rust-side take). This
/// replaces the per-element `data-euv-signal-addrs` DOM attribute that
/// used to require one `get_attribute` + `set_attribute` JS crossing
/// per signal subscription.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-
/// threaded context).
pub(crate) static mut SIGNAL_ADDR_MAP: LazyLock<SignalAddrMapCell> =
    LazyLock::new(|| SignalAddrMapCell(UnsafeCell::new(HashMap::new())));
