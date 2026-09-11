use super::*;

impl SignalAddrs {
    /// Appends `addr` to the address list for `euv_id`.
    ///
    /// Allocates a new entry if `euv_id` is not yet known. Idempotent
    /// within a single mount cycle — the same `(euv_id, addr)` pair may
    /// be pushed multiple times by repeat subscribers; dedup happens at
    /// cleanup time when `take` is called.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    /// - `usize` - The signal inner pointer address to track.
    #[allow(static_mut_refs)]
    pub(crate) fn push(euv_id: usize, addr: usize) {
        unsafe {
            let map_mut_ptr: *mut HashMap<usize, Vec<usize>> =
                SIGNAL_ADDR_MAP.deref().get_0().get();
            let map: &mut HashMap<usize, Vec<usize>> = &mut *map_mut_ptr;
            map.entry(euv_id).or_insert_with(Vec::new).push(addr);
        }
    }

    /// Removes and returns every tracked address for `euv_id`.
    ///
    /// `Some(vec)` when the element had tracked signal addresses; `None`
    /// when the element was never registered (the common case for static
    /// subtrees that never bound a signal attribute).
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's `data-euv-id` value.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<usize>>` - `Some(...)` on success, `None` otherwise.
    #[allow(static_mut_refs)]
    pub(crate) fn take(euv_id: usize) -> Option<Vec<usize>> {
        unsafe {
            let map_mut_ptr: *mut HashMap<usize, Vec<usize>> =
                SIGNAL_ADDR_MAP.deref().get_0().get();
            let map: &mut HashMap<usize, Vec<usize>> = &mut *map_mut_ptr;
            map.remove(&euv_id)
        }
    }
}
