use super::*;

/// A standalone software ray-tracing demo page.
///
/// Renders a 160x100 scene of one mirror sphere, one emissive sphere,
/// and a ground AABB into the Canvas 2D backing buffer. Each frame
/// re-traces every pixel using `raytracing::trace_default` so the
/// mirror reflection bounces off the ground plane (or emissive
/// sphere) and lands somewhere visibly different from the primary
/// hit. Mirrors the existing 2D / GL / GPU tab shape: stats bar,
/// canvas, pause/resume, and a dedicated Enter Fullscreen button.
///
/// # Returns
///
/// - `VirtualNode` - The raytrace page virtual DOM tree.
#[component]
pub(crate) fn page_raytrace(node: VirtualNode<PageRaytraceProps>) -> VirtualNode {
    let _page_raytrace_props: PageRaytraceProps = node.try_get_props().unwrap_or_default();
    let state: UseRayTrace = use_raytrace_state();
    let fullscreen: UseRayTraceFullscreen = use_raytrace_fullscreen_state();
    use_raytrace_fullscreen_popstate(fullscreen);
    let fullscreen_signal: Signal<bool> = fullscreen.get_fullscreen();
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_raytrace_loop(state);
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_pause(state);
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
                icon: "🔦"
                title: "Software Ray Tracing"
                subtitle: "A CPU-side software ray tracer rendering 1 mirror sphere, 1 emissive sphere, and 1 ground AABB into a 160x100 backing buffer at ~24 FPS. Each frame re-traces every pixel via raytracing::trace_default, bouncing the mirror reflection off the ground or the emissive sphere. Click Enter Fullscreen for a larger view."
            }
            euv_card {
                title: "RayTrace Demo"
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
                        "Scene: 1 mirror + 1 emissive + 1 ground"
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
                                id: RAYTRACE_CANVAS_ID
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
                                onclick: raytrace_on_exit_fullscreen(fullscreen)
                            }
                        }
                    }
                }
                div {
                    class: c_game_description()
                    "This demo uses euv-engine's raytracing module to drive a software ray tracer directly on the Canvas 2D backing buffer: every frame, for every pixel, the camera fires a primary Ray through the scene of three occluders (one mirror sphere, one emissive sphere, one ground AABB) using trace_default, which recursively reflects up to RAYTRACE_DEFAULT_MAX_BOUNCES. LightingUniforms::shade combines ambient, Lambertian diffuse, and Blinn-Phong specular per hit, gated by soft_shadow_factor. Renders at 160x100 (so a full per-pixel pass fits in a single requestAnimationFrame) and CSS-scales to the visible canvas."
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
                        onclick: raytrace_on_enter_fullscreen(fullscreen)
                    }
                }
            }
        }
    }
}
