/// The HTML `id` attribute value for the standalone Lighting demo canvas element.
pub(crate) const LIGHTING_CANVAS_ID: &str = "lighting-canvas";

/// The CSS selector used to query the Lighting demo canvas element from the DOM.
pub(crate) const LIGHTING_CANVAS_SELECTOR: &str = "#lighting-canvas";

/// The Canvas 2D context type identifier passed to `HTMLCanvasElement::get_context`.
pub(crate) const LIGHTING_CONTEXT_TYPE: &str = "2d";

/// Logical width of the Lighting page's offscreen render buffer.
///
/// The buffer is sized so a full per-pixel Phong pass finishes well
/// under 16ms per frame on a mid-range laptop. The CSS box scales the
/// buffer to fit the visible canvas via the `c_game_3d_canvas` style.
pub(crate) const LIGHTING_WIDTH: f64 = 320.0;

/// Logical height of the Lighting page's offscreen render buffer.
pub(crate) const LIGHTING_HEIGHT: f64 = 240.0;

/// Delay in milliseconds before the lighting loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a 2D context.
pub(crate) const LIGHTING_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name used to set the fill color on a Canvas 2D context.
pub(crate) const LIGHTING_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// Z position of the eye used as the view direction for the Phong
/// specular term in the Lighting demo.
///
/// The page renders onto a 2D canvas, so we synthesise a fixed
/// "out-of-screen" eye at this Z to keep the specular highlight stable
/// across frames.
pub(crate) const LIGHTING_EYE_Z: f64 = 2.0;
