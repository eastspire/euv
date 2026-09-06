/// The HTML `id` attribute value for the RayTrace demo canvas element.
pub(crate) const RAYTRACE_CANVAS_ID: &str = "raytrace-canvas";

/// The CSS selector used to query the RayTrace demo canvas element from the DOM.
pub(crate) const RAYTRACE_CANVAS_SELECTOR: &str = "#raytrace-canvas";

/// The Canvas 2D context type identifier passed to `HTMLCanvasElement::get_context`.
pub(crate) const RAYTRACE_CONTEXT_TYPE: &str = "2d";

/// Logical width of the RayTrace page's offscreen render buffer.
///
/// The buffer is sized so a full per-pixel software ray pass finishes
/// well under 16ms per frame on a mid-range laptop. The CSS box scales
/// the buffer to fit the visible canvas via the `c_game_3d_canvas`
/// style.
pub(crate) const RAYTRACE_WIDTH: f64 = 320.0;

/// Logical height of the RayTrace page's offscreen render buffer.
pub(crate) const RAYTRACE_HEIGHT: f64 = 240.0;

/// The orbit yaw speed in radians per second for auto-rotation.
///
/// Mirrors the same constant in the 3D game page so the two demos feel
/// visually consistent when both are visible in the sidebar.
pub(crate) const RAYTRACE_AUTO_YAW_SPEED: f64 = 0.5;

/// The minimum angle in radians between the camera pitch and +/- pi/2.
///
/// Prevents the orbit camera from looking straight up or down, which
/// would collapse the `forward x up` cross product and zero the view
/// matrix.
pub(crate) const RAYTRACE_PITCH_CLAMP: f64 = 0.01;

/// The sensitivity multiplier applied to pointer drag deltas before
/// they are folded into orbit angles.
///
/// Matches the value used by the 3D game page's pointer handlers so the
/// two demos feel identical in drag responsiveness.
pub(crate) const RAYTRACE_DRAG_SENSITIVITY: f64 = 0.01;

/// The radius of the orbit sphere on which the camera sits.
///
/// Mirrors `GAME_3D_CAMERA_DISTANCE` so the user can compare the two
/// orbit-camera demos at equivalent zoom levels.
pub(crate) const RAYTRACE_CAMERA_DISTANCE: f64 = 8.0;

/// The y-coordinate of the orbit sphere's centre (the scene's
/// look-at target vertical position).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Y: f64 = 0.4;

/// The z-coordinate of the orbit sphere's centre (the scene's
/// look-at target depth).
pub(crate) const RAYTRACE_CAMERA_LOOK_AT_Z: f64 = 0.0;

/// The JavaScript property name for the touch list `touches` on a
/// `TouchEvent`.
pub(crate) const RAYTRACE_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a
/// `Touch` object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a
/// `Touch` object.
pub(crate) const RAYTRACE_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// Delay in milliseconds before the raytrace loop's first `requestAnimationFrame`
/// callback is scheduled, allowing the canvas element to mount before the
/// first frame attempts to acquire a 2D context.
pub(crate) const RAYTRACE_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name used to set the fill color on a Canvas 2D context.
pub(crate) const RAYTRACE_PROPERTY_FILL_STYLE: &str = "fillStyle";
