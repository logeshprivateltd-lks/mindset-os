use bytemuck::{Pod, Zeroable};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WallpaperPreset {
    #[default]
    SolarAmberGlow = 0,
    AlpineLake = 1,
    AuroraMidnight = 2,
    SunsetHorizon = 3,
    EmeraldZen = 4,
    MinimalStudio = 5,
}

impl WallpaperPreset {
    pub fn all() -> &'static [WallpaperPreset] {
        &[
            WallpaperPreset::SolarAmberGlow,
            WallpaperPreset::AlpineLake,
            WallpaperPreset::AuroraMidnight,
            WallpaperPreset::SunsetHorizon,
            WallpaperPreset::EmeraldZen,
            WallpaperPreset::MinimalStudio,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            WallpaperPreset::SolarAmberGlow => "Solar Amber Glow",
            WallpaperPreset::AlpineLake => "Alpine Lake",
            WallpaperPreset::AuroraMidnight => "Aurora Night",
            WallpaperPreset::SunsetHorizon => "Sunset Horizon",
            WallpaperPreset::EmeraldZen => "Emerald Zen",
            WallpaperPreset::MinimalStudio => "Minimal Studio",
        }
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    pub fn as_f32(&self) -> f32 {
        *self as u32 as f32
    }

    #[inline]
    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => WallpaperPreset::SolarAmberGlow,
            1 => WallpaperPreset::AlpineLake,
            2 => WallpaperPreset::AuroraMidnight,
            3 => WallpaperPreset::SunsetHorizon,
            4 => WallpaperPreset::EmeraldZen,
            5 => WallpaperPreset::MinimalStudio,
            _ => WallpaperPreset::SolarAmberGlow,
        }
    }

    #[inline]
    pub fn from_f32(val: f32) -> Self {
        Self::from_u32(val.round() as u32)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SceneGlobals {
    pub resolution: [f32; 2],
    pub time: f32,
    pub focus_depth: f32,
    pub wallpaper_style: f32,
    pub daily_progress: f32,
    pub _pad: [f32; 2],
}

pub struct BackgroundPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl BackgroundPipeline {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mindset_background_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/background.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("mindset_background_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mindset_background_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mindset_background_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
            label: Some("mindset_background_uniforms"),
            size: std::mem::size_of::<SceneGlobals>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mindset_background_bg"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group,
        }
    }

    pub fn update_uniforms(
        &self,
        queue: &wgpu::Queue,
        resolution: [f32; 2],
        time: f32,
        focus_depth: f32,
        wallpaper_style: f32,
        daily_progress: f32,
    ) {
        let globals = SceneGlobals {
            resolution,
            time,
            focus_depth,
            wallpaper_style,
            daily_progress,
            _pad: [0.0; 2],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[globals]));
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_globals_size_unchanged() {
        assert_eq!(std::mem::size_of::<SceneGlobals>(), 32);
        assert_eq!(std::mem::align_of::<SceneGlobals>(), 4);
    }

    #[test]
    fn test_scene_globals_rust_wgsl_match() {
        let shader_source = include_str!("shaders/background.wgsl");
        let module = wgpu::naga::front::wgsl::parse_str(shader_source)
            .expect("Failed to parse background.wgsl");
        let mut found = false;
        for (_, ty) in module.types.iter() {
            if ty.name.as_deref() == Some("SceneGlobals") {
                if let wgpu::naga::TypeInner::Struct { span, members } = &ty.inner {
                    assert_eq!(
                        *span, 32,
                        "SceneGlobals in WGSL must have size 32 to match Rust struct!"
                    );
                    assert_eq!(members[4].name.as_deref(), Some("daily_progress"));
                    assert_eq!(members[4].offset, 20);
                    found = true;
                }
            }
        }
        assert!(found, "SceneGlobals struct not found in background.wgsl");
    }

    #[test]
    fn test_wallpaper_presets() {
        assert_eq!(WallpaperPreset::all().len(), 6);
        assert_eq!(WallpaperPreset::SolarAmberGlow as u32, 0);
        assert_eq!(WallpaperPreset::AlpineLake as u32, 1);
        assert_eq!(WallpaperPreset::AuroraMidnight as u32, 2);
        assert_eq!(WallpaperPreset::SunsetHorizon as u32, 3);
        assert_eq!(WallpaperPreset::EmeraldZen as u32, 4);
        assert_eq!(WallpaperPreset::MinimalStudio as u32, 5);
    }
}
