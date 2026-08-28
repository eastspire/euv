use super::*;

/// The aggregate i18n state.
///
/// Constructed once per app via `App::use_i18n()`, threaded
/// through any code that needs to render translated text.
/// Cheap to `Clone` (the internal signal is
/// `Copy`-by-pointer).
///
/// # Storage
///
/// Messages are stored in a `Signal<HashMap<String,
/// HashMap<String, String>>>` keyed by `locale → (key →
/// message)`. The map is intentionally two-level: it
/// matches how translation files are typically organized
/// (`en.json` is one flat key/value map, `zh-CN.json` is
/// another, etc.) and it lets `add_messages(locale,
/// &[(k, v), ...])` build the inner map by direct
/// insertion without the user having to allocate one
/// `HashMap` per locale.
/// `I18n` is `Copy` because every field is a `Signal`, which is
/// already `Copy` — the registry hands out cheap `usize`
/// addresses for any `T: Clone + PartialEq + 'static`.
impl Copy for I18n {}

#[derive(Clone, Data, New)]
pub struct I18n {
    /// The currently-active locale tag. Setting this
    /// via `set_locale` triggers a reactive update that
    /// re-evaluates any reactive `t(...)` read.
    pub(crate) locale: Signal<String>,
    /// The locale to fall back to when a key is missing
    /// in the active locale. Defaults to `"en"`.
    pub(crate) fallback_locale: Signal<String>,
    /// The full translation table. The outer key is the
    /// locale tag, the inner key is the message key, the
    /// inner value is the (optionally placeholder-
    /// containing) message.
    pub(crate) messages: Signal<HashMap<String, HashMap<String, String>>>,
}
