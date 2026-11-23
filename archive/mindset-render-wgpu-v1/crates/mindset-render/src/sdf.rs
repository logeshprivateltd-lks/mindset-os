use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SdfInstance {
    /// For rounded boxes: [x, y, width, height] (in logical pixels).
    /// For progress rings: [center_x, center_y, radius, thickness].
    pub rect: [f32; 4],
    /// For rounded boxes: 4 corner radii: top-left, top-right, bottom-right, bottom-left.
    /// For progress rings: [-1.0, -1.0, -1.0, -1.0] (negative x uniquely marks progress ring).
    pub radii: [f32; 4],
    /// RGBA surface tint / track color for rings
    pub fill_color: [f32; 4],
    /// RGBA top-lit specular rim highlight / accent color for rings
    pub border_color: [f32; 4],
    /// RGBA drop shadow color (unused for rings)
    pub shadow_color: [f32; 4],
    /// For rounded boxes: [border_width, shadow_blur, shadow_offset_y, glass_mix]
    /// For progress rings: [progress [0..1], start_angle_radians, 1.0, 0.0]
    pub params: [f32; 4],
    /// Scissor clip boundary: [min_x, min_y, max_x, max_y]. Set to [0.0; 4] for unclipped.
    pub clip_rect: [f32; 4],
}

impl SdfInstance {
    /// Helper to construct a standard frosted glass panel with drop shadow
    pub fn glass_panel(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        tint: [f32; 4],
        border_color: [f32; 4],
    ) -> Self {
        Self {
            rect: [x, y, width, height],
            radii: [radius, radius, radius, radius],
            fill_color: tint,
            border_color,
            shadow_color: [0.0, 0.0, 0.0, 0.45],
            params: [1.2, 16.0, 8.0, 0.85], // border=1.2, blur=16, offset_y=8, glass=0.85
            clip_rect: [0.0; 4],
        }
    }

    /// Helper for interactive buttons
    #[allow(clippy::too_many_arguments)]
    pub fn button(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        radius: f32,
        fill: [f32; 4],
        border: [f32; 4],
        glass: f32,
    ) -> Self {
        Self {
            rect: [x, y, width, height],
            radii: [radius, radius, radius, radius],
            fill_color: fill,
            border_color: border,
            shadow_color: [0.0, 0.0, 0.0, 0.30],
            params: [1.0, 8.0, 3.0, glass],
            clip_rect: [0.0; 4],
        }
    }

    /// Helper for smooth anti-aliased SDF progress rings.
    ///
    /// Ensures the progress ring's radius is strictly clamped (e.g. max 64px, never 200px),
    /// preventing massive unclipped circles that blow up and overlay text.
    #[allow(clippy::too_many_arguments)]
    pub fn progress_ring(
        cx: f32,
        cy: f32,
        outer_radius: f32,
        thickness: f32,
        progress: f32,
        start_angle_radians: f32,
        track_color: [f32; 4],
        accent_color: [f32; 4],
    ) -> Self {
        let clamped_radius = outer_radius.clamp(4.0, 64.0);
        let clamped_thickness = thickness.clamp(1.5, (clamped_radius * 0.7).max(2.0));
        let cx = if cx.is_finite() { cx } else { 0.0 };
        let cy = if cy.is_finite() { cy } else { 0.0 };

        Self {
            rect: [cx, cy, clamped_radius, clamped_thickness],
            radii: [-1.0, -1.0, -1.0, -1.0], // Uniquely marks SDF Progress Ring
            fill_color: track_color,
            border_color: accent_color,
            shadow_color: [0.0; 4],
            params: [progress.clamp(0.0, 1.0), start_angle_radians, 1.0, 0.0],
            clip_rect: [0.0; 4],
        }
    }

    /// Bounded progress ring constructor that clamps radius and center coordinates to a widget's
    /// `LayoutRect` [x, y, width, height] and enforces parent scissor clipping.
    #[allow(clippy::too_many_arguments)]
    pub fn progress_ring_bounded(
        cx: f32,
        cy: f32,
        outer_radius: f32,
        thickness: f32,
        widget_rect: [f32; 4], // [x, y, width, height]
        progress: f32,
        start_angle_radians: f32,
        track_color: [f32; 4],
        accent_color: [f32; 4],
    ) -> Self {
        let [wx, wy, ww, wh] = widget_rect;
        let margin = 4.0_f32;
        // Strictly clamp radius to fit within widget's LayoutRect dimensions, never 200px
        let max_fitting_radius = ((ww.min(wh) * 0.5) - margin).max(4.0);
        let clamped_radius = outer_radius.clamp(4.0, max_fitting_radius).min(64.0);
        let clamped_thickness = thickness.clamp(1.5, (clamped_radius * 0.7).max(2.0));

        // Clamp center_x and center_y so the ring does not bleed outside the widget rect
        let min_cx = wx + clamped_radius + margin;
        let max_cx = (wx + ww - clamped_radius - margin).max(min_cx);
        let min_cy = wy + clamped_radius + margin;
        let max_cy = (wy + wh - clamped_radius - margin).max(min_cy);

        let clamped_cx = cx.clamp(min_cx, max_cx);
        let clamped_cy = cy.clamp(min_cy, max_cy);

        let clip_rect = [wx, wy, wx + ww, wy + wh];

        Self {
            rect: [clamped_cx, clamped_cy, clamped_radius, clamped_thickness],
            radii: [-1.0, -1.0, -1.0, -1.0],
            fill_color: track_color,
            border_color: accent_color,
            shadow_color: [0.0; 4],
            params: [progress.clamp(0.0, 1.0), start_angle_radians, 1.0, 0.0],
            clip_rect,
        }
    }

    /// Progress ring for list items, small cards, or constrained containers.
    ///
    /// Ensures default radius is small (e.g. 12px or clamped to min(rect.w, rect.h)*0.5 - margin),
    /// never 200px, and applies parent scissor clipping.
    #[allow(clippy::too_many_arguments)]
    pub fn progress_ring_in_rect(
        widget_rect: [f32; 4], // [x, y, width, height]
        progress: f32,
        start_angle_radians: f32,
        track_color: [f32; 4],
        accent_color: [f32; 4],
    ) -> Self {
        let [wx, wy, ww, wh] = widget_rect;
        let margin = 4.0_f32;
        let max_fitting_radius = ((ww.min(wh) * 0.5) - margin).max(4.0);
        let default_radius = 12.0_f32.min(max_fitting_radius);
        let thickness = (default_radius * 0.30).clamp(2.0, 5.0);
        let cx = wx + ww * 0.5;
        let cy = wy + wh * 0.5;

        Self::progress_ring_bounded(
            cx,
            cy,
            default_radius,
            thickness,
            widget_rect,
            progress,
            start_angle_radians,
            track_color,
            accent_color,
        )
    }

    /// Attaches an explicit scissor clipping rectangle [min_x, min_y, max_x, max_y] to this instance.
    pub fn with_clip(mut self, clip_rect: [f32; 4]) -> Self {
        self.clip_rect = clip_rect;
        self
    }

    /// Attaches a bounding box [x, y, width, height] as the scissor clip rect [min_x, min_y, max_x, max_y].
    pub fn with_clip_bounds(mut self, x: f32, y: f32, width: f32, height: f32) -> Self {
        self.clip_rect = [x, y, x + width, y + height];
        self
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SdfUniforms {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

pub struct SdfPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_capacity: usize,
    pub instances: Vec<SdfInstance>,
    pub active_clip: Option<[f32; 4]>,
}

impl SdfPipeline {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mindset_sdf_panel_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sdf_panel.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mindset_sdf_bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mindset_sdf_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Vertex buffer layout describing SdfInstance (step mode = Instance)
        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SdfInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // location(0) rect: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(1) radii: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(2) fill_color: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(3) border_color: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(4) shadow_color: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 64,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(5) params: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 80,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // location(6) clip_rect: vec4<f32>
                wgpu::VertexAttribute {
                    offset: 96,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mindset_sdf_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[instance_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mindset_sdf_uniforms"),
            size: std::mem::size_of::<SdfUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let initial_capacity = 256;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("mindset_sdf_instances"),
            size: (initial_capacity * std::mem::size_of::<SdfInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout,
            uniform_buffer,
            instance_buffer,
            instance_capacity: initial_capacity,
            instances: Vec::with_capacity(initial_capacity),
            active_clip: None,
        }
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.active_clip = None;
    }

    /// Sets the active scissor clipping rectangle [min_x, min_y, max_x, max_y] applied to subsequently pushed instances.
    pub fn set_clip_rect(&mut self, clip: Option<[f32; 4]>) {
        self.active_clip = clip;
    }

    /// Sets the active scissor clip from [x, y, width, height] bounds.
    pub fn set_clip_bounds(&mut self, bounds: Option<[f32; 4]>) {
        self.active_clip = bounds.map(|[x, y, w, h]| [x, y, x + w, y + h]);
    }

    pub fn push(&mut self, mut instance: SdfInstance) {
        if let Some(clip) = self.active_clip {
            if instance.clip_rect[2] <= instance.clip_rect[0]
                || instance.clip_rect[3] <= instance.clip_rect[1]
            {
                instance.clip_rect = clip;
            } else {
                instance.clip_rect[0] = instance.clip_rect[0].max(clip[0]);
                instance.clip_rect[1] = instance.clip_rect[1].max(clip[1]);
                instance.clip_rect[2] = instance.clip_rect[2].min(clip[2]);
                instance.clip_rect[3] = instance.clip_rect[3].min(clip[3]);
            }
        }
        self.instances.push(instance);
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        blurred_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mindset_sdf_bg"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(blurred_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen_size: [f32; 2]) {
        let uniforms = SdfUniforms {
            screen_size,
            _pad: [0.0, 0.0],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        if self.instances.is_empty() {
            return;
        }

        // Reallocate buffer if instance count exceeds capacity
        if self.instances.len() > self.instance_capacity {
            self.instance_capacity = (self.instances.len() * 2).max(256);
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("mindset_sdf_instances_resized"),
                size: (self.instance_capacity * std::mem::size_of::<SdfInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, bind_group: &'a wgpu::BindGroup) {
        if self.instances.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        // 6 vertices per quad (0..6), drawn for each instance (0..len)
        pass.draw(0..6, 0..self.instances.len() as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_uniforms_size_and_wgsl_match() {
        assert_eq!(std::mem::size_of::<SdfUniforms>(), 16);
        let shader_source = include_str!("shaders/sdf_panel.wgsl");
        let module = wgpu::naga::front::wgsl::parse_str(shader_source)
            .expect("Failed to parse sdf_panel.wgsl");
        let mut found = false;
        for (_, ty) in module.types.iter() {
            if ty.name.as_deref() == Some("SdfUniforms") {
                if let wgpu::naga::TypeInner::Struct { span, .. } = &ty.inner {
                    assert_eq!(*span, 16, "SdfUniforms in WGSL must have size 16");
                    found = true;
                }
            }
        }
        assert!(found, "SdfUniforms struct not found in sdf_panel.wgsl");
    }

    #[test]
    fn test_sdf_instance_size_and_vertex_layout() {
        assert_eq!(std::mem::size_of::<SdfInstance>(), 112);
        let shader_source = include_str!("shaders/sdf_panel.wgsl");
        let module = wgpu::naga::front::wgsl::parse_str(shader_source)
            .expect("Failed to parse sdf_panel.wgsl");
        let mut found_vertex_input = false;
        for (_, ty) in module.types.iter() {
            if ty.name.as_deref() == Some("VertexInput") {
                found_vertex_input = true;
            }
        }
        assert!(
            found_vertex_input,
            "VertexInput struct not found in sdf_panel.wgsl"
        );
    }

    #[test]
    fn test_progress_ring_clamping_never_200px() {
        // Attempt to create a ring with giant radius 200px
        let ring = SdfInstance::progress_ring(
            150.0,
            150.0,
            200.0,
            15.0,
            0.75,
            0.0,
            [0.2, 0.2, 0.2, 1.0],
            [1.0, 0.5, 0.0, 1.0],
        );

        assert!(
            ring.rect[2] <= 64.0,
            "Progress ring radius must never be 200px; got {}",
            ring.rect[2]
        );
        assert_eq!(
            ring.radii[0], -1.0,
            "Progress ring radii[0] must be -1.0 flag"
        );
    }

    #[test]
    fn test_progress_ring_bounded_to_widget_rect() {
        let widget = [100.0, 100.0, 80.0, 60.0]; // 80x60 container
        let ring = SdfInstance::progress_ring_bounded(
            140.0,
            130.0,
            200.0, // giant unclipped radius attempt
            8.0,
            widget,
            0.5,
            0.0,
            [0.1, 0.1, 0.1, 1.0],
            [0.0, 1.0, 0.5, 1.0],
        );

        // Max fitting radius in 80x60 container is min(80, 60)*0.5 - 4.0 = 26.0
        assert!(
            ring.rect[2] <= 26.0,
            "Ring radius must fit within widget's LayoutRect, got {}",
            ring.rect[2]
        );
        assert!(ring.rect[2] < 200.0, "Ring radius must never be 200px");

        // Center must be clamped within widget
        let r = ring.rect[2];
        assert!(ring.rect[0] >= widget[0] + r);
        assert!(ring.rect[0] <= widget[0] + widget[2] - r);
        assert!(ring.rect[1] >= widget[1] + r);
        assert!(ring.rect[1] <= widget[1] + widget[3] - r);

        // Scissor clip rect must match widget bounds
        assert_eq!(ring.clip_rect, [100.0, 100.0, 180.0, 160.0]);
    }

    #[test]
    fn test_progress_ring_in_rect_defaults() {
        let list_item = [20.0, 40.0, 200.0, 36.0]; // List item row (height 36px)
        let ring = SdfInstance::progress_ring_in_rect(
            list_item,
            0.8,
            0.0,
            [0.2, 0.2, 0.2, 1.0],
            [0.2, 0.6, 1.0, 1.0],
        );

        // Default radius in list item should be small (<= 12px or clamped to min(w, h)*0.5 - margin), never 200px
        assert!(
            ring.rect[2] <= 14.0,
            "List item progress ring must have small radius, got {}",
            ring.rect[2]
        );
        assert!(ring.rect[2] >= 4.0);
        assert_eq!(ring.clip_rect, [20.0, 40.0, 220.0, 76.0]);
    }

    #[test]
    fn test_sdf_pipeline_scissor_clipping_propagation() {
        let mut pipeline_mock_instances: Vec<SdfInstance> = Vec::new();
        let clip = Some([50.0, 50.0, 250.0, 250.0]);

        let mut instance =
            SdfInstance::progress_ring(100.0, 100.0, 20.0, 4.0, 0.5, 0.0, [0.0; 4], [1.0; 4]);
        assert_eq!(instance.clip_rect, [0.0; 4]);

        // Simulating push with active_clip
        if let Some(c) = clip {
            instance.clip_rect = c;
        }
        pipeline_mock_instances.push(instance);

        assert_eq!(
            pipeline_mock_instances[0].clip_rect,
            [50.0, 50.0, 250.0, 250.0]
        );
    }
}
