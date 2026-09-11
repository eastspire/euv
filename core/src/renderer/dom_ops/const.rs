/// JS function name (keyed under `globalThis.__euv_dom_ops__`) for the
/// batched `setAttribute` applier.
///
/// `set_attrs(elem, names, values)` — `names` and `values` are parallel
/// `Array<string>` of equal length. The helper calls
/// `elem.setAttribute(names[i], values[i])` for every index. One JS
/// crossing per element replaces N individual `setAttribute` crossings.
pub(crate) const JS_DOM_OP_SET_ATTRS: &str = "__euv_dom_op_set_attrs";

/// JS function name (keyed under `globalThis.__euv_dom_ops__`) for the
/// batched `removeAttribute` applier.
///
/// `remove_attrs(elem, names)` — `names` is an `Array<string>`. The
/// helper calls `elem.removeAttribute(names[i])` for every index. One
/// JS crossing per element replaces N individual `removeAttribute`
/// crossings.
pub(crate) const JS_DOM_OP_REMOVE_ATTRS: &str = "__euv_dom_op_remove_attrs";

/// JS function name (keyed under `globalThis.__euv_dom_ops__`) for the
/// batched `insertBefore`/`appendChild` applier.
///
/// `apply_child_ops(parent, ops)` — `ops` is an `Array<OpKind>` where
/// `OpKind = { kind: "insertBefore", node, refNode? } | { kind:
/// "appendChild", node } | { kind: "removeChild", node }`. One JS
/// crossing per parent replaces N individual child-move crossings.
pub(crate) const JS_DOM_OP_CHILD_OPS: &str = "__euv_dom_op_child_ops";

/// JS object name (keyed under `globalThis`) that holds the
/// batched DOM-op helpers. Lazily populated by [`ensure_dom_op_table`]
/// on first patch.
pub(crate) const JS_DOM_OP_TABLE: &str = "__euv_dom_ops__";
