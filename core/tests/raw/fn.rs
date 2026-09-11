use super::*;

#[test]
fn raw_html_stores_content_verbatim() {
    let raw: RawHtml = RawHtml::new("<svg viewBox=\"0 0 10 10\"/>".to_string());
    assert_eq!(raw.get_content(), "<svg viewBox=\"0 0 10 10\"/>");
}

#[test]
fn raw_html_default_is_empty_string() {
    let raw: RawHtml = RawHtml::default();
    assert_eq!(raw.get_content(), "");
}

#[test]
fn raw_html_empty_string_is_empty_string() {
    let raw: RawHtml = RawHtml::new(String::new());
    assert_eq!(raw.get_content(), "");
}

#[test]
fn raw_html_unicode_payload() {
    let raw: RawHtml = RawHtml::new("<p>你好 🌍</p>".to_string());
    assert_eq!(raw.get_content(), "<p>你好 🌍</p>");
}

#[test]
fn raw_html_plain_text() {
    let raw: RawHtml = RawHtml::new("hello world".to_string());
    assert_eq!(raw.get_content(), "hello world");
}

#[test]
fn raw_html_preserves_special_chars() {
    let raw: RawHtml = RawHtml::new("<a href=\"x?a=1&b=2\">link</a>".to_string());
    assert_eq!(raw.get_content(), "<a href=\"x?a=1&b=2\">link</a>");
}

#[test]
fn raw_html_clone() {
    let raw: RawHtml = RawHtml::new("<svg/>".to_string());
    let cloned: RawHtml = raw.clone();
    assert_eq!(cloned.get_content(), "<svg/>");
    assert_eq!(cloned, raw);
}

#[test]
fn raw_html_display() {
    let raw: RawHtml = RawHtml::new("<b>bold</b>".to_string());
    assert_eq!(format!("{}", raw), "<b>bold</b>");
}

#[test]
fn raw_html_debug_includes_type_name() {
    let raw: RawHtml = RawHtml::new("<svg/>".to_string());
    let s: String = format!("{:?}", raw);
    assert!(s.contains("RawHtml"));
}

#[test]
fn raw_html_into_text_node_carries_content() {
    let raw: RawHtml = RawHtml::new("<svg/>".to_string());
    let node: VirtualNode =
        VirtualNode::Text(TextNode::new(Cow::Owned(raw.get_content().clone()), None));
    let s: String = format!("{:?}", node);
    assert!(s.contains("svg"));
}
