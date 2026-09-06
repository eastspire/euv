use super::*;

/// Creates the RayTrace page reactive state.
///
/// # Returns
///
/// - `UseRayTrace` - The RayTrace page state.
pub(crate) fn use_raytrace_state() -> UseRayTrace {
    UseRayTrace {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loop_started: App::use_signal(|| false),
        auto_rotate: App::use_signal(|| true),
    }
}

/// Creates the RayTrace page fullscreen overlay state.
///
/// # Returns
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
pub(crate) fn use_raytrace_fullscreen_state() -> UseRayTraceFullscreen {
    UseRayTraceFullscreen {
        fullscreen: App::use_signal(|| false),
    }
}

/// Returns `true` when no element matches the canvas selector, meaning the
/// page or tab was navigated away from and the game loop should stop.
///
/// Hook-context cleanups (`App::use_cleanup`) only run on match-arm
/// switches, not on router navigation, so RAF loops additionally guard on
/// canvas presence to avoid simulating and rendering against a detached
/// canvas forever.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element.
///
/// # Returns
///
/// - `bool` - Whether the canvas is absent from the document.
fn raytrace_canvas_detached(canvas_selector: &str) -> bool {
    window()
        .and_then(|window_value: Window| window_value.document())
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .is_none()
}

/// Acquires the 2D context for the RayTrace demo canvas, resizing the
/// backing buffer to the logical render dimensions if needed.
///
/// Returns `None` if the canvas element cannot be found (for example
/// while the page is mid-route transition) or if a 2D context cannot be
/// acquired.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, CanvasRenderingContext2d)>` - The canvas and its 2D context.
fn acquire_raytrace_canvas() -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(RAYTRACE_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    let width_u32: u32 = RAYTRACE_WIDTH as u32;
    let height_u32: u32 = RAYTRACE_HEIGHT as u32;
    if canvas.width() != width_u32 {
        canvas.set_width(width_u32);
    }
    if canvas.height() != height_u32 {
        canvas.set_height(height_u32);
    }
    let Some(context_object) = canvas.get_context(RAYTRACE_CONTEXT_TYPE).ok().flatten() else {
        return None;
    };
    let context: CanvasRenderingContext2d = context_object.unchecked_into();
    Some((canvas, context))
}

/// Builds the static raytracing scene used by the RayTrace demo.
///
/// Three occluders: a mirror sphere in the centre (Phong specular
/// material drives the reflection), an emissive sphere in the back
/// (acts as the only light source visible to bounced rays), and a
/// ground AABB below the spheres. Returns the occluder list together
/// with the eye position (kept constant so specular highlights stay
/// stable as the camera orbits).
///
/// # Returns
///
/// - `(Vec<Occluder>, Vector3D)` - The static scene occluders and the eye position.
fn build_raytrace_scene() -> (Vec<Occluder>, Vector3D) {
    let ground_min: Vector3D = Vector3D::new(-5.0, -0.6, -5.0);
    let ground_max: Vector3D = Vector3D::new(5.0, -0.5, 5.0);
    let ground_material: Material = Material::phong(Vector3D::new(0.30, 0.32, 0.36), 0.30, 24.0);
    let ground: Occluder = Occluder::aabb(ground_min, ground_max, ground_material);
    let mirror_material: Material = Material::phong(Vector3D::new(0.05, 0.05, 0.06), 1.0, 64.0);
    let mirror: Occluder = Occluder::sphere(Vector3D::new(0.0, 0.4, 0.0), 0.9, mirror_material);
    let emissive_material: Material = Material::emissive(Vector3D::new(1.0, 0.45, 0.10));
    let emissive: Occluder =
        Occluder::sphere(Vector3D::new(1.6, 0.6, -1.4), 0.45, emissive_material);
    let occluders: Vec<Occluder> = vec![ground, mirror, emissive];
    let eye: Vector3D = Vector3D::new(0.0, 0.8, 3.5);
    (occluders, eye)
}

/// Builds the per-frame lighting uniforms for the raytrace scene.
///
/// The single directional sun rotates with the current yaw so the lit
/// side of the spheres tracks the orbiting camera: as the user drags
/// the camera the highlight smoothly slides off the visible side of
/// the mirror. Ambient and the specular eye stay constant.
///
/// # Arguments
///
/// - `Vector3D` - The eye position used for specular calculations.
/// - `f64` - The current camera yaw in radians.
///
/// # Returns
///
/// - `LightingUniforms` - The per-frame lighting.
fn build_raytrace_lighting(eye: Vector3D, yaw: f64) -> LightingUniforms {
    let light_dir: Vector3D = Vector3D::new(-yaw.cos(), -0.5, -yaw.sin()).normalized();
    let sun: Light = Light::new_directional(light_dir, Vector3D::new(1.0, 0.95, 0.85));
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.10, 0.10, 0.14));
    lights.add_light(sun);
    lights
}

/// Clamps an `0..=infinity` linear color channel into the `0..=1`
/// range used by the sRGB gamma curve.
///
/// # Arguments
///
/// - `f64` - The linear color channel value.
///
/// # Returns
///
/// - `f64` - The clamped value in `[0, 1]`.
fn clamp_unit(value: f64) -> f64 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Writes a single pixel into the 2D context with an sRGB gamma
/// correction applied to the linear color.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `i32` - The pixel x coordinate.
/// - `i32` - The pixel y coordinate.
/// - `f64` - The linear red channel.
/// - `f64` - The linear green channel.
/// - `f64` - The linear blue channel.
fn write_pixel(context: &CanvasRenderingContext2d, x: i32, y: i32, r: f64, g: f64, b: f64) {
    let gamma: f64 = 1.0 / 2.2;
    let cr: f64 = clamp_unit(r).powf(gamma);
    let cg: f64 = clamp_unit(g).powf(gamma);
    let cb: f64 = clamp_unit(b).powf(gamma);
    let style: String = format!(
        "rgb({},{},{})",
        (cr * 255.0).round() as u8,
        (cg * 255.0).round() as u8,
        (cb * 255.0).round() as u8,
    );
    let key: JsValue = JsValue::from_str(RAYTRACE_PROPERTY_FILL_STYLE);
    let _: Result<bool, JsValue> = Reflect::set(context.as_ref(), &key, &JsValue::from_str(&style));
    context.fill_rect(x as f64, y as f64, 1.0, 1.0);
}

/// Computes the camera basis (forward, right, up_true) for a given
/// yaw / pitch orbit position.
///
/// Mirrors the spherical-to-Cartesian conversion used by the 3D game
/// page so the two demos produce visually equivalent camera paths. The
/// `look_at` target is fixed at the scene origin's mid-height; only the
/// eye position changes with yaw/pitch.
///
/// # Arguments
///
/// - `Vector3D` - The eye position.
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
///
/// # Returns
///
/// - `(Vector3D, Vector3D, Vector3D)` - The (forward, right, up_true) basis.
fn build_camera_basis(eye: Vector3D, yaw: f64, pitch: f64) -> (Vector3D, Vector3D, Vector3D) {
    let look_at: Vector3D = Vector3D::new(0.0, 0.4, 0.0);
    let up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let forward: Vector3D = (look_at - eye).normalized();
    let _ = yaw;
    let _ = pitch;
    let right: Vector3D = forward.cross(up).normalized();
    let up_true: Vector3D = right.cross(forward).normalized();
    (forward, right, up_true)
}

/// Computes the eye position for the given orbit yaw / pitch angles.
///
/// Mirrors the orbit-to-eye conversion in
/// `create_orbit_camera` from the 3D game page: the camera sits on a
/// sphere of radius `RAYTRACE_CAMERA_DISTANCE` centred on the scene's
/// look-at target, so dragging horizontally rotates around the scene
/// and dragging vertically tilts up / down.
///
/// # Arguments
///
/// - `f64` - The orbit yaw in radians.
/// - `f64` - The orbit pitch in radians.
///
/// # Returns
///
/// - `Vector3D` - The eye position in world space.
fn compute_eye_position(yaw: f64, pitch: f64) -> Vector3D {
    let cos_pitch: f64 = pitch.cos();
    let distance: f64 = RAYTRACE_CAMERA_DISTANCE;
    Vector3D::new(
        distance * yaw.sin() * cos_pitch,
        RAYTRACE_CAMERA_LOOK_AT_Y + distance * pitch.sin(),
        RAYTRACE_CAMERA_LOOK_AT_Z + distance * yaw.cos() * cos_pitch,
    )
}

/// Renders one full frame of the RayTrace demo into the supplied 2D
/// context.
///
/// Builds the camera basis once, then for every pixel in the backing
/// buffer computes a primary `Ray`, calls `trace_default` to walk the
/// scene and bounce reflections up to `RAYTRACE_DEFAULT_MAX_BOUNCES`,
/// and writes the resulting linear color into the pixel. Lighting is
/// rebuilt per frame from the current yaw so the directional sun tracks
/// the camera orbit.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The 2D context to render into.
/// - `&[Occluder]` - The scene occluders.
/// - `&LightingUniforms` - The per-frame lighting.
fn render_raytrace_frame(
    context: &CanvasRenderingContext2d,
    occluders: &[Occluder],
    lights: &LightingUniforms,
    yaw: f64,
    pitch: f64,
) {
    let width: f64 = RAYTRACE_WIDTH;
    let height: f64 = RAYTRACE_HEIGHT;
    let eye: Vector3D = compute_eye_position(yaw, pitch);
    let (forward, right, up_true): (Vector3D, Vector3D, Vector3D) =
        build_camera_basis(eye, yaw, pitch);
    let aspect: f64 = width / height;
    let focal: f64 = 1.0;
    context.clear_rect(0.0, 0.0, width, height);
    let width_i32: i32 = width as i32;
    let height_i32: i32 = height as i32;
    for y in 0..height_i32 {
        for x in 0..width_i32 {
            let ndc_x: f64 = ((x as f64 + 0.5) / width) * 2.0 - 1.0;
            let ndc_y: f64 = 1.0 - ((y as f64 + 0.5) / height) * 2.0;
            let dir: Vector3D =
                (forward.scaled(focal) + right.scaled(ndc_x * aspect) + up_true.scaled(ndc_y))
                    .normalized();
            let ray: Ray = Ray::new(eye, dir);
            let color: Vector3D = trace_default(ray, occluders, lights);
            write_pixel(context, x, y, color.get_x(), color.get_y(), color.get_z());
        }
    }
}

/// Starts the RayTrace page `requestAnimationFrame` loop.
///
/// Per frame: applies auto-rotate yaw if enabled, rebuilds lighting
/// from the current yaw, and re-traces every pixel. The FPS counter,
/// `use_cleanup` cancellation, and canvas-detached guard mirror the
/// game_2d / game_3d pattern.
///
/// # Arguments
///
/// - `UseRayTrace` - The RayTrace page state.
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_raytrace_loop(state: UseRayTrace, angles: RayTraceCameraAngles) {
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let (occluders, eye) = build_raytrace_scene();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let yaw_clone: Rc<Cell<f64>> = angles.yaw.clone();
    let pitch_clone: Rc<Cell<f64>> = angles.pitch.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if raytrace_canvas_detached(RAYTRACE_CANVAS_SELECTOR) {
            return;
        }
        let Some(window_value): Option<Window> = window() else {
            return;
        };
        let Some(performance): Option<Performance> = window_value.performance() else {
            return;
        };
        let current_time: f64 = performance.now() / 1000.0;
        let prev: f64 = last_clone.get();
        let frame_time: f64 = if prev < 0.0 {
            1.0 / 60.0
        } else {
            (current_time - prev).min(0.25)
        };
        last_clone.set(current_time);
        if state.get_auto_rotate().get() {
            yaw_clone.set(yaw_clone.get() + RAYTRACE_AUTO_YAW_SPEED * frame_time);
        }
        let yaw: f64 = yaw_clone.get();
        let pitch: f64 = pitch_clone.get();
        if state.get_running().get() {
            if let Some((_canvas, context)) = acquire_raytrace_canvas() {
                let lights: LightingUniforms = build_raytrace_lighting(eye, yaw);
                render_raytrace_frame(&context, &occluders, &lights, yaw, pitch);
            }
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
            state.get_fps().set(fps);
            frame_clone.set(0);
            fps_clone.set(0.0);
        }
        let Some(raf_closure_ref): Option<&'static Closure<dyn FnMut()>> = cell_clone.try_get()
        else {
            return;
        };
        let next_id: i32 = window_value
            .request_animation_frame(raf_closure_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_clone.set(Some(next_id));
    }));
    let _: Result<(), _> = closure_cell.try_set(raf_closure);
    let start_timeout_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let start_timeout_clone: Rc<Cell<Option<i32>>> = start_timeout_id.clone();
    let raf_for_start: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_start: RafClosureCell = closure_cell.clone();
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let Some(start_window): Option<Window> = window() else {
            return;
        };
        let Some(start_raf_ref): Option<&'static Closure<dyn FnMut()>> = cell_for_start.try_get()
        else {
            return;
        };
        let start_id: i32 = start_window
            .request_animation_frame(start_raf_ref.as_ref().unchecked_ref())
            .unwrap_or_default();
        raf_for_start.set(Some(start_id));
    }));
    let start_callback: Function = start_closure.as_ref().unchecked_ref::<Function>().clone();
    start_closure.forget();
    let Some(start_window): Option<Window> = window() else {
        return;
    };
    let timeout_id: i32 = start_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            &start_callback,
            RAYTRACE_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or_default();
    start_timeout_clone.set(Some(timeout_id));
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    App::use_cleanup(move || {
        if let Some(cancel_id) = raf_for_cleanup.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timeout_id) = start_timeout_id.get() {
            let Some(window_value): Option<Window> = window() else {
                return;
            };
            window_value.clear_timeout_with_handle(timeout_id);
        }
        let _: Option<_> = cell_for_cleanup.try_take();
    });
    state.get_loop_started().set(true);
}

/// Creates a click handler that toggles the RayTrace loop between
/// running and paused.
///
/// # Arguments
///
/// - `UseRayTrace` - The RayTrace page state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_toggle_pause(state: UseRayTrace) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Creates a click handler that toggles camera auto-rotation.
///
/// # Arguments
///
/// - `UseRayTrace` - The RayTrace page state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_toggle_auto_rotate(state: UseRayTrace) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_auto_rotate().get();
        state.get_auto_rotate().set(!current);
    }))
}

/// Creates a click handler that resets the camera orbit angles.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn raytrace_on_reset_camera(angles: RayTraceCameraAngles) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        angles.yaw.set(0.6);
        angles.pitch.set(0.25);
    }))
}

/// Reads the client X coordinate off a `MouseEvent`-like object via reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client X coordinate, or `0.0` if missing.
fn client_x_from_event(event: &Event) -> f64 {
    Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default()
}

/// Reads the client Y coordinate off a `MouseEvent`-like object via reflection.
///
/// # Arguments
///
/// - `&Event` - The pointer event.
///
/// # Returns
///
/// - `f64` - The client Y coordinate, or `0.0` if missing.
fn client_y_from_event(event: &Event) -> f64 {
    Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default()
}

/// Creates a pointer move handler that updates the camera orbit angles
/// from drag movement and disables auto-rotate for the rest of the
/// session.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
/// - `UseRayTrace` - The RayTrace page state.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer move handler.
pub(crate) fn raytrace_on_pointer_move(
    angles: RayTraceCameraAngles,
    state: UseRayTrace,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let client_x: f64 = client_x_from_event(&event);
        let client_y: f64 = client_y_from_event(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * RAYTRACE_DRAG_SENSITIVITY;
        let pitch: f64 = (angles.pitch.get() + dy * RAYTRACE_DRAG_SENSITIVITY).clamp(
            -HALF_PI + RAYTRACE_PITCH_CLAMP,
            HALF_PI - RAYTRACE_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
        state.get_auto_rotate().set(false);
    }))
}

/// Creates a pointer down handler that records the drag start position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer down handler.
pub(crate) fn raytrace_on_pointer_down(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let client_x: f64 = client_x_from_event(&event);
        let client_y: f64 = client_y_from_event(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a pointer up / leave handler that clears the drag state.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer up handler.
pub(crate) fn raytrace_on_pointer_up(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        last_pointer.set(None);
    }))
}

/// Extracts the first touch's client coordinates from a `TouchEvent`.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` of the first touch.
fn first_touch_client(event: &Event) -> (f64, f64) {
    let touches_value: JsValue = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_TOUCHES),
    )
    .ok()
    .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&touch, &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_X))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    let client_y: f64 = Reflect::get(&touch, &JsValue::from_str(RAYTRACE_EVENT_PROPERTY_CLIENT_Y))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or_default();
    (client_x, client_y)
}

/// Creates a touch start handler that records the first touch position
/// and prevents default to avoid page scrolling during camera drag.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn raytrace_on_touch_start(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let (client_x, client_y): (f64, f64) = first_touch_client(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a touch move handler that updates orbit angles from the
/// single-finger drag and disables auto-rotate.
///
/// # Arguments
///
/// - `RayTraceCameraAngles` - The non-reactive camera orbit angles.
/// - `UseRayTrace` - The RayTrace page state.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch move handler.
pub(crate) fn raytrace_on_touch_move(
    angles: RayTraceCameraAngles,
    state: UseRayTrace,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let (client_x, client_y): (f64, f64) = first_touch_client(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * RAYTRACE_DRAG_SENSITIVITY;
        let pitch: f64 = (angles.pitch.get() + dy * RAYTRACE_DRAG_SENSITIVITY).clamp(
            -HALF_PI + RAYTRACE_PITCH_CLAMP,
            HALF_PI - RAYTRACE_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
        state.get_auto_rotate().set(false);
    }))
}

/// Creates a touch end handler that clears the drag state.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch end handler.
pub(crate) fn raytrace_on_touch_end(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        last_pointer.set(None);
    }))
}

/// Enters landscape fullscreen mode for the RayTrace page.
///
/// Sets the fullscreen signal, pushes a history entry so the system
/// back button can exit, and re-applies safe-area insets to the
/// newly-mounted overlay container.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
pub(crate) fn enter_raytrace_fullscreen(state: UseRayTraceFullscreen) {
    state.get_fullscreen().set(true);
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
}

/// Exits landscape fullscreen mode for the RayTrace page.
///
/// Used by the in-overlay Exit button. Clears the fullscreen signal
/// and re-applies the safe-area insets. The `history.back()` call
/// inside `Router::overlay_back` consumes the browser history entry
/// that was pushed on enter.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
pub(crate) fn exit_raytrace_fullscreen(state: UseRayTraceFullscreen) {
    state.get_fullscreen().set(false);
    UseEuvLayout::apply_cached_insets();
}

/// Exits landscape fullscreen mode without consuming a browser history
/// entry.
///
/// Used when the exit is triggered by the system back button: the
/// `popstate` event itself has already consumed the `pushState` entry
/// that was created when entering fullscreen, so calling
/// `history.back()` again would over-consume the history stack.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
pub(crate) fn exit_raytrace_fullscreen_from_popstate(state: UseRayTraceFullscreen) {
    state.get_fullscreen().set(false);
    UseEuvLayout::apply_cached_insets();
}

/// Subscribes to browser `popstate` events to handle the system back
/// button while the RayTrace page is in landscape fullscreen mode.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_raytrace_fullscreen_popstate(state: UseRayTraceFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_fullscreen().get() {
            exit_raytrace_fullscreen_from_popstate(state);
            true
        } else {
            false
        }
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode for the RayTrace page.
///
/// Delegates to [`enter_raytrace_fullscreen`], which sets the
/// fullscreen signal, pushes a history entry, and reapplies
/// safe-area insets to the newly-mounted overlay container. The
/// canvas itself is not recreated — the running raytrace loop, FPS
/// counter, and pause state all survive the transition.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn raytrace_on_enter_fullscreen(
    state: UseRayTraceFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_raytrace_fullscreen(state);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode for the RayTrace page.
///
/// Delegates to [`exit_raytrace_fullscreen`], which clears the
/// fullscreen signal and reapplies safe-area insets. The
/// `history.back()` call inside `Router::overlay_back` consumes the
/// browser history entry that was pushed on enter.
///
/// # Arguments
///
/// - `UseRayTraceFullscreen` - The RayTrace page fullscreen state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn raytrace_on_exit_fullscreen(
    state: UseRayTraceFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_raytrace_fullscreen(state);
        Router::overlay_back(None);
    }))
}
