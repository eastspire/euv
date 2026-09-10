use super::*;

/// Helper body of the `keyed_element` free function.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode` value.
fn keyed_element(key: &str, tag: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Owned(tag.to_string())),
        attributes: vec![],
        children: vec![],
        key: Some(key.to_string()),
        props: None,
    }
}
/// Helper body of the `unkeyed_element` free function.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode` value.
fn unkeyed_element(tag: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Owned(tag.to_string())),
        attributes: vec![],
        children: vec![],
        key: None,
        props: None,
    }
}

#[test]
fn node_key_returns_element_key() {
    let node: VirtualNode = keyed_element("a", "div");
    assert_eq!(node.key(), Some("a"));
}

#[test]
fn node_key_returns_none_for_unkeyed_element() {
    let node: VirtualNode = unkeyed_element("div");
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_text() {
    let node: VirtualNode = VirtualNode::Text(TextNode::new(Cow::Owned("hello".to_string()), None));
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_fragment() {
    let node: VirtualNode = VirtualNode::Fragment(vec![]);
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_empty() {
    let node: VirtualNode = VirtualNode::Empty;
    assert_eq!(node.key(), None);
}

#[test]
fn node_has_key_matches_node_key_some() {
    let node: VirtualNode = keyed_element("a", "div");
    assert!(node.has_key());
}

#[test]
fn node_has_key_matches_node_key_none() {
    let node: VirtualNode = unkeyed_element("div");
    assert!(!node.has_key());
}

#[test]
fn all_have_keys_true_for_all_keyed() {
    let children: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    assert!(all_have_keys(&children));
}

#[test]
fn all_have_keys_false_when_any_unkeyed() {
    let children: Vec<VirtualNode> = vec![keyed_element("a", "div"), unkeyed_element("span")];
    assert!(!all_have_keys(&children));
}

#[test]
fn all_have_keys_false_for_empty() {
    let children: Vec<VirtualNode> = vec![];
    assert!(!all_have_keys(&children));
}

#[test]
fn all_have_keys_false_for_single_unkeyed() {
    let children: Vec<VirtualNode> = vec![unkeyed_element("div")];
    assert!(!all_have_keys(&children));
}

#[test]
fn diff_children_keyed_when_both_have_keys() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn diff_children_positional_when_unkeyed() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn diff_children_keyed_when_old_unkeyed() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(ops, vec![DiffOp::Update { index: 0 }]);
}

#[test]
fn positional_no_change() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = old.clone();
    assert_eq!(
        diff_positional(&old, &new),
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn positional_insert_at_tail() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    assert_eq!(
        diff_positional(&old, &new),
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: unkeyed_element("span"),
            },
        ]
    );
}

#[test]
fn positional_remove_from_tail() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 1 },]
    );
}

#[test]
fn positional_empty_to_non_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: unkeyed_element("div"),
            },
            DiffOp::Insert {
                index: 1,
                node: unkeyed_element("span"),
            },
        ]
    );
}

#[test]
fn positional_non_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Remove { index: 1 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn positional_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![];
    assert!(diff_positional(&old, &new).is_empty());
}

#[test]
fn keyed_no_change() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = old.clone();
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn keyed_insert_at_tail() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Insert {
                index: 2,
                node: keyed_element("c", "p"),
            },
        ]
    );
}

#[test]
fn keyed_insert_at_head() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![
        keyed_element("z", "h1"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("z", "h1"),
            },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_remove_from_tail() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 1 },]
    );
}

#[test]
fn keyed_remove_from_head() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn keyed_swap_two() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span"), keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn keyed_reorder_three() {
    let old: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let new: Vec<VirtualNode> = vec![
        keyed_element("c", "p"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_insert_and_remove() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("c", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("c", "p"),
            },
            DiffOp::Remove { index: 1 },
        ]
    );
}

#[test]
fn keyed_replace_all() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("x", "h1"), keyed_element("y", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("x", "h1"),
            },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("y", "p"),
            },
            DiffOp::Remove { index: 1 },
            DiffOp::Remove { index: 0 },
        ]
    );
}

#[test]
fn keyed_empty_to_non_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("a", "div"),
            },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("b", "span"),
            },
        ]
    );
}

#[test]
fn keyed_non_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Remove { index: 1 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn keyed_single_element_no_change() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    assert_eq!(diff_keyed(&old, &new), vec![DiffOp::Update { index: 0 }]);
}

#[test]
fn keyed_single_element_replaced() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("b", "span"),
            },
            DiffOp::Remove { index: 0 },
        ]
    );
}

#[test]
fn keyed_move_from_tail_to_head() {
    let old: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let new: Vec<VirtualNode> = vec![
        keyed_element("c", "p"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_keeps_ops_in_walk_order() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span"), keyed_element("c", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("c", "p"),
            },
            DiffOp::Remove { index: 0 },
        ]
    );
}
