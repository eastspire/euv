use super::*;

/// Represents the available rendering backend tabs on the 2D game page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Game2DTab {
    /// The Canvas 2D rendering backend tab.
    #[default]
    Canvas2D,
    /// The WebGL 2 rendering backend tab.
    WebGl,
    /// The WebGPU rendering backend tab.
    WebGpu,
    /// The CPU Phong lighting demo tab.
    Lighting,
}
