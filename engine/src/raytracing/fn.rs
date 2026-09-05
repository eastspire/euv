use super::*;

/// Recursively traces a ray through the scene and returns the final shaded
/// color.
///
/// The function first finds the closest hit via [`closest_hit`]. On a miss
/// it returns the ambient color. On a hit it evaluates the surface material
/// with [`LightingUniforms::shade`] and, when the hit material has a
/// non-zero specular component, recurses with a reflected ray up to
/// `max_bounces` times (decrementing the ray's `depth` field).
///
/// # Arguments
///
/// - `Ray` - The ray to trace.
/// - `&[Occluder]` - All occluding surfaces in the scene.
/// - `&LightingUniforms` - Lighting parameters used during shading.
/// - `u32` - The maximum number of bounces allowed for this ray.
///
/// # Returns
///
/// - `Vector3D` - The final traced color.
pub fn trace(
    ray: Ray,
    occluders: &[Occluder],
    lights: &LightingUniforms,
    max_bounces: u32,
) -> Vector3D {
    let ambient: Vector3D = lights.get_ambient();
    match closest_hit(&ray, occluders) {
        None => ambient,
        Some(hit) => {
            let occluder_points: Vec<(Vector3D, f64)> = collect_occluder_points(occluders);
            let material: Material = hit.get_material().clone();
            let mut color: Vector3D = lights.shade(
                hit.get_position(),
                hit.get_normal(),
                &material,
                &occluder_points,
            );
            if ray.get_depth() < max_bounces {
                let spec: f64 = material.get_specular();
                if spec > EPSILON {
                    let reflected: Ray = reflect_ray(&ray, &hit);
                    let bounced: Vector3D = trace(reflected, occluders, lights, max_bounces);
                    color += bounced.scaled(spec);
                }
            }
            color
        }
    }
}

/// Convenience wrapper that traces a ray using the [`RAYTRACE_DEFAULT_MAX_BOUNCES`]
/// constant as the bounce limit.
///
/// # Arguments
///
/// - `Ray` - The ray to trace.
/// - `&[Occluder]` - All occluding surfaces in the scene.
/// - `&LightingUniforms` - Lighting parameters used during shading.
///
/// # Returns
///
/// - `Vector3D` - The final traced color.
pub fn trace_default(ray: Ray, occluders: &[Occluder], lights: &LightingUniforms) -> Vector3D {
    trace(ray, occluders, lights, RAYTRACE_DEFAULT_MAX_BOUNCES)
}

/// Finds the closest intersection between a ray and a list of occluders.
///
/// # Arguments
///
/// - `&Ray` - The ray to test.
/// - `&[Occluder]` - The occluders to test against.
///
/// # Returns
///
/// - `Option<Hit>` - The closest hit, or `None` if the ray misses.
pub fn closest_hit(ray: &Ray, occluders: &[Occluder]) -> Option<Hit> {
    let mut best: Option<Hit> = None;
    let origin: Vector3D = ray.get_origin();
    let dir: Vector3D = ray.get_direction();
    let t_min: f64 = ray.get_t_min();
    let t_max: f64 = ray.get_t_max();
    for occ in occluders.iter() {
        let candidate: Option<Hit> = match occ.get_kind() {
            OccluderKind::Sphere => {
                let center: Vector3D = occ.get_center();
                let radius: f64 = occ.get_extent().get_x();
                match ray_sphere_intersect(origin, dir, center, radius) {
                    Some((t, n)) => {
                        if t >= t_min && t <= t_max {
                            let hit_pos: Vector3D = origin + dir.scaled(t);
                            Some(Hit {
                                t,
                                position: hit_pos,
                                normal: n,
                                material: occ.get_material().clone(),
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
            OccluderKind::Aabb => {
                let aabb_min: Vector3D = occ.get_center();
                let aabb_max: Vector3D = occ.get_extent();
                match ray_aabb_intersect(origin, dir, aabb_min, aabb_max) {
                    Some((t_near, _t_far, n)) => {
                        if t_near >= t_min && t_near <= t_max {
                            let hit_pos: Vector3D = origin + dir.scaled(t_near);
                            Some(Hit {
                                t: t_near,
                                position: hit_pos,
                                normal: n,
                                material: occ.get_material().clone(),
                            })
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
        };
        if let Some(c) = candidate {
            best = match best {
                Some(prev) if prev.get_t() <= c.get_t() => Some(prev),
                _ => Some(c),
            };
        }
    }
    best
}

/// Builds a reflected ray bouncing off the hit surface.
///
/// # Arguments
///
/// - `&Ray` - The incoming ray.
/// - `&Hit` - The hit point with normal information.
///
/// # Returns
///
/// - `Ray` - A new ray originating at the hit point with the reflected
///   direction, `t_min` reset to `RAYTRACE_DEFAULT_T_MIN`, `t_max` set to
///   `RAYTRACE_DEFAULT_T_MAX`, and `depth` incremented by one.
pub fn reflect_ray(ray: &Ray, hit: &Hit) -> Ray {
    let dir: Vector3D = ray.get_direction();
    let normal: Vector3D = hit.get_normal();
    let dot: f64 = dir.dot(normal);
    let reflected_dir: Vector3D = dir - normal.scaled(2.0 * dot);
    Ray {
        origin: hit.get_position(),
        direction: reflected_dir,
        t_min: RAYTRACE_DEFAULT_T_MIN,
        t_max: RAYTRACE_DEFAULT_T_MAX,
        depth: ray.get_depth() + 1,
    }
}

/// Returns the AABB extents `(min, max)` of an [`Occluder`].
///
/// For AABB occluders this is `(center, extent)`. For sphere occluders
/// the bounding box is computed from the center and the `.x` component of
/// `extent` (the sphere radius).
///
/// # Arguments
///
/// - `&Occluder` - The occluder to bound.
///
/// # Returns
///
/// - `(Vector3D, Vector3D)` - The `(min, max)` corners of the AABB.
pub fn occluder_aabb_extents(occluder: &Occluder) -> (Vector3D, Vector3D) {
    match occluder.get_kind() {
        OccluderKind::Aabb => (occluder.get_center(), occluder.get_extent()),
        OccluderKind::Sphere => {
            let center: Vector3D = occluder.get_center();
            let radius: f64 = occluder.get_extent().get_x();
            let r: Vector3D = Vector3D::new(radius, radius, radius);
            (center - r, center + r)
        }
    }
}

/// Helper that flattens every occluder into `(center, radius)` sphere
/// tuples used by [`soft_shadow_factor`].
///
/// For sphere occluders the tuple is `(center, radius)`. For AABB
/// occluders a conservative bounding sphere is computed from the AABB.
fn collect_occluder_points(occluders: &[Occluder]) -> Vec<(Vector3D, f64)> {
    let mut out: Vec<(Vector3D, f64)> = Vec::new();
    for occ in occluders.iter() {
        let (mn, mx): (Vector3D, Vector3D) = occluder_aabb_extents(occ);
        let cx: f64 = (mn.get_x() + mx.get_x()) * 0.5;
        let cy: f64 = (mn.get_y() + mx.get_y()) * 0.5;
        let cz: f64 = (mn.get_z() + mx.get_z()) * 0.5;
        let ex: f64 = (mx.get_x() - mn.get_x()) * 0.5;
        let ey: f64 = (mx.get_y() - mn.get_y()) * 0.5;
        let ez: f64 = (mx.get_z() - mn.get_z()) * 0.5;
        let r: f64 = (ex * ex + ey * ey + ez * ez).sqrt();
        out.push((Vector3D::new(cx, cy, cz), r));
    }
    out
}
