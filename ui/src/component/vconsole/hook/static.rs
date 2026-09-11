use super::*;

thread_local! {
    /// OPT-23: shared mutable storage backing the vConsole log signal.
    ///
    /// `Console::push` appends to this `RefCell` in place, then re-broadcasts
    /// the snapshot via the public `Signal` so existing reactive subscribers
    /// re-render. The `RefCell` is `thread_local` so it sidesteps the
    /// `Rc` / `RefCell` `Sync` requirement that a plain `OnceLock` would
    /// hit (we are single-threaded WASM, but the Rust checker doesn't know).
    pub(crate) static CONSOLE_LOG_REF: RefCell<Option<Rc<RefCell<Vec<ConsoleEntry>>>>>
        = const { RefCell::new(None) };
}

/// Global storage for the Console log signal.
///
/// Initialized via `init_console` and accessed through `get_console_signal`.
/// Uses `SignalCell` for safe single-threaded WASM contexts without raw pointers.
pub(crate) static CONSOLE_LOG_SIGNAL: SignalCell<Vec<ConsoleEntry>> = SignalCell::none();

/// OPT-23: returns a clone of the shared `Rc<RefCell<Vec<ConsoleEntry>>>`
/// that backs the vConsole log signal, or `None` if `Console::init` has
/// not yet installed it.
pub(crate) fn console_log_ref() -> Option<Rc<RefCell<Vec<ConsoleEntry>>>> {
    CONSOLE_LOG_REF.with(|cell| cell.borrow().clone())
}

/// OPT-23: installs the shared `Rc<RefCell<Vec<ConsoleEntry>>>` backing
/// store for the vConsole log signal. Called from `Console::init`.
pub(crate) fn install_console_log_ref(logs_ref: Rc<RefCell<Vec<ConsoleEntry>>>) {
    CONSOLE_LOG_REF.with(|cell| {
        *cell.borrow_mut() = Some(logs_ref);
    });
}
