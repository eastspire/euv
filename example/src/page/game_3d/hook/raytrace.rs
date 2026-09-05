use super::*;

/// Reactive state for the 3D RayTrace demo.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame3DRayTrace {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the raytrace loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the raytrace loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
}

/// Creates the 3D RayTrace demo reactive state.
pub(crate) fn use_game_3d_raytrace_state() -> UseGame3DRayTrace {
    UseGame3DRayTrace {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loop_started: App::use_signal(|| false),
    }
}

/// Acquires the 2D context for the RayTrace demo canvas, resizing the
/// backing buffer to the logical render dimensions if needed.
///
/// Returns `None` if the canvas element cannot be found (for example
/// while the page is mid-route transition) or if a 2D context cannot be
/// acquired.
fn acquire_raytrace_canvas() -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(GAME_3D_RAYTRACE_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    let width_u32: u32 = GAME_3D_RAYTRACE_WIDTH as u32;
    let height_u32: u32 = GAME_3D_RAYTRACE_HEIGHT as u32;
    if canvas.width() != width_u32 {
        canvas.set_width(width_u32);
    }
    if canvas.height() != height_u32 {
        canvas.set_height(height_u32);
    }
    let Some(context_object) = canvas
        .get_context(GAME_3D_RAYTRACE_CONTEXT_TYPE)
        .ok()
        .flatten()
    else {
        return None;
    };
    let context: CanvasRenderingContext2d = context_object.unchecked_into();
    Some((canvas, context))
}

/// Builds the static raytracing scene used by the 3D RayTrace demo.
///
/// Three occluders: a mirror sphere in the centre (Phong specular
/// material drives the reflection), an emissive sphere in the back
/// (acts as the only light source visible to bounced rays), and a
/// ground AABB below the spheres. The two lights (one ambient, one
/// soft point light positioned above and behind the camera) feed
/// `LightingUniforms::shade` for the diffuse contribution of the
/// first hit.
fn build_raytrace_scene() -> (Vec<Occluder>, LightingUniforms) {
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
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.10, 0.10, 0.14));
    let lamp: Light = Light::new_point(
        Vector3D::new(-2.0, 2.0, 2.0),
        Vector3D::new(0.95, 0.95, 0.85),
        1.4,
    );
    lights.add_light(lamp);
    (occluders, lights)
}

/// Clamps an `0..=infinity` linear color channel into the `0..=1`
/// range used by the sRGB gamma curve.
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
    let key: JsValue = JsValue::from_str(GAME_3D_PROPERTY_FILL_STYLE);
    let _: Result<bool, JsValue> = Reflect::set(context.as_ref(), &key, &JsValue::from_str(&style));
    context.fill_rect(x as f64, y as f64, 1.0, 1.0);
}

/// Renders one full frame of the RayTrace demo into the supplied 2D
/// context.
///
/// Builds the camera basis once, then for every pixel in the backing
/// buffer computes a primary `Ray`, calls `trace_default` to walk the
/// scene and bounce reflections up to `RAYTRACE_DEFAULT_MAX_BOUNCES`,
/// and writes the resulting linear color into the pixel.
fn render_raytrace_frame(
    context: &CanvasRenderingContext2d,
    occluders: &[Occluder],
    lights: &LightingUniforms,
) {
    let width: f64 = GAME_3D_RAYTRACE_WIDTH;
    let height: f64 = GAME_3D_RAYTRACE_HEIGHT;
    let eye: Vector3D = lights.get_eye();
    let look_at: Vector3D = Vector3D::new(0.0, 0.4, 0.0);
    let up: Vector3D = Vector3D::new(0.0, 1.0, 0.0);
    let forward: Vector3D = (look_at - eye).normalized();
    let right: Vector3D = forward.cross(up).normalized();
    let up_true: Vector3D = right.cross(forward).normalized();
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

/// Starts the 3D RayTrace demo `requestAnimationFrame` loop.
///
/// Mirrors `start_game_2d_lighting_loop` (FPS counter, `use_cleanup`
/// cancellation, canvas-detached guard) but the scene is fully static
/// and re-traced every frame; there is no integration step.
pub(crate) fn start_game_3d_raytrace_loop(state: UseGame3DRayTrace) {
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let (occluders, lights) = build_raytrace_scene();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if game_3d_canvas_detached(GAME_3D_RAYTRACE_CANVAS_SELECTOR) {
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
        if state.get_running().get() {
            if let Some((_canvas, context)) = acquire_raytrace_canvas() {
                render_raytrace_frame(&context, &occluders, &lights);
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
            GAME_3D_LOOP_START_DELAY_MILLIS,
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

/// Creates a click handler that toggles the 3D RayTrace loop between
/// running and paused.
///
/// # Arguments
///
/// - `UseGame3DRayTrace` - The 3D RayTrace state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn game_3d_raytrace_on_toggle_pause(
    state: UseGame3DRayTrace,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}
