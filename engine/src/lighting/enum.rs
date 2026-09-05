use super::*;

/// Describes the kind of light source being represented.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LightType {
    /// An infinitely distant directional light (e.g. sunlight). The `direction`
    /// field carries the unit vector pointing away from the light source.
    #[default]
    Directional,
    /// A positional point light with inverse-square falloff.
    Point,
    /// A positional spotlight with a cone defined by a unit direction and a
    /// half-angle whose cosine is stored in the light's `spot_cos` field.
    Spot,
}

/// Describes the shading model applied to a [`Material`] during lighting and
/// ray-tracing evaluation.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MaterialKind {
    /// Pure Lambertian diffuse, no specular highlight.
    #[default]
    Lambert,
    /// Lambertian diffuse plus Blinn-Phong style specular highlight.
    Phong,
    /// Physically based rendering model placeholder (full PBR is intentionally
    /// not implemented in this revision; we keep the variant so callers can
    /// branch on it without a breaking change later).
    Pbr,
}
