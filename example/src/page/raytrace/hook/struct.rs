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
}

/// Reactive state for the RayTrace page fullscreen overlay.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseRayTraceFullscreen {
    /// Whether the RayTrace page is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
}
