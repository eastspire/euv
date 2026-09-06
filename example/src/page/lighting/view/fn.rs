use super::*;

/// A standalone CPU Phong shading demo page.
///
/// Renders a 320x240 scene of five Phong spheres plus a ground line
/// into the Canvas 2D backing buffer using a per-pixel lighting pass:
/// for every sphere pixel, the surface normal is reconstructed from
/// the screen-space position and fed to `LightingUniforms::shade`,
/// which sums ambient + Lambert + Phong contributions from one
/// directional sun and one point lamp. Mirrors the existing
/// `/raytrace` tab shape: stats bar, canvas, pause/resume, and a
/// dedicated Enter Fullscreen button.
///
/// # Returns
///
/// - `VirtualNode` - The lighting page virtual DOM tree.
#[component]
pub(crate) fn page_lighting(node: VirtualNode<PageLightingProps>) -> VirtualNode {
    let _page_lighting_props: PageLightingProps = node.try_get_props().unwrap_or_default();
    let state: UseLighting = use_lighting_state();
    let fullscreen: UseLightingFullscreen = use_lighting_fullscreen_state();
    use_lighting_fullscreen_popstate(fullscreen);
    let fullscreen_signal: Signal<bool> = fullscreen.get_fullscreen();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_lighting_loop(state);
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = lighting_on_toggle_pause(state);
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let pause_label: &str = if state.get_running().get() {
        "Pause"
    } else {
        "Resume"
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "💡"
                title: "CPU Phong Lighting"
                subtitle: "A standalone CPU Phong shading demo powered by euv-engine's lighting module. Renders five spheres and a ground line into a 320x240 backing buffer at ~24 FPS using a per-pixel lighting pass: every sphere pixel reconstructs the surface normal from the screen-space position and feeds it to LightingUniforms::shade, which sums ambient + Lambert + Phong contributions from one directional sun and one point lamp. No shaders, no GPU pipeline — just Rust math. Click Enter Fullscreen for a larger view."
            }
            euv_card {
                title: "Lighting Demo"
                div {
                    class: c_game_stats_bar()
                    span {
                        class: c_game_stats_label()
                        "FPS: "
                        span {
                            class: c_game_stats_fps_value()
                            fps_display
                        }
                    }
                    span {
                        class: c_game_stats_label()
                        "Lights: 1 directional + 1 point"
                        span {
                            class: c_game_stats_count_value()
                        }
                    }
                }
                div {
                    class: if { fullscreen_signal.get() } {
                        c_game_container_fullscreen()
                    } else {
                        c_game_canvas_wrapper()
                    }
                    div {
                        class: c_game_fullscreen_canvas_wrapper()
                        div {
                            class: c_game_fullscreen_canvas_letterbox()
                            canvas {
                                id: LIGHTING_CANVAS_ID
                                class: if { fullscreen_signal.get() } {
                                    c_game_3d_canvas_fullscreen()
                                } else {
                                    c_game_3d_canvas()
                                }
                            }
                        }
                    }
                    if { fullscreen_signal.get() } {
                        div {
                            class: c_game_fullscreen_toolbar()
                            euv_button {
                                variant: EuvButtonVariant::Primary
                                label: "Exit"
                                onclick: lighting_on_exit_fullscreen(fullscreen)
                            }
                        }
                    }
                }
                div {
                    class: c_game_description()
                    "This demo uses euv-engine's lighting module to drive a per-pixel Phong shading pass directly on the Canvas 2D backing buffer: every sphere is rendered as a CPU pixel loop that reconstructs the surface normal from the screen-space position and feeds it to LightingUniforms::shade together with one directional sun and one point lamp. The ground line is shaded with the same pipeline (a fixed up-pointing normal) so the directional sun's side-light is clearly visible. No shaders, no GPU pipeline — just Rust math running through compute_lambert / compute_phong per pixel."
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: pause_label
                        onclick: on_toggle_pause
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Enter Fullscreen"
                        onclick: lighting_on_enter_fullscreen(fullscreen)
                    }
                }
            }
        }
    }
}
