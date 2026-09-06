use super::*;

/// A single shaded sphere rendered by the standalone Lighting demo.
///
/// Each sphere occupies a `(cx, cy)` centre in canvas pixel space and a
/// `radius` in pixels. The albedo and specular colour come straight from
/// `Material` so the same `lighting::compute_lambert` /
/// `lighting::compute_phong` routines used by the 3D engine can drive
/// the per-pixel fill pass.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LightingSphere {
    /// The horizontal centre of the sphere in canvas pixels.
    pub(crate) cx: f64,
    /// The vertical centre of the sphere in canvas pixels.
    pub(crate) cy: f64,
    /// The sphere radius in canvas pixels.
    pub(crate) radius: f64,
    /// The surface material applied to every shaded pixel.
    pub(crate) material: Material,
}

/// Reactive state for the standalone Lighting page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLighting {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the lighting loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// Whether the lighting loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
}

/// Reactive state for the standalone Lighting page fullscreen overlay.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseLightingFullscreen {
    /// Whether the Lighting page is currently in landscape fullscreen.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
}
