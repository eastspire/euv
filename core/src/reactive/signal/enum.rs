use super::*;

/// A single slab slot.
///
/// Free slots form a singly-linked free list: `next` is the index of the
/// next free slot, or `usize::MAX` to terminate. The `free_head` field of
/// [`SignalSlab`] points to the first free slot (or `usize::MAX` when no
/// free slots exist, in which case a new slot is pushed onto the
/// underlying `Vec`).
pub(crate) enum SignalSlot {
    /// Slot is currently free; `next` indexes the following free entry.
    Free { next: usize },
    /// Slot holds a live signal inner.
    Occupied(Box<dyn AnySignalInner>),
}
