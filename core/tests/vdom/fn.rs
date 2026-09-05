use super::*;
#[test]
fn css_debug_format_works() {
    let css: Css = Css::default();
    let formatted: String = format!("{:?}", css);
    assert!(formatted.contains("Css"));
}

#[test]
fn pseudo_rule_equality_same_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    assert_eq!(a, b);
}

#[test]
fn pseudo_rule_equality_different_selectors() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":focus"), String::from("background: blue;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_equality_different_styles() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: red;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_hash_same_for_equal_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn pseudo_rule_debug_format_works() {
    let rule: PseudoRule = PseudoRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("PseudoRule"));
}

#[test]
fn media_rule_equality_same_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_eq!(a, b);
}

#[test]
fn media_rule_equality_different_queries() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(min-width: 768px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_ne!(a, b);
}

#[test]
fn media_rule_hash_same_for_equal_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn media_rule_debug_format_works() {
    let rule: MediaRule = MediaRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("MediaRule"));
}

#[test]
fn attribute_entry_debug_format_works() {
    let entry: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(String::from("btn")),
    );
    let formatted: String = format!("{:?}", entry);
    assert!(formatted.contains("AttributeEntry"));
}

#[test]
fn opt10_static_text_borrows_without_alloc() {
    // OPT 10: StaticText must wrap a `&'static str` without allocating.
    let value: AttributeValue = AttributeValue::StaticText("color: red;");
    match value {
        AttributeValue::StaticText(s) => assert_eq!(s, "color: red;"),
        _ => panic!("expected AttributeValue::StaticText"),
    }
}

#[test]
fn opt10_static_text_matches_text_in_debug_layout() {
    // OPT 10: cloning an AttributeValue must preserve the StaticText
    // variant verbatim (no implicit to_string() during the clone).
    // Since `AttributeValue` derives `CustomDebug` which wraps each
    // variant, pattern-matching through the Debug output guarantees
    // the variant survived the clone.
    let original: AttributeValue = AttributeValue::StaticText("color:red;");
    let cloned: AttributeValue = original.clone();
    let original_dbg: String = format!("{:?}", original);
    let cloned_dbg: String = format!("{:?}", cloned);
    assert!(
        original_dbg.contains("StaticText"),
        "expected original Debug to mention StaticText, got: {original_dbg}"
    );
    assert_eq!(
        original_dbg, cloned_dbg,
        "AttributeValue::clone changed the variant"
    );
    assert!(
        cloned_dbg.contains("color:red;"),
        "expected cloned value to retain the literal string, got: {cloned_dbg}"
    );
}

#[test]
fn opt11_from_static_css_yields_cssref_not_css() {
    // OPT 11: `From<&'static Css>` must produce `CssRef`, not the
    // owned `Css` variant (which would have deep-cloned the payload).
    use std::sync::LazyLock;
    static STATIC_CSS: LazyLock<Css> = LazyLock::new(|| {
        Css::new(
            String::from("opt11-fixture"),
            String::from("color: blue;"),
            Vec::new(),
            Vec::new(),
        )
    });
    let value: AttributeValue = (&*STATIC_CSS).into();
    match value {
        AttributeValue::CssRef(css_ref) => {
            assert_eq!(css_ref.get_name(), "opt11-fixture");
            assert_eq!(css_ref.get_style(), "color: blue;");
        }
        AttributeValue::Css(_) => panic!(
            "OPT 11 regression: `&'static Css` produced owned Css \
             variant; expected CssRef to skip the deep clone."
        ),
        other => panic!("expected AttributeValue::CssRef, got {other:?}"),
    }
}

#[test]
fn opt11_cssref_does_not_clone_inner_collections() {
    // OPT 11: constructing an `AttributeValue::CssRef(&'static Css)`
    // must keep the inner `Vec<PseudoRule>` / `Vec<MediaRule>` storage
    // shared with the source (i.e. it must NOT call clone on them).
    use std::sync::LazyLock;
    static STATIC_CSS: LazyLock<Css> = LazyLock::new(|| {
        Css::new(
            String::from("opt11-shared"),
            String::from("display: flex;"),
            Vec::new(),
            Vec::new(),
        )
    });
    let value: AttributeValue = AttributeValue::CssRef(&*STATIC_CSS);
    let AttributeValue::CssRef(css_ref) = value else {
        panic!("expected AttributeValue::CssRef");
    };
    // Pointer identity check — borrowed storage must be the same object.
    assert!(std::ptr::eq(
        css_ref as *const Css,
        &*STATIC_CSS as *const Css
    ));
}

#[test]
fn native_css_construct_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let _: Css = Css::default();
        let _: PseudoRule = PseudoRule::default();
        let _: MediaRule = MediaRule::default();
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_pseudo_rule_clone_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let rule: PseudoRule = PseudoRule::default();
        let cloned: PseudoRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_media_rule_clone_does_not_panic() {
    let result: Result<(), String> = catch_unwind(AssertUnwindSafe(|| {
        let rule: MediaRule = MediaRule::default();
        let cloned: MediaRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}
