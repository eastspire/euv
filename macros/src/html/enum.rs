use super::*;
use std::fmt::{self, Debug, Formatter};

fn fmt_lit_str(lit: &syn::LitStr, formatter: &mut Formatter<'_>) -> fmt::Result {
    write!(formatter, "Text({:?})", lit.value())
}

/// Debug formatting for `HtmlNode`.
///
/// Hand-rolled because `HtmlNode::Text` carries a `proc_macro2::LitStr`
/// token (not a plain `String`) and we don't want to drag a full
/// `TokenStream` formatter into the derive output. Other variants
/// delegate to their `Debug` impls.
impl Debug for HtmlNode {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element(element) => formatter.debug_tuple("Element").field(element).finish(),
            Self::Text(lit) => fmt_lit_str(lit, formatter),
            Self::Expr(expr) => formatter.debug_tuple("Expr").field(expr).finish(),
            Self::Dynamic(expr) => formatter.debug_tuple("Dynamic").field(expr).finish(),
            Self::If(html_if) => formatter.debug_tuple("If").field(html_if).finish(),
            Self::Match(html_match) => formatter.debug_tuple("Match").field(html_match).finish(),
            Self::For(html_for) => formatter.debug_tuple("For").field(html_for).finish(),
            Self::DynamicTag(dynamic_tag) => formatter
                .debug_tuple("DynamicTag")
                .field(dynamic_tag)
                .finish(),
        }
    }
}

/// Represents a single HTML node, which may be an element or text.
///
/// Parsed from the `html!` macro input before code generation.
#[derive(Clone)]
pub(crate) enum HtmlNode {
    /// An HTML element.
    Element(HtmlElement),
    /// A text string literal.
    ///
    /// OPT 29: stores the original `LitStr` token (not the parsed
    /// `String`) so the codegen can re-emit the literal source as a
    /// `Cow::Borrowed(&'static str)` constant — the binary embeds the
    /// string once and every render reuses the same slice instead of
    /// allocating a fresh `String` per text node.
    Text(syn::LitStr),
    /// A bare Rust expression (identifiers without braces), converted to a
    /// `VirtualNode` via `IntoNode::into_node`. This is a static one-shot
    /// conversion — no re-rendering on signal changes.
    Expr(Expr),
    /// A braced Rust expression `{expr}` in a child position, automatically
    /// wrapped into a `DynamicNode` that re-renders when signals change.
    /// The expression is evaluated inside a `move || { ... }` closure each
    /// time the dynamic node's render function is called.
    Dynamic(Expr),
    /// A reactive conditional: `if {signal} { html... } else if {signal} { html... } else { html... }`.
    ///
    /// Each condition is a signal expression in braces. When any signal changes,
    /// the entire conditional is re-evaluated and wrapped in a `DynamicNode`.
    If(HtmlIf),
    /// A reactive match: `match {signal} { pattern => { html... } ... }`.
    ///
    /// The signal expression in braces is re-evaluated on change, and the
    /// matching arm's HTML is rendered inside a `DynamicNode`.
    Match(HtmlMatch),
    /// A reactive for loop: `for pattern in {iterable} { html... }` or
    /// `for pattern in iterable { html... }`.
    ///
    /// The pattern is a Rust binding pattern (e.g., `item` or `(index, item)`).
    /// The iterable expression may be wrapped in braces (reactive) or written
    /// as a bare expression. Each iteration's HTML is collected into a
    /// `DynamicNode` fragment.
    For(HtmlFor),
    /// A dynamic tag: `{tag_expr} { attr: value, ... children ... }`.
    ///
    /// The expression in braces evaluates to a tag name string at runtime.
    /// If the tag name matches a registered user component, the component
    /// function is called with the provided attributes and children.
    /// Otherwise, a native HTML element is created.
    DynamicTag(HtmlDynamicTag),
}

/// Represents the value side of an attribute.
///
/// Supports plain expressions, style objects, reactive/inline conditionals,
/// reactive/inline match expressions, and merged multi-class/multi-style attribute values.
#[derive(Clone, Debug)]
pub(crate) enum HtmlAttrValue {
    /// A normal Rust expression.
    Expr(Expr),
    /// A conditional: `if {expr} { value }` (reactive) or `if condition { value }` (inline).
    If(HtmlAttrIf),
    /// A match expression: `match {expr} { ... }` (reactive) or `match expr { ... }` (inline).
    Match(HtmlAttrMatch),
    /// A style object: `{key: value; key2: value2;}`.
    ///
    /// The value can be either a string literal or an expression.
    Style(Vec<(String, HtmlStylePropValue)>),
    /// Multiple class attribute values merged from repeated `class:` declarations.
    ///
    /// Each entry is an independent expression (e.g., `c_foo()`, `c_bar()`).
    Classes(Vec<HtmlAttrValue>),
    /// Multiple style attribute values merged from repeated `style:` declarations.
    ///
    /// Each entry is an independent `Style` value.
    Styles(Vec<HtmlAttrValue>),
}

/// Represents a single value in a style property.
///
/// May be a static string literal, a dynamic expression, a reactive/inline conditional,
/// or a reactive/inline match expression.
#[derive(Clone, Debug)]
pub(crate) enum HtmlStylePropValue {
    /// A static string literal.
    Literal(String),
    /// A dynamic expression.
    Expr(Expr),
    /// A conditional in attribute value position.
    ///
    /// Syntax: `if {expr} { value }` (reactive) or `if condition { value }` (inline).
    If(HtmlAttrIf),
    /// A match expression in attribute value position.
    ///
    /// Syntax: `match {expr} { ... }` (reactive) or `match expr { ... }` (inline).
    Match(HtmlAttrMatch),
}

/// Determines how `attr_if_to_tokens` wraps each branch body during code generation.
///
/// - `Reactive` - Each branch body is wrapped with `IntoReactiveString::into_reactive_string()`,
///   ensuring all branches produce a `String`. Used for `class` and `style` attributes
///   where the `if` and implicit `else` branches may return different types
///   (e.g., `Css` vs `&str`).
/// - `Raw` - Branch bodies are emitted as-is without wrapping. Used for component props
///   where the branch types are already consistent or handled externally.
#[derive(Clone, Copy, Debug, Default, DisplayDebug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum AttrIfMode {
    /// Wrap each branch body with `IntoReactiveString::into_reactive_string()`.
    Reactive,
    /// Emit branch bodies as-is without any wrapping.
    #[default]
    Raw,
}
