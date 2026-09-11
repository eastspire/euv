use super::*;

/// Implements construction, insertion, and querying for `SpatialHashGrid2D`.
impl SpatialHashGrid2D {
    /// Creates a new 2D spatial hash grid with the given cell size.
    ///
    /// # Arguments
    ///
    /// - `f64` - The world-space size of each grid cell.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - The new grid.
    pub fn create(cell_size: f64) -> SpatialHashGrid2D {
        let safe_size: f64 = cell_size.max(EPSILON);
        let mut grid: SpatialHashGrid2D = SpatialHashGrid2D::new(safe_size);
        grid.set_inverse_cell_size(1.0 / safe_size);
        grid
    }

    /// Creates a new 2D spatial hash grid with the default cell size.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - The new grid.
    pub fn with_default_size() -> SpatialHashGrid2D {
        Self::create(SPATIAL_DEFAULT_CELL_SIZE_2D)
    }

    /// Inserts a body index into all cells overlapping the given bounding box.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body index to insert.
    /// - `Vector2D` - The minimum corner of the bounding box.
    /// - `Vector2D` - The maximum corner of the bounding box.
    pub fn insert(&mut self, index: usize, min: Vector2D, max: Vector2D) {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                self.get_mut_cells()
                    .entry((col, row))
                    .or_default()
                    .push(index);
            }
        }
    }

    /// Returns all candidate body indices whose cells overlap the given bounding box.
    ///
    /// Deduplicates indices so each candidate appears at most once.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The list of candidate body indices.
    pub fn query(&self, min: Vector2D, max: Vector2D) -> Vec<usize> {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = Vec::new();
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                if let Some(entries) = self.get_cells().get(&(col, row)) {
                    for index in entries {
                        if seen.insert(*index) {
                            result.push(*index);
                        }
                    }
                }
            }
        }
        result
    }

    /// Removes all entries from the grid, preparing it for a fresh insertion pass.
    pub fn clear(&mut self) {
        // Preserve each cell's underlying Vec buffer across frames so the
        // spatial hash doesn't pay a fresh allocation cost on every tick.
        self.get_mut_cells().values_mut().for_each(Vec::clear);
    }

    /// Appends all candidate body indices overlapping the query box into `out`,
    /// deduplicating via the caller-provided `seen` set.
    ///
    /// Both `out` and `seen` are cleared first, so the caller can reuse the same
    /// buffers across all queries in a step without any per-query allocation.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The minimum corner of the query box.
    /// - `Vector2D` - The maximum corner of the query box.
    /// - `&mut Vec<usize>` - The output buffer, cleared then filled with candidates.
    /// - `&mut HashSet<usize>` - The dedup scratch set, cleared then reused.
    pub fn query_into(
        &self,
        min: Vector2D,
        max: Vector2D,
        out: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) {
        out.clear();
        seen.clear();
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                if let Some(entries) = self.get_cells().get(&(col, row)) {
                    for index in entries {
                        if seen.insert(*index) {
                            out.push(*index);
                        }
                    }
                }
            }
        }
    }
}

/// Implements construction, insertion, and querying for `SpatialHashGrid3D`.
impl SpatialHashGrid3D {
    /// Creates a new 3D spatial hash grid with the given cell size.
    ///
    /// # Arguments
    ///
    /// - `f64` - The world-space size of each grid cell.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - The new grid.
    pub fn create(cell_size: f64) -> SpatialHashGrid3D {
        let safe_size: f64 = cell_size.max(EPSILON);
        let mut grid: SpatialHashGrid3D = SpatialHashGrid3D::new(safe_size);
        grid.set_inverse_cell_size(1.0 / safe_size);
        grid
    }

    /// Creates a new 3D spatial hash grid with the default cell size.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - The new grid.
    pub fn with_default_size() -> SpatialHashGrid3D {
        Self::create(SPATIAL_DEFAULT_CELL_SIZE_3D)
    }

    /// Inserts a body index into all cells overlapping the given 3D bounding box.
    ///
    /// # Arguments
    ///
    /// - `usize` - The body index to insert.
    /// - `Vector3D` - The minimum corner of the bounding box.
    /// - `Vector3D` - The maximum corner of the bounding box.
    pub fn insert(&mut self, index: usize, min: Vector3D, max: Vector3D) {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    self.get_mut_cells()
                        .entry((col, row, layer))
                        .or_default()
                        .push(index);
                }
            }
        }
    }

    /// Returns all candidate body indices whose cells overlap the given 3D bounding box.
    ///
    /// Deduplicates indices so each candidate appears at most once.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The minimum corner of the query box.
    /// - `Vector3D` - The maximum corner of the query box.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - The list of candidate body indices.
    pub fn query(&self, min: Vector3D, max: Vector3D) -> Vec<usize> {
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        let mut seen: HashSet<usize> = HashSet::new();
        let mut result: Vec<usize> = Vec::new();
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    if let Some(entries) = self.get_cells().get(&(col, row, layer)) {
                        for index in entries {
                            if seen.insert(*index) {
                                result.push(*index);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Removes all entries from the grid, preparing it for a fresh insertion pass.
    pub fn clear(&mut self) {
        // Preserve each cell's underlying Vec buffer across frames so the
        // spatial hash doesn't pay a fresh allocation cost on every tick.
        self.get_mut_cells().values_mut().for_each(Vec::clear);
    }

    /// Appends all candidate body indices overlapping the query box into `out`,
    /// deduplicating via the caller-provided `seen` set.
    ///
    /// Both `out` and `seen` are cleared first, so the caller can reuse the same
    /// buffers across all queries in a step without any per-query allocation.
    ///
    /// # Arguments
    ///
    /// - `Vector3D` - The minimum corner of the query box.
    /// - `Vector3D` - The maximum corner of the query box.
    /// - `&mut Vec<usize>` - The output buffer, cleared then filled with candidates.
    /// - `&mut HashSet<usize>` - The dedup scratch set, cleared then reused.
    pub fn query_into(
        &self,
        min: Vector3D,
        max: Vector3D,
        out: &mut Vec<usize>,
        seen: &mut HashSet<usize>,
    ) {
        out.clear();
        seen.clear();
        let inv: f64 = self.get_inverse_cell_size();
        let min_col: i32 = (min.get_x() * inv).floor() as i32;
        let min_row: i32 = (min.get_y() * inv).floor() as i32;
        let min_layer: i32 = (min.get_z() * inv).floor() as i32;
        let max_col: i32 = (max.get_x() * inv).floor() as i32;
        let max_row: i32 = (max.get_y() * inv).floor() as i32;
        let max_layer: i32 = (max.get_z() * inv).floor() as i32;
        for col in min_col..=max_col {
            for row in min_row..=max_row {
                for layer in min_layer..=max_layer {
                    if let Some(entries) = self.get_cells().get(&(col, row, layer)) {
                        for index in entries {
                            if seen.insert(*index) {
                                out.push(*index);
                            }
                        }
                    }
                }
            }
        }
    }
}
/// Default-construction for [`SpatialHashGrid2D`].
impl Default for SpatialHashGrid2D {
    /// Constructs a default [`SpatialHashGrid2D`] value.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid2D` - A default-constructed instance with the documented initial state.
    fn default() -> SpatialHashGrid2D {
        SpatialHashGrid2D::with_default_size()
    }
}

/// Implements `Default` for `SpatialHashGrid3D` with the default cell size.
impl Default for SpatialHashGrid3D {
    /// Constructs a default [`SpatialHashGrid3D`] value.
    ///
    /// # Returns
    ///
    /// - `SpatialHashGrid3D` - A default-constructed instance with the documented initial state.
    fn default() -> SpatialHashGrid3D {
        SpatialHashGrid3D::with_default_size()
    }
}
