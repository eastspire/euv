use super::*;

/// Cached table of JS batched DOM-op helpers.
///
/// Resolved lazily on first patch via [`ensure_dom_op_table`]. The
/// functions accept parallel arrays / op-tuples so a single JS-side
/// loop replaces N individual `setAttribute` / `removeAttribute` /
/// `insertBefore` / `appendChild` / `removeChild` crossings.
#[derive(Clone)]
pub(crate) struct DomOpTable {
    /// `setAttribute` batch helper: `(elem, names[], values[])`.
    pub(crate) set_attrs: Function,
    /// `removeAttribute` batch helper: `(elem, names[])`.
    pub(crate) remove_attrs: Function,
    /// Child-mutation batch helper: `(parent, ops[])`.
    pub(crate) child_ops: Function,
}

/// A single child-mutation op for [`apply_child_ops_batch`].
///
/// Each variant maps to exactly one web-sys call, preserving the
/// semantics of the previous per-op patch path:
/// - `InsertBefore { node, reference }` →
///   `parent.insert_before(node, Some(reference))` if reference is
///   `Some`, else `parent.append_child(node)`.
/// - `AppendChild(node)` → `parent.append_child(node)`.
/// - `RemoveChild(node)` → `parent.remove_child(node)`.
#[derive(Clone)]
pub(crate) enum ChildOp {
    /// `parent.insert_before(node, Some(reference))` — when `reference`
    /// is `None` this collapses to `append_child` on the JS side, but
    /// the JS glue handles the conversion explicitly to keep the
    /// helper's branching predictable.
    InsertBefore {
        /// The node to insert.
        node: Node,
        /// The reference node before which `node` is inserted, or
        /// `None` to append.
        reference: Option<Node>,
    },
    /// `parent.append_child(node)`.
    AppendChild(Node),
    /// `parent.remove_child(node)`.
    RemoveChild(Node),
}

/// `Sync` wrapper around `Option<DomOpTable>` for `thread_local!`
/// storage.
pub(crate) struct DomOpTableCell(pub(crate) UnsafeCell<Option<DomOpTable>>);

thread_local! {
    /// Per-thread cache for the JS batched DOM-op table. The first
    /// patch triggers the `Reflect::get(globalThis, "__euv_dom_ops__")`
    /// lookup (or installs the helpers if missing); subsequent patches
    /// reuse the cached functions without any further global lookup.
    pub static DOM_OP_TABLE_CELL: DomOpTableCell =
        DomOpTableCell(const { UnsafeCell::new(None) });
}
