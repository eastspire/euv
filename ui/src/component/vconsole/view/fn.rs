use super::*;

/// Renders a vConsole-style floating debug panel with a toggle button and a half-page drawer.
///
/// The panel displays log entries from `Console::log`, `Console::warn`, and `Console::error`
/// calls. Provides level-based filtering with color-coded entries and clear/close actions.
/// When closed, a floating button with a badge showing the log count is rendered.
/// When open, a bottom drawer panel slides up showing all log entries with filter controls.
///
/// # Arguments
///
/// - `VirtualNode<EuvVconsolePanelProps>` - The props node containing the panel visibility signal.
///
/// # Returns
///
/// - `VirtualNode` - The vConsole panel virtual DOM tree.
#[component]
pub fn euv_vconsole_panel(node: VirtualNode<EuvVconsolePanelProps>) -> VirtualNode {
    let EuvVconsolePanelProps { panel_open }: EuvVconsolePanelProps =
        node.try_get_props().unwrap_or_default();
    let console_signal: Signal<Vec<ConsoleEntry>> =
        Console::get_signal().unwrap_or_else(|| Signal::create(Vec::new()));
    html! {
        euv_vconsole_fab {
            panel_open: panel_open
            console_signal: console_signal
        }
        euv_vconsole_drawer {
            console_signal: console_signal
            panel_open: panel_open
        }
    }
}

/// Renders the floating action button for the vConsole panel.
///
/// Uses the shared `euv_logo` component with the `Fab` variant
/// to display the branded "E" button with gradient background.
/// The button stays visible at all times, even when the drawer is open,
/// so the user can always tap it to toggle the panel.
///
/// # Arguments
///
/// - `VirtualNode<EuvVconsoleFabProps>` - The props node containing panel visibility signal and log count.
///
/// # Returns
///
/// - `VirtualNode` - The floating action button virtual DOM tree.
#[component]
pub fn euv_vconsole_fab(node: VirtualNode<EuvVconsoleFabProps>) -> VirtualNode {
    let EuvVconsoleFabProps {
        panel_open,
        console_signal,
    }: EuvVconsoleFabProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_vconsole_fab()
            euv_logo {
                variant: LogoButtonVariant::Fab
                on_click: Console::fab_on_click(panel_open)
                span {
                    class: if { !console_signal.get().is_empty() } {
                        c_vconsole_badge()
                    }
                    if { console_signal.get().len() > 99 } {
                        "99+"
                    } else if { !console_signal.get().is_empty() } {
                        console_signal.get().len().to_string()
                    }
                }
            }
        }
    }
}

/// Renders the vConsole drawer panel with log entries, level filter, and controls.
///
/// Both the overlay and the panel are always present in the DOM.
/// When closed the overlay uses `opacity:0` + `pointer-events:none` and the panel
/// uses `transform:translateY(100%)` so the browser has already performed layout;
/// opening only triggers a CSS transition instead of a costly DOM rebuild.
///
/// # Arguments
///
/// - `VirtualNode<EuvVconsoleDrawerProps>` - The props node containing console signal, panel visibility signal, and log count.
///
/// # Returns
///
/// - `VirtualNode` - The drawer panel virtual DOM tree.
#[component]
pub fn euv_vconsole_drawer(node: VirtualNode<EuvVconsoleDrawerProps>) -> VirtualNode {
    let EuvVconsoleDrawerProps {
        console_signal,
        panel_open,
    }: EuvVconsoleDrawerProps = node.try_get_props().unwrap_or_default();
    let filter_signal: Signal<LogFilter> = App::use_signal(|| LogFilter::All);
    let on_overlay_click = move |_: Event| {
        Router::overlay_stack_close();
        panel_open.set(false);
    };
    let on_clear_click = move |_: Event| {
        Console::clear();
    };
    let on_close_click = move |_: Event| {
        Router::overlay_stack_close();
        panel_open.set(false);
    };
    html! {
        div {
            div {
                class: if { panel_open.get() } {
                    c_vconsole_overlay().to_string()
                } else {
                    format!("{} {}", c_vconsole_overlay().get_name(), c_vconsole_overlay_hidden().get_name())
                }
                onclick: on_overlay_click
            }
            div {
                class: if { panel_open.get() } {
                    c_vconsole_panel().to_string()
                } else {
                    format!("{} {}", c_vconsole_panel().get_name(), c_vconsole_panel_closed().get_name())
                }
                div {
                    class: c_vconsole_header()
                    h3 {
                        class: c_vconsole_title()
                        span {
                            class: c_vconsole_title_dot()
                        }
                        "Console"
                        span {
                            class: c_vconsole_count()
                            format!(" ({})", console_signal.get().len())
                        }
                    }
                    div {
                        class: c_vconsole_header_actions()
                        button {
                            class: c_vconsole_close_button()
                            onclick: on_close_click
                            "×"
                        }
                    }
                }
                div {
                    class: c_vconsole_filter_bar()
                    span {
                        class: if { filter_signal.get() == LogFilter::All } {
                            c_vconsole_filter_badge()
                        } else {
                            c_vconsole_filter_badge_outline()
                        }
                        onclick: LogFilter::on_filter_all(filter_signal)
                        "All"
                    }
                    span {
                        class: if { filter_signal.get() == LogFilter::Log } {
                            c_vconsole_filter_badge()
                        } else {
                            c_vconsole_filter_badge_outline()
                        }
                        onclick: LogFilter::on_filter_log(filter_signal)
                        "Log"
                    }
                    span {
                        class: if { filter_signal.get() == LogFilter::Warn } {
                            c_vconsole_filter_badge()
                        } else {
                            c_vconsole_filter_badge_outline()
                        }
                        onclick: LogFilter::on_filter_warn(filter_signal)
                        "Warn"
                    }
                    span {
                        class: if { filter_signal.get() == LogFilter::Error } {
                            c_vconsole_filter_badge()
                        } else {
                            c_vconsole_filter_badge_outline()
                        }
                        onclick: LogFilter::on_filter_error(filter_signal)
                        "Error"
                    }
                    span {
                        class: c_vconsole_clear_button()
                        onclick: on_clear_click
                        "Clear"
                    }
                }
                div {
                    class: c_vconsole_body()
                    build_vconsole_log_nodes(console_signal, filter_signal)
                }
            }
        }
    }
}

/// Builds the vConsole log entry virtual nodes from the reactive log signal with level filtering.
///
/// Always renders both the empty-state and the log-list containers, toggling
/// visibility with CSS `height` + `overflow` so that the `if` arm-switch never triggers
/// `render_full_replace` when the first log entry arrives.
///
/// OPT-23: the previous implementation called `Console::filter_entries` three
/// times per render (once for the empty-state class branch, once for the
/// log-list class branch, and once for the `for` body) — each call cloned
/// the entire log Vec. With 200 entries that was 600 entry clones per
/// render. We now subscribe to `logs` + `filter` via `computed!` and the
/// resulting cached signal is read once per render.
///
/// # Arguments
///
/// - `Signal<Vec<ConsoleEntry>>` - A `Signal<Vec<ConsoleEntry>>` parameter.
/// - `Signal<LogFilter>` - A `Signal<LogFilter>` parameter.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode` value.
fn build_vconsole_log_nodes(
    logs: Signal<Vec<ConsoleEntry>>,
    filter: Signal<LogFilter>,
) -> VirtualNode {
    let cached_filter: Signal<Vec<(usize, ConsoleEntry)>> =
        computed!(logs, filter, |log_snapshot: Vec<ConsoleEntry>,
                                 current_filter: LogFilter|
         -> Vec<(usize, ConsoleEntry)> {
            let _ = log_snapshot;
            let _ = current_filter;
            Console::filter_entries(logs, filter)
        });
    let snapshot: Vec<(usize, ConsoleEntry)> = cached_filter.get();
    let snapshot_ref: &[(usize, ConsoleEntry)] = snapshot.as_slice();
    let snapshot_len: usize = snapshot_ref.len();
    let entries_iter = snapshot_ref.iter();
    html! {
        div {
            class: if { snapshot_len == 0 } {
                c_vconsole_empty()
            } else {
                c_vconsole_empty_hidden()
            }
            "No logs yet."
        }
        div {
            class: if { snapshot_len == 0 } {
                c_vconsole_log_list_hidden()
            } else {
                c_vconsole_log_list()
            }
            for (index, entry) in entries_iter {
                div {
                    key: index.to_string()
                    class: c_vconsole_log_item()
                    span {
                        class: c_vconsole_level_badge()
                        entry.get_level().badge()
                    }
                    entry.get_message().clone()
                }
            }
        }
    }
}
