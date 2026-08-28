/// The canvas 2D rendering context type identifier.
pub(crate) const RENDERER_CONTEXT_TYPE_2D: &str = "2d";

/// The default font family used for text rendering.
pub(crate) const RENDERER_DEFAULT_FONT_FAMILY: &str = "sans-serif";

/// The default font size in pixels.
pub(crate) const RENDERER_DEFAULT_FONT_SIZE: f64 = 16.0;

/// The default camera zoom level.
pub(crate) const RENDERER_DEFAULT_CAMERA_ZOOM: f64 = 1.0;

/// The default camera rotation in radians.
pub(crate) const RENDERER_DEFAULT_CAMERA_ROTATION: f64 = 0.0;

/// The canvas context property name for text rendering.
pub(crate) const RENDERER_PROPERTY_TEXT_RENDERING: &str = "textRendering";

/// The high-quality text rendering value for geometric precision.
pub(crate) const RENDERER_TEXT_RENDERING_GEOMETRIC_PRECISION: &str = "geometricPrecision";

/// The window property name for device pixel ratio (HiDPI scale factor).
pub(crate) const RENDERER_PROPERTY_DEVICE_PIXEL_RATIO: &str = "devicePixelRatio";

/// The fallback device pixel ratio when window detection fails.
pub(crate) const RENDERER_DEFAULT_DEVICE_PIXEL_RATIO: f64 = 1.0;

/// The canvas context property name for image smoothing quality.
pub(crate) const RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY: &str = "imageSmoothingQuality";

/// The low-quality image smoothing value (fastest, pixelated-friendly).
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_LOW: &str = "low";

/// The medium-quality image smoothing value.
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_MEDIUM: &str = "medium";

/// The high-quality image smoothing value.
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH: &str = "high";

/// The HTML element tag name for creating a canvas element.
pub(crate) const RENDERER_ELEMENT_CANVAS: &str = "canvas";

/// The default SSAA scale factor (2.0 means 4x supersampling).
pub(crate) const RENDERER_DEFAULT_SSAA_SCALE_FACTOR: f64 = 2.0;

/// The CSS composite operation string for the `Normal` blend mode.
pub(crate) const BLEND_MODE_NORMAL: &str = "source-over";

/// The CSS composite operation string for the `Multiply` blend mode.
pub(crate) const BLEND_MODE_MULTIPLY: &str = "multiply";

/// The CSS composite operation string for the `Screen` blend mode.
pub(crate) const BLEND_MODE_SCREEN: &str = "screen";

/// The CSS composite operation string for the `Lighter` blend mode.
pub(crate) const BLEND_MODE_LIGHTER: &str = "lighter";

/// The CSS composite operation string for the `Overlay` blend mode.
pub(crate) const BLEND_MODE_OVERLAY: &str = "overlay";

/// The CSS composite operation string for the `Darken` blend mode.
pub(crate) const BLEND_MODE_DARKEN: &str = "darken";

/// The CSS composite operation string for the `Lighten` blend mode.
pub(crate) const BLEND_MODE_LIGHTEN: &str = "lighten";

/// The CSS composite operation string for the `ColorDodge` blend mode.
pub(crate) const BLEND_MODE_COLOR_DODGE: &str = "color-dodge";

/// The CSS composite operation string for the `ColorBurn` blend mode.
pub(crate) const BLEND_MODE_COLOR_BURN: &str = "color-burn";

/// The CSS composite operation string for the `HardLight` blend mode.
pub(crate) const BLEND_MODE_HARD_LIGHT: &str = "hard-light";

/// The CSS composite operation string for the `SoftLight` blend mode.
pub(crate) const BLEND_MODE_SOFT_LIGHT: &str = "soft-light";

/// The CSS composite operation string for the `Difference` blend mode.
pub(crate) const BLEND_MODE_DIFFERENCE: &str = "difference";

/// The CSS composite operation string for the `Exclusion` blend mode.
pub(crate) const BLEND_MODE_EXCLUSION: &str = "exclusion";

/// The CSS composite operation string for the `Hue` blend mode.
pub(crate) const BLEND_MODE_HUE: &str = "hue";

/// The CSS composite operation string for the `Saturation` blend mode.
pub(crate) const BLEND_MODE_SATURATION: &str = "saturation";

/// The CSS composite operation string for the `Color` blend mode.
pub(crate) const BLEND_MODE_COLOR: &str = "color";

/// The CSS composite operation string for the `Luminosity` blend mode.
pub(crate) const BLEND_MODE_LUMINOSITY: &str = "luminosity";

/// The default shadow color used when no explicit color is provided.
pub(crate) const RENDERER_DEFAULT_SHADOW_COLOR: &str = "rgba(0, 0, 0, 0.5)";

/// The default shadow blur radius in pixels.
pub(crate) const RENDERER_DEFAULT_SHADOW_BLUR: f64 = 4.0;

/// The default render layer z-index for background elements.
pub(crate) const RENDERER_LAYER_BACKGROUND: i32 = 0;

/// The default render layer z-index for foreground game objects.
pub(crate) const RENDERER_LAYER_FOREGROUND: i32 = 100;

/// The default render layer z-index for UI overlay elements.
pub(crate) const RENDERER_LAYER_UI: i32 = 1000;

/// The JavaScript property name for `powerPreference` on `GpuRequestAdapterOptions`.
pub(crate) const WEBGPU_PROPERTY_POWER_PREFERENCE: &str = "powerPreference";

/// The WebGPU context type string used to obtain a `GpuCanvasContext` from a canvas element.
///
/// This is the argument to `HTMLCanvasElement.getContext(...)` - **not** a
/// property name on `Navigator`. The browser exposes WebGPU via
/// `Navigator.gpu` (the string `"gpu"`), which is a separate concept.
/// Mixing the two up is a long-standing bug; see
/// [`WEBGPU_NAVIGATOR_GPU_KEY`] for the navigator-side key.
pub(crate) const WEBGPU_CONTEXT_TYPE: &str = "webgpu";

/// The `Navigator` property name that exposes the WebGPU `GPU` interface.
///
/// Per the WebGPU spec and MDN (`Navigator.gpu`), browsers expose the
/// entry point to the API under the property name `"gpu"` - not
/// `"webgpu"`. Using [`WEBGPU_CONTEXT_TYPE`] (which is `"webgpu"`)
/// as the key for `Reflect::get(navigator, ...)` always returns
/// `undefined`, even when WebGPU is fully supported, because
/// `navigator` does not have a `"webgpu"` property. This constant
/// exists to make the correct key explicit at every probe site.
pub(crate) const WEBGPU_NAVIGATOR_GPU_KEY: &str = "gpu";

/// The JavaScript method name `requestAdapter` on `Gpu`.
pub(crate) const WEBGPU_METHOD_REQUEST_ADAPTER: &str = "requestAdapter";

/// The JavaScript method name `requestDevice` on `GpuAdapter`.
pub(crate) const WEBGPU_METHOD_REQUEST_DEVICE: &str = "requestDevice";

/// The JavaScript method name `getPreferredCanvasFormat` on `Gpu`.
pub(crate) const WEBGPU_METHOD_GET_PREFERRED_FORMAT: &str = "getPreferredCanvasFormat";

/// Error message used when the WebGPU init promise race loses to the
/// `INIT_PROMISE_TIMEOUT_MILLIS` timer. Surfaced to `JsFuture::await` as
/// `Err`, which causes the caller to fall into its `WebGPU Not Supported`
/// branch instead of leaving the UI stuck on `Initializing...`.
pub(crate) const RENDERER_TIMEOUT_ERROR_MESSAGE: &str = "WebGPU initialization timed out";

/// The JavaScript method name `configure` on `GpuCanvasContext`.
pub(crate) const WEBGPU_METHOD_CONFIGURE: &str = "configure";

/// The JavaScript method name `unconfigure` on `GpuCanvasContext`.
///
/// Releases the GPU resources associated with the canvas context so that
/// the DOM canvas can be detached/GCed and the WebGPU device can be
/// safely destroyed. Called from `WebGpuRenderer::dispose` to tear down
/// a renderer cleanly when its host component is unmounted.
pub(crate) const WEBGPU_METHOD_UNCONFIGURE: &str = "unconfigure";

/// The JavaScript method name `destroy` on `GpuDevice`.
///
/// Used by `WebGpuRenderer::dispose` to release GPU memory. After
/// `destroy` is called any further use of the device raises a JS
/// error, so this must only run as the final teardown step.
pub(crate) const WEBGPU_METHOD_DESTROY: &str = "destroy";

/// The JavaScript property name `queue` on `GpuDevice`.
pub(crate) const WEBGPU_PROPERTY_QUEUE: &str = "queue";

/// The JavaScript property name `device` on `GpuCanvasConfiguration`.
pub(crate) const WEBGPU_PROPERTY_DEVICE: &str = "device";

/// The JavaScript property name `format` on `GpuCanvasConfiguration`.
pub(crate) const WEBGPU_PROPERTY_FORMAT: &str = "format";

/// The JavaScript method name `createShaderModule` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_SHADER_MODULE: &str = "createShaderModule";

/// The JavaScript property name `code` on `GpuShaderModuleDescriptor`.
pub(crate) const WEBGPU_PROPERTY_CODE: &str = "code";

/// The JavaScript method name `createCommandEncoder` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_COMMAND_ENCODER: &str = "createCommandEncoder";

/// The JavaScript method name `getCurrentTexture` on `GpuCanvasContext`.
pub(crate) const WEBGPU_METHOD_GET_CURRENT_TEXTURE: &str = "getCurrentTexture";

/// The JavaScript method name `createView` on `GpuTexture`.
pub(crate) const WEBGPU_METHOD_CREATE_VIEW: &str = "createView";

/// The JavaScript method name `beginRenderPass` on `GpuCommandEncoder`.
pub(crate) const WEBGPU_METHOD_BEGIN_RENDER_PASS: &str = "beginRenderPass";

/// The JavaScript method name `end` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_END: &str = "end";

/// The JavaScript method name `finish` on `GpuCommandEncoder`.
pub(crate) const WEBGPU_METHOD_FINISH: &str = "finish";

/// The JavaScript method name `submit` on `GpuQueue`.
pub(crate) const WEBGPU_METHOD_SUBMIT: &str = "submit";

/// The JavaScript property name `view` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_VIEW: &str = "view";

/// The JavaScript property name `loadOp` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_LOAD_OP: &str = "loadOp";

/// The JavaScript property name `storeOp` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_STORE_OP: &str = "storeOp";

/// The JavaScript property name `clearValue` on `GpuRenderPassColorAttachment`.
pub(crate) const WEBGPU_PROPERTY_CLEAR_VALUE: &str = "clearValue";

/// The JavaScript property name `colorAttachments` on `GpuRenderPassDescriptor`.
pub(crate) const WEBGPU_PROPERTY_COLOR_ATTACHMENTS: &str = "colorAttachments";

/// The JavaScript property name `r` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_R: &str = "r";

/// The JavaScript property name `g` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_G: &str = "g";

/// The JavaScript property name `b` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_B: &str = "b";

/// The JavaScript property name `a` on `GpuColorDict`.
pub(crate) const WEBGPU_PROPERTY_A: &str = "a";

/// The WebGPU `clear` load operation string.
pub(crate) const WEBGPU_LOAD_OP_CLEAR: &str = "clear";

/// The WebGPU `store` store operation string.
pub(crate) const WEBGPU_STORE_OP_STORE: &str = "store";

/// The JavaScript method name `createRenderPipeline` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_RENDER_PIPELINE: &str = "createRenderPipeline";

/// The JavaScript method name `setPipeline` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_PIPELINE: &str = "setPipeline";

/// The JavaScript method name `draw` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_DRAW: &str = "draw";

/// The JavaScript property name `vertex` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_VERTEX: &str = "vertex";

/// The JavaScript property name `fragment` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_FRAGMENT: &str = "fragment";

/// The JavaScript property name `module` on `GpuVertexState` / `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_MODULE: &str = "module";

/// The JavaScript property name `entryPoint` on `GpuVertexState` / `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_ENTRY_POINT: &str = "entryPoint";

/// The JavaScript property name `buffers` on `GpuVertexState`.
pub(crate) const WEBGPU_PROPERTY_BUFFERS: &str = "buffers";

/// The JavaScript property name `targets` on `GpuFragmentState`.
pub(crate) const WEBGPU_PROPERTY_TARGETS: &str = "targets";

/// The JavaScript property name `primitive` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_PRIMITIVE: &str = "primitive";

/// The JavaScript property name `topology` on `GpuPrimitiveState`.
pub(crate) const WEBGPU_PROPERTY_TOPOLOGY: &str = "topology";

/// The JavaScript property name `layout` on `GpuRenderPipelineDescriptor`.
pub(crate) const WEBGPU_PROPERTY_LAYOUT: &str = "layout";

/// The spec-compliant value for `GPURenderPipelineDescriptor.layout`
/// when no explicit `GPUPipelineLayout` is supplied.
///
/// Per the WebGPU spec, `layout` is `GPUPipelineLayout | "auto"`. The
/// string `"auto"` instructs the implementation to derive an implicit
/// pipeline layout from the shader bindings. Setting `layout` to JS
/// `null` works on some browsers but is NOT spec-compliant and produces
/// a pipeline that the GPU driver may reject silently (the symptom we
/// saw: `WebGPU: No pipeline set.` at draw time, with no create error).
pub(crate) const WEBGPU_AUTO_LAYOUT: &str = "auto";

/// The vertex shader entry point name used in WGSL shaders.
pub(crate) const WEBGPU_VERTEX_ENTRY_POINT: &str = "vs_main";

/// The fragment shader entry point name used in WGSL shaders.
pub(crate) const WEBGPU_FRAGMENT_ENTRY_POINT: &str = "fs_main";

/// The WebGPU primitive topology string for triangle lists.
pub(crate) const WEBGPU_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: &str = "triangle-list";

/// The `GpuMultisampleState` property key in a render pipeline descriptor.
pub(crate) const WEBGPU_PROPERTY_MULTISAMPLE: &str = "multisample";

/// The `count` property key inside `GpuMultisampleState`.
///
/// When `count` is `1`, MSAA is disabled (one sample per pixel). Values like
/// `4` enable 4x multisample anti-aliasing.
pub(crate) const WEBGPU_PROPERTY_COUNT: &str = "count";

/// The `sampleCount` property key inside `GpuTextureDescriptor`.
///
/// Sets the multisample count for the texture. Must be `1` for non-multisampled
/// textures or `4` (or `8`, depending on adapter support) for MSAA textures.
pub(crate) const WEBGPU_PROPERTY_SAMPLE_COUNT: &str = "sampleCount";

/// The `usage` property key inside `GpuTextureDescriptor`.
///
/// Bitmask of `GpuTextureUsage` flags. We use `RENDER_ATTACHMENT` (bit 0x10)
/// for the multisample color buffer that render passes draw into and from
/// which the swap chain resolves.
pub(crate) const WEBGPU_PROPERTY_USAGE: &str = "usage";

/// The bitmask value for `GpuTextureUsage.RENDER_ATTACHMENT`.
///
/// WebGPU spec defines this as `0x10`. The multisample texture must carry this
/// usage so it can be bound as a color attachment in `beginRenderPass`.
pub(crate) const WEBGPU_TEXTURE_USAGE_RENDER_ATTACHMENT: f64 = 16.0;

/// The `format` property key inside `GpuTextureDescriptor`.
pub(crate) const WEBGPU_PROPERTY_TEXTURE_FORMAT: &str = "format";

/// The `size` property key inside `GpuTextureDescriptor`.
///
/// Holds a `GpuExtent3dDict` describing width/height/depth of the texture.
pub(crate) const WEBGPU_PROPERTY_SIZE: &str = "size";

/// The `width` property key inside `GpuExtent3dDict`.
pub(crate) const WEBGPU_PROPERTY_EXTENT_WIDTH: &str = "width";

/// The `height` property key inside `GpuExtent3dDict`.
pub(crate) const WEBGPU_PROPERTY_EXTENT_HEIGHT: &str = "height";

/// The `depthOrArrayLayers` property key inside `GpuExtent3dDict`.
///
/// Always `1` for 2D textures; required by the spec even when unused.
pub(crate) const WEBGPU_PROPERTY_EXTENT_DEPTH: &str = "depthOrArrayLayers";

/// The JavaScript method name `createTexture` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_TEXTURE: &str = "createTexture";

/// The JavaScript method name `createBuffer` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BUFFER: &str = "createBuffer";

/// The JavaScript method name `getBindGroupLayout` on `GpuRenderPipeline`.
///
/// With `layout: "auto"` pipelines the bind group layout is derived from the
/// shader; index `0` corresponds to `@group(0)` in WGSL.
pub(crate) const WEBGPU_METHOD_GET_BIND_GROUP_LAYOUT: &str = "getBindGroupLayout";

/// The JavaScript method name `createBindGroup` on `GpuDevice`.
pub(crate) const WEBGPU_METHOD_CREATE_BIND_GROUP: &str = "createBindGroup";

/// The JavaScript method name `writeBuffer` on `GpuQueue`.
///
/// Uploads host data into a `GpuBuffer` without a staging encoder, which is
/// the canonical way to refresh small per-frame uniform buffers.
pub(crate) const WEBGPU_METHOD_WRITE_BUFFER: &str = "writeBuffer";

/// The JavaScript method name `setBindGroup` on `GpuRenderPassEncoder`.
pub(crate) const WEBGPU_METHOD_SET_BIND_GROUP: &str = "setBindGroup";

/// The bitmask value for `GPUBufferUsage.UNIFORM` (`0x40`).
pub(crate) const WEBGPU_BUFFER_USAGE_UNIFORM: f64 = 64.0;

/// The bitmask value for `GPUBufferUsage.COPY_DST` (`0x08`).
///
/// Required on any buffer that is the destination of `queue.writeBuffer`.
pub(crate) const WEBGPU_BUFFER_USAGE_COPY_DST: f64 = 8.0;

/// The `entries` property key inside `GpuBindGroupDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ENTRIES: &str = "entries";

/// The `binding` property key inside `GpuBindGroupEntry`.
pub(crate) const WEBGPU_PROPERTY_BINDING: &str = "binding";

/// The `resource` property key inside `GpuBindGroupEntry`.
pub(crate) const WEBGPU_PROPERTY_RESOURCE: &str = "resource";

/// The `buffer` property key inside `GpuBufferBinding`.
pub(crate) const WEBGPU_PROPERTY_BUFFER: &str = "buffer";

/// The `resolveTarget` property key inside `GpuRenderPassColorAttachment`.
///
/// Holds the destination `GpuTextureView` for MSAA resolve. Omit when MSAA is
/// disabled; present when the attachment's texture has `sampleCount > 1`.
pub(crate) const WEBGPU_PROPERTY_RESOLVE_TARGET: &str = "resolveTarget";

/// Upper bound, in milliseconds, for how long `WebGpuRenderer::init` is
/// allowed to wait on the adapter and device promises before falling into
/// the `WebGPU Not Supported` branch.
///
/// The browser's `navigator.gpu.requestAdapter()` / `requestDevice()`
/// promises are not always guaranteed to settle — headless contexts,
/// sandboxed iframes, and some GPU device-lost paths leave them pending
/// indefinitely. To keep the UI from stalling on `Initializing...` in those
/// cases, each promise is raced against a timer that rejects after this
/// many milliseconds (see `timeout_promise` / `with_timeout` in
/// `impl.rs`); on timeout the caller surfaces `RENDERER_TIMEOUT_ERROR_MESSAGE`
/// and treats the outcome the same as "no adapter available".
pub(crate) const INIT_PROMISE_TIMEOUT_MILLIS: i32 = 3000;

/// Minification / magnification filter mode that picks the nearest texel.
pub(crate) const WEBGPU_FILTER_MODE_NEAREST: &str = "nearest";

/// `clamp-to-edge` address mode for U / V / W sampler axes.
pub(crate) const WEBGPU_ADDRESS_MODE_CLAMP_TO_EDGE: &str = "clamp-to-edge";

/// `textureSampleCompare` comparison function: keep fragments closer to
/// the camera than the reference depth.
pub(crate) const WEBGPU_COMPARE_LESS: &str = "less";

/// 2D view dimension (the default for `GpuTexture.createView`).
pub(crate) const WEBGPU_TEXTURE_VIEW_DIMENSION_2D: &str = "2d";

/// Aspect selector that exposes every channel of a multi-aspect texture.
pub(crate) const WEBGPU_TEXTURE_ASPECT_ALL: &str = "all";

/// Aspect selector that restricts a depth-stencil view to its depth channel.
pub(crate) const WEBGPU_TEXTURE_ASPECT_DEPTH_ONLY: &str = "depth-only";

/// Buffer usage: index buffer for `setIndexBuffer`.
pub(crate) const WEBGPU_BUFFER_USAGE_INDEX: f64 = 4.0;
/// Buffer usage: vertex buffer for `setVertexBuffer`.
pub(crate) const WEBGPU_BUFFER_USAGE_VERTEX: f64 = 8.0;
/// 24-bit depth, 8-bit stencil, no multisample. The most common choice
/// for a render pass's depth-stencil attachment.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS_STENCIL8: &str = "depth24plus-stencil8";

/// 16-bit unorm depth, no stencil. Cheaper than depth24plus, but no
/// stencil, slightly more z-fighting.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH16_UNORM: &str = "depth16unorm";

/// 32-bit float depth, no stencil. Required for view-space z-buffers
/// used in deferred renderers.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH32_FLOAT: &str = "depth32float";

/// 24-bit depth (no stencil) without MSAA. The default for shadow maps
/// and other single-sample depth render targets.
pub(crate) const WEBGPU_DEPTH_FORMAT_DEPTH24_PLUS: &str = "depth24plus";

/// Validation-layer error filter; catches shader compile/link errors,
/// bind-group mismatches, OOB draws, etc.
pub(crate) const WEBGPU_ERROR_FILTER_VALIDATION: &str = "validation";

/// `loadOp` / `depthLoadOp` value that preserves the previous contents
/// of the attachment.
pub(crate) const WEBGPU_LOAD_OP_LOAD: &str = "load";

/// `storeOp` value that discards the result (saves bandwidth when we
/// will not read it back, e.g. the depth buffer at the end of a pass).
pub(crate) const WEBGPU_STORE_OP_DISCARD: &str = "discard";

/// Map a buffer for CPU readback. Requires `MAP_READ` usage.
pub(crate) const WEBGPU_MAP_MODE_READ: f64 = 1.0;

/// Map a buffer for CPU write. Requires `MAP_WRITE` usage.
pub(crate) const WEBGPU_MAP_MODE_WRITE: f64 = 2.0;

/// Texture usage: read in a shader (sampled texture / uniform texel buffer).
pub(crate) const WEBGPU_TEXTURE_USAGE_COPY_SRC: f64 = 1.0;
/// Texture usage: written in a shader (storage texture).
pub(crate) const WEBGPU_TEXTURE_USAGE_COPY_DST: f64 = 2.0;
/// Texture usage: sampled texture binding.
pub(crate) const WEBGPU_TEXTURE_USAGE_TEXTURE_BINDING: f64 = 8.0;
/// Texture usage: storage texture binding.
pub(crate) const WEBGPU_TEXTURE_USAGE_STORAGE_BINDING: f64 = 16.0;

/// `GpuCommandEncoder.beginComputePass(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_BEGIN_COMPUTE_PASS: &str = "beginComputePass";

/// `GpuCommandEncoder.copyTextureToBuffer(...)` method name.
pub(crate) const WEBGPU_METHOD_COPY_TEXTURE_TO_BUFFER: &str = "copyTextureToBuffer";

/// `GpuDevice.createComputePipeline(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_CREATE_COMPUTE_PIPELINE: &str = "createComputePipeline";

/// `GpuDevice.createSampler(descriptor)` method name.
pub(crate) const WEBGPU_METHOD_CREATE_SAMPLER: &str = "createSampler";

/// `GpuTexture.generateMipmap()` method name (Chrome extension, not spec).
pub(crate) const WEBGPU_METHOD_GENERATE_MIPMAP: &str = "generateMipmap";

/// `GpuBuffer.getMappedRange(offset, size)` method name.
pub(crate) const WEBGPU_METHOD_GET_MAPPED_RANGE: &str = "getMappedRange";

/// `GpuBuffer.mapAsync(mode, offset, size)` method name.
pub(crate) const WEBGPU_METHOD_MAP_ASYNC: &str = "mapAsync";

/// `GpuDevice.popErrorScope()` method name.
pub(crate) const WEBGPU_METHOD_POP_ERROR_SCOPE: &str = "popErrorScope";

/// `GpuDevice.pushErrorScope(filter)` method name.
pub(crate) const WEBGPU_METHOD_PUSH_ERROR_SCOPE: &str = "pushErrorScope";

/// `GpuRenderPassEncoder.setBlendConstant(color)` method name.
pub(crate) const WEBGPU_METHOD_SET_BLEND_CONSTANT: &str = "setBlendConstant";

/// `GpuRenderPassEncoder.setScissorRect(x, y, w, h)` method name.
pub(crate) const WEBGPU_METHOD_SET_SCISSOR_RECT: &str = "setScissorRect";

/// `GpuRenderPassEncoder.setStencilReference(value)` method name.
pub(crate) const WEBGPU_METHOD_SET_STENCIL_REFERENCE: &str = "setStencilReference";

/// `GpuRenderPassEncoder.setViewport(x, y, w, h, minDepth, maxDepth)` method name.
pub(crate) const WEBGPU_METHOD_SET_VIEWPORT: &str = "setViewport";

/// `GpuBuffer.unmap()` method name.
pub(crate) const WEBGPU_METHOD_UNMAP: &str = "unmap";

/// `GpuQueue.writeTexture(destination, data, dataLayout, size)` method name.
pub(crate) const WEBGPU_METHOD_WRITE_TEXTURE: &str = "writeTexture";

/// `GpuComputePassEncoder.dispatch(x, y, z)` method name.
pub(crate) const WEBGPU_METHOD_DISPATCH: &str = "dispatch";

/// `arrayStride` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_ARRAY_STRIDE: &str = "arrayStride";

/// `stepMode` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_STEP_MODE: &str = "stepMode";

/// `attributes` property key inside `GpuVertexBufferLayout`.
pub(crate) const WEBGPU_PROPERTY_ATTRIBUTES: &str = "attributes";

/// `shaderLocation` property key inside `GpuVertexAttribute`.
pub(crate) const WEBGPU_PROPERTY_SHADER_LOCATION: &str = "shaderLocation";

/// `offset` property key inside `GpuVertexAttribute`.
pub(crate) const WEBGPU_PROPERTY_OFFSET: &str = "offset";

/// `dimension` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_DIMENSION: &str = "dimension";

/// `aspect` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ASPECT: &str = "aspect";

/// `baseMipLevel` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_BASE_MIP_LEVEL: &str = "baseMipLevel";

/// `mipLevelCount` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIP_LEVEL_COUNT: &str = "mipLevelCount";

/// `baseArrayLayer` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_BASE_ARRAY_LAYER: &str = "baseArrayLayer";

/// `arrayLayerCount` property key inside `GpuTextureViewDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ARRAY_LAYER_COUNT: &str = "arrayLayerCount";

/// `addressModeU` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_U: &str = "addressModeU";

/// `addressModeV` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_V: &str = "addressModeV";

/// `addressModeW` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_ADDRESS_MODE_W: &str = "addressModeW";

/// `magFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MAG_FILTER: &str = "magFilter";

/// `minFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIN_FILTER: &str = "minFilter";

/// `mipmapFilter` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_MIPMAP_FILTER: &str = "mipmapFilter";

/// `compare` property key inside `GpuSamplerDescriptor`.
pub(crate) const WEBGPU_PROPERTY_COMPARE: &str = "compare";

/// `sampler` property key inside a sampler binding entry.
pub(crate) const WEBGPU_PROPERTY_SAMPLER: &str = "sampler";

/// `texture` property key inside a texture binding entry / `writeTexture` dest.
pub(crate) const WEBGPU_PROPERTY_TEXTURE: &str = "texture";

/// `mipLevel` property key inside a texture binding entry / `writeTexture` dest.
pub(crate) const WEBGPU_PROPERTY_MIP_LEVEL: &str = "mipLevel";

/// `origin` property key inside `GpuImageCopyTexture`.
pub(crate) const WEBGPU_PROPERTY_ORIGIN: &str = "origin";

/// `bytesPerRow` property key inside `GpuImageDataLayout`.
pub(crate) const WEBGPU_PROPERTY_BYTES_PER_ROW: &str = "bytesPerRow";

/// `rowsPerImage` property key inside `GpuImageDataLayout`.
pub(crate) const WEBGPU_PROPERTY_ROWS_PER_IMAGE: &str = "rowsPerImage";

/// `offsetBytes` property key inside `GpuImageDataLayout` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_OFFSET_BYTES: &str = "offsetBytes";

/// `source` property key inside `GpuImageCopyTexture` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_SOURCE: &str = "source";

/// `destination` property key inside `GpuImageCopyTexture` (used by `writeTexture`).
pub(crate) const WEBGPU_PROPERTY_DESTINATION: &str = "destination";

/// `copySize` property key inside `GpuImageCopyTexture` (used by `copyTextureToBuffer`).
pub(crate) const WEBGPU_PROPERTY_COPY_SIZE: &str = "copySize";

/// `width` property key (used by `GpuExtent3D`, `GpuOrigin3D`, viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_WIDTH: &str = "width";

/// `height` property key (used by `GpuExtent3D`, `GpuOrigin3D`, viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_HEIGHT: &str = "height";

/// `depthOrArrayLayers` property key inside `GpuExtent3D`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_OR_1: &str = "depthOrArrayLayers";

/// `x` property key (used by `GpuOrigin3D` and viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_X: &str = "x";

/// `y` property key (used by `GpuOrigin3D` and viewport / scissor).
pub(crate) const WEBGPU_PROPERTY_Y: &str = "y";

/// `minDepth` property key inside `setViewport`.
pub(crate) const WEBGPU_PROPERTY_MIN_DEPTH: &str = "minDepth";

/// `maxDepth` property key inside `setViewport`.
pub(crate) const WEBGPU_PROPERTY_MAX_DEPTH: &str = "maxDepth";

/// Default `minDepth` value passed to `setViewport` when the caller did not
/// supply one. WebGPU's spec default is `0.0`.
pub(crate) const WEBGPU_DEFAULT_VIEWPORT_MIN_DEPTH: f64 = 0.0;

/// Default `maxDepth` value passed to `setViewport` when the caller did not
/// supply one. WebGPU's spec default is `1.0`.
pub(crate) const WEBGPU_DEFAULT_VIEWPORT_MAX_DEPTH: f64 = 1.0;

/// `depthClearValue` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_CLEAR_VALUE: &str = "depthClearValue";

/// `depthLoadOp` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_LOAD_OP: &str = "depthLoadOp";

/// `depthStoreOp` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_STORE_OP: &str = "depthStoreOp";

/// `depthReadOnly` property key inside `GpuRenderPassDepthStencilAttachment`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_READ_ONLY: &str = "depthReadOnly";

/// `depthCompare` property key inside `GpuDepthStencilState`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_COMPARE: &str = "depthCompare";

/// `depthWriteEnabled` property key inside `GpuDepthStencilState`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_WRITE_ENABLED: &str = "depthWriteEnabled";

/// `depthStencil` property key inside `GpuRenderPassDescriptor`.
pub(crate) const WEBGPU_PROPERTY_DEPTH_STENCIL: &str = "depthStencil";

/// `depthStencilAttachment` property key inside `GpuRenderPassDescriptor` (alt spelling).
pub(crate) const WEBGPU_PROPERTY_DEPTH_STENCIL_ATTACHMENT: &str = "depthStencilAttachment";

/// `compute` property key inside `GpuRenderPassDescriptor` (compute-pass dispatch).
pub(crate) const WEBGPU_PROPERTY_COMPUTE: &str = "compute";

/// `label` property key (debug marker string, set on every WebGPU object).
pub(crate) const WEBGPU_PROPERTY_LABEL: &str = "label";

/// `lost` property key inside the `GpuDevice.lost` Promise.
pub(crate) const WEBGPU_PROPERTY_LOST: &str = "lost";

/// `textureView` property key inside `GpuTextureBinding`.
pub(crate) const WEBGPU_PROPERTY_TEXTURE_VIEW: &str = "textureView";
