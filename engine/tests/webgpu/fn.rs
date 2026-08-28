use super::*;

#[test]
fn set_viewport_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, &ViewportDescriptor) = WebGpuRenderer::set_viewport;
}

#[test]
fn set_scissor_rect_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, u32, u32, u32) = WebGpuRenderer::set_scissor_rect;
}

#[test]
fn set_stencil_reference_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32) = WebGpuRenderer::set_stencil_reference;
}

#[test]
fn set_blend_constant_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, f32, f32, f32, f32) = WebGpuRenderer::set_blend_constant;
}

#[test]
fn set_bind_group_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_with_dynamic_offsets;
}

#[test]
fn set_bind_group_compute_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_compute_with_dynamic_offsets;
}

#[test]
fn generate_mipmaps_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue) = WebGpuRenderer::generate_mipmaps;
}

#[test]
fn create_shader_module_with_label_signature_pinned() {
    /// Helper body of the `_type_check` free function.
    ///
    /// # Arguments
    ///
    /// - `&WebGpuRenderer` - Shared reference to a `WebGpuRenderer`.
    /// - `&str` - Shared reference to a `str`.
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    fn _type_check(renderer: &WebGpuRenderer, source: &str, label: &str) -> JsValue {
        renderer.create_shader_module_with_label(source, label)
    }
    let _ = _type_check;
}

#[test]
fn read_buffer_is_async() {
    /// Helper body of the `assert_future` free function.
    ///
    /// # Arguments
    ///
    /// - `F: Future` - A generic type parameter.
    fn assert_future<F>(_: F)
    where
        F: Future,
    {
    }
    let fut: Ready<Option<Vec<u8>>> = ready(None);
    assert_future(fut);
}

#[test]
fn begin_render_pass_full_signature_pinned() {
    /// Helper body of the `_type_check` free function.
    ///
    /// # Arguments
    ///
    /// - `&mut WebGpuRenderer` - Mutable reference to a `WebGpuRenderer` (mutated in place).
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `&mut RenderPassColorAttachment` - Mutable reference to a `RenderPassColorAttachment` (mutated in place).
    /// - `Option<&RenderPassDepthStencilAttachment>` - A `Option<&RenderPassDepthStencilAttachment>` parameter.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    fn _type_check(
        renderer: &mut WebGpuRenderer,
        encoder: &JsValue,
        color: &mut RenderPassColorAttachment,
        depth: Option<&RenderPassDepthStencilAttachment>,
    ) -> JsValue {
        renderer.begin_render_pass_full(encoder, color, depth)
    }
    let _ = _type_check;
}

#[test]
fn create_render_pipeline_full_signature_pinned() {
    /// Helper body of the `_type_check` free function.
    ///
    /// # Arguments
    ///
    /// - `&WebGpuRenderer` - Shared reference to a `WebGpuRenderer`.
    /// - `S: AsRef<str>` - A generic type parameter.
    /// - `&[VertexBufferLayout]` - Shared reference to a `[VertexBufferLayout]`.
    /// - `&str` - Shared reference to a `str`.
    /// - `&str` - Shared reference to a `str`.
    /// - `Option<&str>` - A `Option<&str>` parameter.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    fn _type_check<S>(
        renderer: &WebGpuRenderer,
        shader_code: S,
        vertex_buffer_layouts: &[VertexBufferLayout],
        vertex_entry: &str,
        fragment_entry: &str,
        depth_format: Option<&str>,
    ) -> JsValue
    where
        S: AsRef<str>,
    {
        renderer.create_render_pipeline_full(
            shader_code,
            vertex_buffer_layouts,
            vertex_entry,
            fragment_entry,
            depth_format,
        )
    }
    let _ = _type_check::<&str>;
}

#[test]
fn create_view_signature_pinned() {
    /// Helper body of the `_type_check` free function.
    ///
    /// # Arguments
    ///
    /// - `&WebGpuRenderer` - Shared reference to a `WebGpuRenderer`.
    /// - `&JsValue` - Shared reference to a `JsValue`.
    /// - `Option<&TextureViewDescriptor>` - A `Option<&TextureViewDescriptor>` parameter.
    ///
    /// # Returns
    ///
    /// - `JsValue` - A `JsValue` value.
    fn _type_check(
        renderer: &WebGpuRenderer,
        texture: &JsValue,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> JsValue {
        renderer.create_view(texture, descriptor)
    }
    let _ = _type_check;
}

#[test]
fn push_error_scope_signature_pinned() {
    let _: fn(&WebGpuRenderer, &str) = WebGpuRenderer::push_error_scope;
}

#[test]
fn texture_view_descriptor_full_returns_canonical_shape() {
    let d: TextureViewDescriptor = TextureViewDescriptor::full();
    assert!(d.get_format().is_none());
    assert!(d.get_dimension().is_none());
    assert_eq!(d.get_base_mip_level(), 0);
    assert_eq!(d.get_mip_level_count(), 0);
    assert_eq!(d.get_base_array_layer(), 0);
    assert_eq!(d.get_array_layer_count(), 0);
    assert!(d.get_aspect().is_none());
}

#[test]
fn gpu_sampler_descriptor_default_returns_nearest_clamp() {
    let s: GpuSamplerDescriptor = GpuSamplerDescriptor::default_sampler();
    assert_eq!(s.get_mag_filter(), "nearest");
    assert_eq!(s.get_min_filter(), "nearest");
    assert_eq!(s.get_mipmap_filter(), "nearest");
    assert_eq!(s.get_address_mode_u(), "clamp-to-edge");
    assert_eq!(s.get_address_mode_v(), "clamp-to-edge");
    assert_eq!(s.get_address_mode_w(), "clamp-to-edge");
    assert!(!s.get_compare());
}
