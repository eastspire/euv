use super::*;

/// Represents the available rendering backend tabs on the 3D game page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Game3DTab {
    /// The Canvas 2D rendering backend tab.
    #[default]
    Canvas2D,
    /// The WebGL 2 rendering backend tab.
    WebGl,
    /// The WebGPU rendering backend tab.
    WebGpu,
    /// The software ray tracer tab.
    RayTrace,
}
