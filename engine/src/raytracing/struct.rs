use super::*;

/// A 3D ray carrying the parameters needed for traversal and depth tracking.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Ray {
    /// The world-space origin of the ray.
    #[get(type(copy))]
    pub(crate) origin: Vector3D,
    /// The unit direction of the ray.
    #[get(type(copy))]
    pub(crate) direction: Vector3D,
    /// The minimum acceptable `t` value for valid intersections.
    #[get(type(copy))]
    pub(crate) t_min: f64,
    /// The maximum `t` value before the ray escapes the scene.
    #[get(type(copy))]
    pub(crate) t_max: f64,
    /// The current recursion depth (used to terminate recursive bounces).
    #[get(type(copy))]
    pub(crate) depth: u32,
}

/// The result of a ray-object intersection.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Hit {
    /// The ray parameter at the hit point.
    #[get(type(copy))]
    pub(crate) t: f64,
    /// The world-space hit position.
    #[get(type(copy))]
    pub(crate) position: Vector3D,
    /// The outward unit normal at the hit point.
    #[get(type(copy))]
    pub(crate) normal: Vector3D,
    /// The material of the hit surface.
    pub(crate) material: Material,
}

/// A ray-traceable geometric surface with an attached material.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct Occluder {
    /// The geometric shape represented by this occluder.
    #[get(type(copy))]
    pub(crate) kind: OccluderKind,
    /// For spheres: the center. For AABBs: the minimum corner.
    #[get(type(copy))]
    pub(crate) center: Vector3D,
    /// For spheres: `.x` is the radius. For AABBs: the maximum corner.
    #[get(type(copy))]
    pub(crate) extent: Vector3D,
    /// The surface material.
    pub(crate) material: Material,
}
