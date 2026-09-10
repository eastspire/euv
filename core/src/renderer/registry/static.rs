use super::*;

/// Global auto-incrementing ID counter for DOM elements.
pub static NEXT_EUV_ID: AtomicUsize = AtomicUsize::new(0);

/// Global auto-incrementing ID counter for DynamicNode placeholder elements.
pub static NEXT_EUV_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(0);

/// Whether `dispatch_updates` is currently executing.
pub static SIGNAL_UPDATE_DISPATCHING: AtomicBool = AtomicBool::new(false);

/// Set of dynamic node IDs marked dirty since the last dispatch drain.
///
/// OPT 6: the dispatcher's hot path used to scan the entire signal
/// update registry (`HashMap<usize, SignalUpdateEntry>`) on every
/// tick to find slots whose `dirty` flag was set. For a SPA with N
/// dynamic nodes and only a handful of changed signals per tick, this
/// was an `O(N)` scan with a `*const SignalUpdateSlot` pointer
/// dereference + branch per entry. The dirty-set replaces it with a
/// single `HashSet::drain()` over the IDs that were actually marked
/// dirty — a `O(脏节点数)` operation that does no per-slot pointer
/// traversal until the matching slot is found.
///
/// Populated by `Registry::mark_dirty` and drained by
/// `Scheduler::dispatch_updates`. A redundant `dirty = true` set is a
/// no-op thanks to `HashSet` semantics. The set survives the dispatch
/// itself (the dirty flag is reset inside the loop); only the
/// `mark_dirty` path inserts into it.
pub(crate) static mut DIRTY_UPDATE_IDS: LazyLock<DirtyUpdateIdsCell> =
    LazyLock::new(|| DirtyUpdateIdsCell(UnsafeCell::new(HashSet::new())));

/// Global handler registry, mapping (element_id, event_name) to HandlerEntry.
pub(crate) static mut HANDLER_REGISTRY: LazyLock<HandlerRegistryCell> =
    LazyLock::new(|| HandlerRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global set of event names that have already been delegated at the window level.
pub(crate) static mut DELEGATED_EVENTS: LazyLock<DelegatedEventsCell> =
    LazyLock::new(|| DelegatedEventsCell(UnsafeCell::new(HashSet::new())));

/// Global signal update callback registry, mapping keys to SignalUpdateEntry.
pub(crate) static mut SIGNAL_UPDATE_REGISTRY: LazyLock<SignalUpdateRegistryCell> =
    LazyLock::new(|| SignalUpdateRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global auto-incrementing ID counter for window event handler entries.
pub static NEXT_WINDOW_HANDLER_ID: AtomicUsize = AtomicUsize::new(0);

/// Global window event proxy registry, mapping event names to handler lists.
pub(crate) static mut WINDOW_EVENT_REGISTRY: LazyLock<WindowEventRegistryCell> =
    LazyLock::new(|| WindowEventRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global `NodeRef` registry used to clear `NodeRef` handles when the
/// DOM element they point to is unmounted.
///
/// NP-3: each time a `ref:` attribute fires, the mount path registers
/// the `NodeRef`'s shared interior cell into this map under the
/// element's `euv_id`. `cleanup_subtree` then drains the entries for
/// that id and calls `NodeRef::clear` so `get()` / `get_cloned()` return
/// `None` after the underlying DOM subtree is gone.
pub(crate) static mut NODEREF_REGISTRY: LazyLock<NodeRefRegistryCell> =
    LazyLock::new(|| NodeRefRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global typed-attribute-bridge registry, mapping bridge signal addresses
/// to the `AttributeBridge` that the bridge signal's listener mutates on
/// every set.
///
/// Populated by `Registry::register_attribute_bridge` (called from the
/// per-`{sig}` mount paths in `Renderer::create_dom_with_doc`) and drained
/// by `Registry::cleanup_attribute_bridge` (called from
/// `Signal::<String>::clear_listeners`).
///
/// Replaces the older bridge-signal + `BridgeRefsCell::track` chain that
/// allocated a `HashSet<usize>` per bridge (the source-dependency set).
/// With this registry the bridge struct is freed at exactly the same time
/// as the bridge signal, and no per-source-signal `HashSet` is needed.
pub(crate) static mut ATTRIBUTE_BRIDGES: LazyLock<AttributeBridgesCell> =
    LazyLock::new(|| AttributeBridgesCell(UnsafeCell::new(HashMap::new())));
