use super::*;

/// A standalone interactive software ray-tracing demo page.
///
/// Renders a 320x240 scene of one mirror sphere, one emissive sphere,
/// and a ground AABB into the Canvas 2D backing buffer. Each frame
/// re-traces every pixel using `raytracing::trace_default` so the
/// mirror reflection bounces off the ground plane (or emissive
/// sphere) and lands somewhere visibly different from the primary
/// hit. The camera is an orbit camera whose yaw / pitch are driven
/// by mouse drag, touch drag, or auto-rotation; the directional sun
/// rotates with the yaw so the lit side of the spheres tracks the
/// orbiting camera. Mirrors the existing 2D / GL / GPU tab shape:
/// stats bar, canvas, pause/resume, reset-camera, auto-rotate
/// toggle, and a dedicated Enter Fullscreen button.
///
/// # Returns
/// - `VirtualNode` - The raytrace page virtual DOM tree.
#[component]
pub(crate) fn page_raytrace(node: VirtualNode<PageRaytraceProps>) -> VirtualNode {
    let _page_raytrace_props: PageRaytraceProps = node.try_get_props().unwrap_or_default();
    let state: UseRayTrace = use_raytrace_state();
    let fullscreen: UseRayTraceFullscreen = use_raytrace_fullscreen_state();
    use_raytrace_fullscreen_popstate(fullscreen);
    let fullscreen_signal: Signal<bool> = fullscreen.get_fullscreen();
    let angles_store: Signal<RayTraceCameraAngles> = App::use_signal(RayTraceCameraAngles::default);
    let angles: RayTraceCameraAngles = angles_store.get();
    let last_pointer: Signal<Rc<Cell<Option<(f64, f64)>>>> =
        App::use_signal(|| Rc::new(Cell::new(None)));
    let loop_started: Signal<bool> = state.get_loop_started();
    if !loop_started.get() {
        loop_started.set(true);
        start_raytrace_loop(state, angles.clone());
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_pause(state);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> = raytrace_on_toggle_auto_rotate(state);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = raytrace_on_reset_camera(angles.clone());
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_down(pointer_cell.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_pointer_move(angles.clone(), state, pointer_cell.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = raytrace_on_pointer_up(pointer_cell.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_start(pointer_cell.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        raytrace_on_touch_move(angles.clone(), state, pointer_cell.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = raytrace_on_touch_end(pointer_cell.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let pause_label: &str = if state.get_running().get() {
        "Pause"
    } else {
        "Resume"
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        "Auto: On"
    } else {
        "Auto: Off"
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🔦"
                title: "Software Ray Tracing"
                subtitle: "A CPU-side software ray tracer rendering 1 mirror sphere, 1 emissive sphere, and 1 ground AABB into a 320x240 backing buffer at ~15 FPS. Each frame re-traces every pixel via raytracing::trace_default, bouncing the mirror reflection off the ground or the emissive sphere. Drag the canvas to orbit the camera; the directional sun rotates with the yaw so the lit side of the spheres tracks the orbiting camera. Click Enter Fullscreen for a larger view."
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
                                    c_raytrace_canvas_fullscreen()
                                } else {
                                    c_game_3d_canvas()
                                }
                                onmousedown: on_pointer_down.clone()
                                onmousemove: on_pointer_move.clone()
                                onmouseup: on_pointer_up.clone()
                                onmouseleave: on_pointer_up.clone()
                                ontouchstart: on_touch_start.clone()
                                ontouchmove: on_touch_move.clone()
                                ontouchend: on_touch_end.clone()
                                ontouchcancel: on_touch_end.clone()
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
                    "This demo uses euv-engine's raytracing module to drive a software ray tracer directly on the Canvas 2D backing buffer: every frame, for every pixel, the camera fires a primary Ray through the scene of three occluders (one mirror sphere, one emissive sphere, one ground AABB) using trace_default, which recursively reflects up to RAYTRACE_DEFAULT_MAX_BOUNCES. LightingUniforms::shade combines ambient, Lambertian diffuse, and Blinn-Phong specular per hit, gated by soft_shadow_factor. The camera is an orbit camera driven by yaw/pitch — drag the canvas to look around, auto-rotate for a hands-free tour, and the directional sun rotates with the yaw so the lit side of the spheres tracks the orbiting camera."
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
                        label: auto_rotate_label
                        onclick: on_toggle_auto_rotate
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset Camera"
                        onclick: on_reset_camera
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
