use super::*;

/// Reactive state for the standalone RayTrace page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTrace {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the raytrace loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the raytrace loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
    /// Whether the camera auto-rotates around the scene each frame.
    ///
    /// Dragging on the canvas disables auto-rotate for the rest of the
    /// session; the toolbar button re-enables it.
    #[get(type(copy))]
    pub(crate) auto_rotate: Signal<bool>,
}

/// Reactive state for the RayTrace page fullscreen overlay.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceFullscreen {
    /// Whether the RayTrace page is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
}

/// Non-reactive camera orbit angles persisted via a `Signal` wrapper.
///
/// The `Signal` is read once to obtain the `Rc` handles; all subsequent
/// reads and writes go through `Cell` which bypasses the reactivity
/// system entirely, preventing re-render storms during rapid mouse
/// drag. `PartialEq` is derived so the type satisfies `Signal<T>`'s
/// `T: PartialEq` bound (the bound only matters for re-render skipping,
/// never for value equality, since the cell values change every frame).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RayTraceCameraAngles {
    /// The orbit yaw angle in radians.
    pub(crate) yaw: Rc<Cell<f64>>,
    /// The orbit pitch angle in radians.
    pub(crate) pitch: Rc<Cell<f64>>,
}

impl RayTraceCameraAngles {
    /// Creates a default `RayTraceCameraAngles` with sensible starting
    /// values: a slight downward look (pitch 0.25) so the ground AABB
    /// is visible in the first frame.
    ///
    /// # Returns
    ///
    /// - `RayTraceCameraAngles` - The new camera angles.
    pub(crate) fn default() -> RayTraceCameraAngles {
        RayTraceCameraAngles {
            yaw: Rc::new(Cell::new(0.6)),
            pitch: Rc::new(Cell::new(0.25)),
        }
    }
}
