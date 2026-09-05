/// The HTML `id` attribute value for the RayTrace demo canvas element.
pub(crate) const RAYTRACE_CANVAS_ID: &str = "raytrace-canvas";

/// The CSS selector used to query the RayTrace demo canvas element from the DOM.
pub(crate) const RAYTRACE_CANVAS_SELECTOR: &str = "#raytrace-canvas";

/// The Canvas 2D context type identifier passed to `HTMLCanvasElement::get_context`.
pub(crate) const RAYTRACE_CONTEXT_TYPE: &str = "2d";

/// Logical width of the RayTrace page's offscreen render buffer.
///
/// The buffer is intentionally low resolution (160x100) so a full
/// per-pixel software ray pass finishes well under 16ms per frame on a
/// mid-range laptop. The CSS box scales the buffer to fit the visible
/// canvas via the `c_game_3d_canvas` style.
pub(crate) const RAYTRACE_WIDTH: f64 = 160.0;

/// Logical height of the RayTrace page's offscreen render buffer.
pub(crate) const RAYTRACE_HEIGHT: f64 = 100.0;

/// Delay in milliseconds before the raytrace loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a 2D context.
pub(crate) const RAYTRACE_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name used to set the fill color on a Canvas 2D context.
pub(crate) const RAYTRACE_PROPERTY_FILL_STYLE: &str = "fillStyle";
