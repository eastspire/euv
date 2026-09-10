/// Extension trait for `Element` providing DOM attribute/property manipulation methods.
///
/// Since Rust's orphan rules prevent adding inherent methods to foreign types like
/// `web_sys::Element`, this trait provides the same functionality through an extension
/// trait pattern. All methods are available on any `Element` reference via trait dispatch.
pub trait ElementExt {
    /// Removes or clears a DOM attribute/property, depending on the attribute name.
    ///
    /// For `value`, sets the DOM property to an empty string rather than calling
    /// `remove_attribute`, because `remove_attribute("value")` only removes the
    /// HTML attribute and does not clear the displayed value of input elements.
    /// For boolean properties (`checked`, `disabled`, `selected`, `readonly`),
    /// sets the DOM property to `false` rather than calling `remove_attribute`,
    /// because `remove_attribute` on a previously-set attribute may not correctly
    /// reset the property in all browsers.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to remove.
    fn remove_attribute_or_property(&self, name: &str);

    /// Sets a DOM attribute or property, depending on the attribute name.
    ///
    /// For `value`, uses the DOM property to ensure input elements update correctly.
    /// For boolean attributes (`checked`, `disabled`, `selected`, `readonly`),
    /// uses the DOM property so that the browser honors the value correctly
    /// (HTML attributes are present-or-absent, not true/false strings).
    /// For all other attributes, uses `set_attribute`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name of the attribute or property to set.
    /// - `&str` - The value to assign.
    fn set_attribute_or_property(&self, name: &str, value: &str);

    /// Tracks a signal address on the element for cleanup purposes.
    ///
    /// Stores the signal's inner address in the Rust-side
    /// [`crate::renderer::signal_addrs::SignalAddrs`] registry, keyed by
    /// the element's `data-euv-id`. The cleanup path reads from this
    /// registry instead of parsing a DOM attribute, eliminating the
    /// per-signal `get_attribute` + `set_attribute` JS-boundary crossings
    /// the previous `data-euv-signal-addrs` round-trip used to pay.
    ///
    /// # Arguments
    ///
    /// - `usize` - The signal's inner address to track.
    fn track_signal_addr(&self, addr: usize);
}
