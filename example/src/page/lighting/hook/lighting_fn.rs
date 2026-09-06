use super::*;

/// Creates the standalone Lighting page reactive state.
///
/// # Returns
///
/// - `UseLighting` - The Lighting page state.
pub(crate) fn use_lighting_state() -> UseLighting {
    UseLighting {
        fps: App::use_signal(|| 0.0),
        running: App::use_signal(|| true),
        loop_started: App::use_signal(|| false),
    }
}

/// Creates the Lighting page fullscreen overlay state.
///
/// # Returns
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
pub(crate) fn use_lighting_fullscreen_state() -> UseLightingFullscreen {
    UseLightingFullscreen {
        fullscreen: App::use_signal(|| false),
    }
}

/// Returns `true` when no element matches the canvas selector, meaning the
/// page or tab was navigated away from and the lighting loop should stop.
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
fn lighting_canvas_detached(canvas_selector: &str) -> bool {
    window()
        .and_then(|window_value: Window| window_value.document())
        .and_then(|document: Document| document.query_selector(canvas_selector).ok().flatten())
        .is_none()
}

/// Acquires the 2D context for the Lighting demo canvas, resizing the
/// backing buffer to the logical render dimensions if needed.
///
/// Returns `None` if the canvas element cannot be found (for example
/// while the page is mid-route transition) or if a 2D context cannot be
/// acquired.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, CanvasRenderingContext2d)>` - The canvas and its 2D context.
fn acquire_lighting_canvas() -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let window_value: Window = window()?;
    let document_value: Document = window_value.document()?;
    let element: Element = document_value
        .query_selector(LIGHTING_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    let width_u32: u32 = LIGHTING_WIDTH as u32;
    let height_u32: u32 = LIGHTING_HEIGHT as u32;
    if canvas.width() != width_u32 {
        canvas.set_width(width_u32);
    }
    if canvas.height() != height_u32 {
        canvas.set_height(height_u32);
    }
    let Some(context_object) = canvas.get_context(LIGHTING_CONTEXT_TYPE).ok().flatten() else {
        return None;
    };
    let context: CanvasRenderingContext2d = context_object.unchecked_into();
    Some((canvas, context))
}

/// Builds the static lighting scene used by the standalone Lighting demo.
///
/// Five spheres at varied positions and sizes plus a horizontal ground
/// line at the bottom of the canvas. Returns the sphere list and the
/// `LightingUniforms` (one directional sun + one point lamp + ambient +
/// eye) consumed by `LightingUniforms::shade`.
///
/// # Returns
///
/// - `(Vec<LightingSphere>, LightingUniforms)` - The static scene spheres and lighting.
fn build_lighting_scene() -> (Vec<LightingSphere>, LightingUniforms) {
    let width: f64 = LIGHTING_WIDTH;
    let height: f64 = LIGHTING_HEIGHT;
    let ground_y: f64 = height * 0.78;
    let red_material: Material = Material::phong(Vector3D::new(0.85, 0.20, 0.20), 0.5, 24.0);
    let green_material: Material = Material::phong(Vector3D::new(0.20, 0.80, 0.30), 0.6, 32.0);
    let blue_material: Material = Material::phong(Vector3D::new(0.25, 0.45, 0.95), 0.4, 18.0);
    let yellow_material: Material = Material::phong(Vector3D::new(0.95, 0.85, 0.20), 0.7, 48.0);
    let magenta_material: Material = Material::lambert(Vector3D::new(0.85, 0.25, 0.75));
    let spheres: Vec<LightingSphere> = vec![
        LightingSphere {
            cx: width * 0.22,
            cy: height * 0.42,
            radius: 24.0,
            material: red_material.clone(),
        },
        LightingSphere {
            cx: width * 0.42,
            cy: height * 0.55,
            radius: 18.0,
            material: green_material.clone(),
        },
        LightingSphere {
            cx: width * 0.62,
            cy: height * 0.40,
            radius: 22.0,
            material: blue_material.clone(),
        },
        LightingSphere {
            cx: width * 0.78,
            cy: height * 0.62,
            radius: 16.0,
            material: yellow_material.clone(),
        },
        LightingSphere {
            cx: width * 0.50,
            cy: height * 0.20,
            radius: 12.0,
            material: magenta_material,
        },
    ];
    let eye: Vector3D = Vector3D::new(0.0, 0.0, LIGHTING_EYE_Z);
    let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
    lights.set_ambient(Vector3D::new(0.08, 0.08, 0.10));
    let sun: Light = Light::new_directional(
        Vector3D::new(-0.45, -0.55, -0.70),
        Vector3D::new(1.00, 0.95, 0.85),
    );
    let lamp: Light = Light::new_point(
        Vector3D::new(width * 0.5, -10.0, 1.2),
        Vector3D::new(0.40, 0.70, 1.00),
        1.4,
    );
    lights.add_light(sun);
    lights.add_light(lamp);
    let _: f64 = ground_y;
    (spheres, lights)
}

/// Packs an `0..=255` red channel byte from a linear 0..=1 float.
///
/// # Arguments
///
/// - `f64` - The linear color channel value.
///
/// # Returns
///
/// - `u8` - The packed 8-bit channel value.
fn clamp_byte(value: f64) -> u8 {
    let clamped: f64 = if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    };
    (clamped * 255.0).round() as u8
}

/// Sets the canvas 2D context `fillStyle` to the CSS rgb string of
/// `(r, g, b)` after applying a soft `^(1/2.2)` gamma curve.
///
/// The lighting math runs in linear space; this gamma correction keeps
/// the visual result from looking washed-out on a standard sRGB
/// display.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `f64` - The linear red channel.
/// - `f64` - The linear green channel.
/// - `f64` - The linear blue channel.
fn apply_pixel_style(context: &CanvasRenderingContext2d, r: f64, g: f64, b: f64) {
    let gamma: f64 = 1.0 / 2.2;
    let cr: f64 = r.max(0.0).powf(gamma);
    let cg: f64 = g.max(0.0).powf(gamma);
    let cb: f64 = b.max(0.0).powf(gamma);
    let style: String = format!(
        "rgb({},{},{})",
        clamp_byte(cr),
        clamp_byte(cg),
        clamp_byte(cb),
    );
    let key: JsValue = JsValue::from_str(LIGHTING_PROPERTY_FILL_STYLE);
    let _: Result<bool, JsValue> = Reflect::set(context.as_ref(), &key, &JsValue::from_str(&style));
}

/// Renders a single sphere into the 2D context by computing per-pixel
/// Phong shading.
///
/// For every pixel inside the sphere's bounding box, the surface
/// normal is reconstructed by treating the sphere as a unit sphere
/// whose "north pole" points out of the screen: `n = (dx, dy, dz) / r`
/// where `dz = sqrt(r² - dx² - dy²)`. The reconstructed normal feeds
/// `LightingUniforms::shade`, which sums ambient + Lambert + Phong for
/// every light in the scene.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `&LightingSphere` - The sphere to render.
/// - `&LightingUniforms` - The scene lighting.
fn render_lighting_sphere(
    context: &CanvasRenderingContext2d,
    sphere: &LightingSphere,
    lights: &LightingUniforms,
) {
    let r: f64 = sphere.radius;
    let r2: f64 = r * r;
    let x_start: i32 = (sphere.cx - r).floor() as i32;
    let x_end: i32 = (sphere.cx + r).ceil() as i32;
    let y_start: i32 = (sphere.cy - r).floor() as i32;
    let y_end: i32 = (sphere.cy + r).ceil() as i32;
    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let dx: f64 = x as f64 - sphere.cx;
            let dy: f64 = y as f64 - sphere.cy;
            let d2: f64 = dx * dx + dy * dy;
            if d2 > r2 {
                continue;
            }
            let dz: f64 = (r2 - d2).max(0.0).sqrt();
            let nx: f64 = dx / r;
            let ny: f64 = dy / r;
            let nz: f64 = dz / r;
            let normal: Vector3D = Vector3D::new(nx, ny, nz);
            let position: Vector3D = Vector3D::new(sphere.cx, sphere.cy, dz / r);
            let occluders: Vec<(Vector3D, f64)> = Vec::new();
            let color: Vector3D = lights.shade(position, normal, &sphere.material, &occluders);
            apply_pixel_style(context, color.get_x(), color.get_y(), color.get_z());
            context.fill_rect(x as f64, y as f64, 1.0, 1.0);
        }
    }
}

/// Renders the ground line at the bottom of the canvas with the same
/// lighting pipeline: the surface normal points straight up (0, -1, 0)
/// in our canvas coordinate system so the directional sun light still
/// hits it from the side.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `&LightingUniforms` - The scene lighting.
fn render_lighting_ground(context: &CanvasRenderingContext2d, lights: &LightingUniforms) {
    let width: f64 = LIGHTING_WIDTH;
    let height: f64 = LIGHTING_HEIGHT;
    let ground_y: f64 = (height * 0.78) as i32 as f64;
    let ground_material: Material = Material::phong(Vector3D::new(0.55, 0.55, 0.60), 0.15, 12.0);
    let normal: Vector3D = Vector3D::new(0.0, -1.0, 0.0);
    for x in 0..(width as i32) {
        let position: Vector3D = Vector3D::new(x as f64, ground_y, 0.0);
        let occluders: Vec<(Vector3D, f64)> = Vec::new();
        let color: Vector3D = lights.shade(position, normal, &ground_material, &occluders);
        apply_pixel_style(context, color.get_x(), color.get_y(), color.get_z());
        context.fill_rect(x as f64, ground_y, 1.0, 1.0);
    }
}

/// Renders one full frame of the Lighting demo into the supplied 2D
/// context.
///
/// Clears the canvas, then draws the ground line and every sphere in
/// scene order. Spheres later in the list are drawn on top of earlier
/// ones; there is no depth sorting because the fixed layout does not
/// produce overlapping silhouettes.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The target 2D context.
/// - `&[LightingSphere]` - The scene spheres.
/// - `&LightingUniforms` - The scene lighting.
fn render_lighting_frame(
    context: &CanvasRenderingContext2d,
    spheres: &[LightingSphere],
    lights: &LightingUniforms,
) {
    context.clear_rect(0.0, 0.0, LIGHTING_WIDTH, LIGHTING_HEIGHT);
    render_lighting_ground(context, lights);
    for sphere in spheres.iter() {
        render_lighting_sphere(context, sphere, lights);
    }
}

/// Starts the standalone Lighting page `requestAnimationFrame` loop.
///
/// The scene is fully static and re-shaded every frame; there is no
/// integration step. The FPS counter, `use_cleanup` cancellation, and
/// canvas-detached guard mirror the raytrace pattern.
///
/// # Arguments
///
/// - `UseLighting` - The Lighting page state.
pub(crate) fn start_lighting_loop(state: UseLighting) {
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let (spheres, lights) = build_lighting_scene();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        if lighting_canvas_detached(LIGHTING_CANVAS_SELECTOR) {
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
            if let Some((_canvas, context)) = acquire_lighting_canvas() {
                render_lighting_frame(&context, &spheres, &lights);
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
            LIGHTING_LOOP_START_DELAY_MILLIS,
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

/// Creates a click handler that toggles the standalone Lighting loop
/// between running and paused.
///
/// # Arguments
///
/// - `UseLighting` - The Lighting page state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The toggle handler.
pub(crate) fn lighting_on_toggle_pause(state: UseLighting) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Enters landscape fullscreen mode for the standalone Lighting page.
///
/// Sets the fullscreen signal, pushes a history entry so the system
/// back button can exit, and re-applies safe-area insets to the
/// newly-mounted overlay container.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
pub(crate) fn enter_lighting_fullscreen(state: UseLightingFullscreen) {
    state.get_fullscreen().set(true);
    Router::overlay_push_state();
    UseEuvLayout::apply_cached_insets();
}

/// Exits landscape fullscreen mode for the standalone Lighting page.
///
/// Used by the in-overlay Exit button. Clears the fullscreen signal
/// and re-applies the safe-area insets. The `history.back()` call
/// inside `Router::overlay_back` consumes the browser history entry
/// that was pushed on enter.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
pub(crate) fn exit_lighting_fullscreen(state: UseLightingFullscreen) {
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
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
pub(crate) fn exit_lighting_fullscreen_from_popstate(state: UseLightingFullscreen) {
    state.get_fullscreen().set(false);
    UseEuvLayout::apply_cached_insets();
}

/// Subscribes to browser `popstate` events to handle the system back
/// button while the Lighting page is in landscape fullscreen mode.
///
/// Returns the guard ID so the page can unregister it on unmount.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
///
/// # Returns
///
/// - `usize` - The popstate guard ID.
pub(crate) fn use_lighting_fullscreen_popstate(state: UseLightingFullscreen) -> usize {
    Router::register_popstate_guard(Rc::new(move || {
        if state.get_fullscreen().get() {
            exit_lighting_fullscreen_from_popstate(state);
            true
        } else {
            false
        }
    }))
}

/// Creates a click event handler that enters landscape fullscreen mode
/// for the standalone Lighting page.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn lighting_on_enter_fullscreen(
    state: UseLightingFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        enter_lighting_fullscreen(state);
    }))
}

/// Creates a click event handler that exits landscape fullscreen mode
/// for the standalone Lighting page.
///
/// # Arguments
///
/// - `UseLightingFullscreen` - The Lighting page fullscreen state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn lighting_on_exit_fullscreen(
    state: UseLightingFullscreen,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        exit_lighting_fullscreen(state);
        Router::overlay_back(None);
    }))
}
