use super::*;

/// Implements factory constructors and accessors for [`Ray`] and
/// [`Occluder`].
impl Ray {
    /// Creates a new ray starting at `origin` pointing in `direction`.
    ///
    /// `t_min` and `t_max` default to [`RAYTRACE_DEFAULT_T_MIN`] and
    /// [`RAYTRACE_DEFAULT_T_MAX`]. `depth` defaults to 0.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The ray origin.
    /// - `Vector3D` - The unit direction.
    ///
    /// # Returns
    ///
    /// - `Ray` - The new ray.
    pub fn new(origin: Vector3D, direction: Vector3D) -> Ray {
        Ray {
            origin,
            direction,
            t_min: RAYTRACE_DEFAULT_T_MIN,
            t_max: RAYTRACE_DEFAULT_T_MAX,
            depth: 0,
        }
    }

    /// Computes the world-space point at distance `t` along this ray.
    ///
    /// # Arguments
    ///
    /// - `f64` - The ray parameter.
    ///
    /// # Returns
    ///
    /// - `Vector3D` - `origin + direction * t`.
    pub fn at(&self, t: f64) -> Vector3D {
        self.get_origin() + self.get_direction().scaled(t)
    }

    /// Returns a clone of this ray with `depth` replaced by `depth`.
    ///
    /// # Arguments
    ///
    /// - `u32` - The new recursion depth.
    ///
    /// # Returns
    ///
    /// - `Ray` - The cloned ray with updated depth.
    pub fn with_depth(&self, depth: u32) -> Ray {
        Ray {
            origin: self.get_origin(),
            direction: self.get_direction(),
            t_min: self.get_t_min(),
            t_max: self.get_t_max(),
            depth,
        }
    }
}

/// Implements factory constructors for [`Occluder`].
impl Occluder {
    /// Creates a spherical occluder centered at `center` with `radius`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The sphere center.
    /// - `f64` - The sphere radius.
    /// - `Material` - The surface material.
    ///
    /// # Returns
    ///
    /// - `Occluder` - The new sphere occluder.
    pub fn sphere(center: Vector3D, radius: f64, material: Material) -> Occluder {
        Occluder {
            kind: OccluderKind::Sphere,
            center,
            extent: Vector3D::new(radius, radius, radius),
            material,
        }
    }

    /// Creates an axis-aligned bounding-box occluder from `min` to `max`.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The AABB minimum corner.
    /// - `Vector3D` - The AABB maximum corner.
    /// - `Material` - The surface material.
    ///
    /// # Returns
    ///
    /// - `Occluder` - The new AABB occluder.
    pub fn aabb(min: Vector3D, max: Vector3D, material: Material) -> Occluder {
        Occluder {
            kind: OccluderKind::Aabb,
            center: min,
            extent: max,
            material,
        }
    }

    /// Returns a list of `(center, radius)` sphere tuples approximating
    /// this occluder, suitable for [`soft_shadow_factor`].
    ///
    /// For sphere occluders this returns `(center, radius)`. For AABB
    /// occluders the bounding sphere is computed conservatively from the
    /// AABB extents.
    ///
    /// # Returns
    ///
    /// - `Vec<(Vector3D, f64)>` - One bounding sphere per occluder.
    pub fn occluder_points(&self) -> Vec<(Vector3D, f64)> {
        let (mn, mx): (Vector3D, Vector3D) = occluder_aabb_extents(self);
        let cx: f64 = (mn.get_x() + mx.get_x()) * 0.5;
        let cy: f64 = (mn.get_y() + mx.get_y()) * 0.5;
        let cz: f64 = (mn.get_z() + mx.get_z()) * 0.5;
        let ex: f64 = (mx.get_x() - mn.get_x()) * 0.5;
        let ey: f64 = (mx.get_y() - mn.get_y()) * 0.5;
        let ez: f64 = (mx.get_z() - mn.get_z()) * 0.5;
        let r: f64 = (ex * ex + ey * ey + ez * ez).sqrt();
        vec![(Vector3D::new(cx, cy, cz), r)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A ray that escapes the scene (no occluders) returns the ambient color.
    #[test]
    fn trace_miss_returns_ambient() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 0.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::new(0.2, 0.4, 0.6));
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0));
        let occluders: [Occluder; 0] = [];
        let color: Vector3D = trace(ray, &occluders, &lights, RAYTRACE_DEFAULT_MAX_BOUNCES);
        assert!(
            (color.get_x() - 0.2).abs() < EPSILON,
            "expected ambient red 0.2, got {}",
            color.get_x(),
        );
        assert!(
            (color.get_y() - 0.4).abs() < EPSILON,
            "expected ambient green 0.4, got {}",
            color.get_y(),
        );
        assert!(
            (color.get_z() - 0.6).abs() < EPSILON,
            "expected ambient blue 0.6, got {}",
            color.get_z(),
        );
    }

    /// A ray that hits an emissive sphere returns the sphere's emissive
    /// color (no shadow attenuation because the surface IS the light).
    #[test]
    fn trace_emissive_sphere() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 5.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::zero());
        let sphere_material: Material = Material::emissive(Vector3D::new(1.0, 0.0, 0.0));
        let sphere: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, sphere_material);
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 5.0), Vector3D::new(0.0, 0.0, -1.0));
        let occluders: [Occluder; 1] = [sphere];
        let color: Vector3D = trace(ray, &occluders, &lights, RAYTRACE_DEFAULT_MAX_BOUNCES);
        assert!(
            (color.get_x() - 1.0).abs() < EPSILON,
            "expected emissive red 1.0, got {}",
            color.get_x(),
        );
        assert!(
            color.get_y().abs() < EPSILON,
            "expected emissive green 0.0, got {}",
            color.get_y(),
        );
        assert!(
            color.get_z().abs() < EPSILON,
            "expected emissive blue 0.0, got {}",
            color.get_z(),
        );
    }

    /// A ray that hits a mirror sphere (Phong specular = 1.0) reflects
    /// once and lands on an emissive sphere, returning a mixed color.
    #[test]
    fn trace_reflection_single_bounce() {
        let eye: Vector3D = Vector3D::new(0.0, 0.0, 10.0);
        let mut lights: LightingUniforms = LightingUniforms::with_eye(eye);
        lights.set_ambient(Vector3D::zero());
        let mirror_material: Material = Material::phong(Vector3D::zero(), 1.0, 32.0);
        let mirror: Occluder = Occluder::sphere(Vector3D::zero(), 1.0, mirror_material);
        // Emissive sphere along +z past the mirror. Ray bounces straight
        // back along +z after hitting the dead-center +z hemisphere, so
        // place the emissive on that line.
        let emissive_material: Material = Material::emissive(Vector3D::new(0.0, 1.0, 0.0));
        let emissive: Occluder =
            Occluder::sphere(Vector3D::new(0.0, 0.0, 15.0), 1.0, emissive_material);
        let ray: Ray = Ray::new(Vector3D::new(0.0, 0.0, 10.0), Vector3D::new(0.0, 0.0, -1.0));
        let occluders: [Occluder; 2] = [mirror, emissive];
        let color: Vector3D = trace(ray, &occluders, &lights, RAYTRACE_DEFAULT_MAX_BOUNCES);
        assert!(
            color.get_y() > 0.0,
            "expected bounce to bring back some green, got {}",
            color.get_y(),
        );
        assert!(
            color.get_x().abs() < EPSILON,
            "expected red ~0 (no red light), got {}",
            color.get_x(),
        );
        assert!(
            color.get_z().abs() < EPSILON,
            "expected blue ~0 (no blue light), got {}",
            color.get_z(),
        );
    }
}
