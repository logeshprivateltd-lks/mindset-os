//! Target management for full-resolution scene color and half-resolution
//! Dual-Kawase ping-pong blur textures.

pub struct MaterialTargetManager {
    pub width: u32,
    pub height: u32,
    pub blur_width: u32,
    pub blur_height: u32,
    pub format: wgpu::TextureFormat,

    // Full-resolution offscreen scene color target (background + lower layers)
    pub scene_color_tex: wgpu::Texture,
    pub scene_color_view: wgpu::TextureView,

    // Half-resolution ping-pong blur textures
    pub blur_ping_tex: wgpu::Texture,
    pub blur_ping_view: wgpu::TextureView,
    pub blur_pong_tex: wgpu::Texture,
    pub blur_pong_view: wgpu::TextureView,

    // Bilinear clamping sampler for smooth tent-filter and downsampling
    pub linear_sampler: wgpu::Sampler,
}

impl MaterialTargetManager {
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> Self {
        let safe_width = width.max(1);
        let safe_height = height.max(1);
        let blur_width = (safe_width / 2).max(1);
        let blur_height = (safe_height / 2).max(1);

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("mindset_linear_clamp_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let (scene_color_tex, scene_color_view) =
            Self::create_color_texture(device, safe_width, safe_height, format, "scene_color");
        let (blur_ping_tex, blur_ping_view) =
            Self::create_color_texture(device, blur_width, blur_height, format, "blur_ping");
        let (blur_pong_tex, blur_pong_view) =
            Self::create_color_texture(device, blur_width, blur_height, format, "blur_pong");

        Self {
            width: safe_width,
            height: safe_height,
            blur_width,
            blur_height,
            format,
            scene_color_tex,
            scene_color_view,
            blur_ping_tex,
            blur_ping_view,
            blur_pong_tex,
            blur_pong_view,
            linear_sampler,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let safe_width = width.max(1);
        let safe_height = height.max(1);
        if safe_width == self.width && safe_height == self.height {
            return;
        }

        self.width = safe_width;
        self.height = safe_height;
        self.blur_width = (safe_width / 2).max(1);
        self.blur_height = (safe_height / 2).max(1);

        let (scene_color_tex, scene_color_view) =
            Self::create_color_texture(device, self.width, self.height, self.format, "scene_color");
        let (blur_ping_tex, blur_ping_view) = Self::create_color_texture(
            device,
            self.blur_width,
            self.blur_height,
            self.format,
            "blur_ping",
        );
        let (blur_pong_tex, blur_pong_view) = Self::create_color_texture(
            device,
            self.blur_width,
            self.blur_height,
            self.format,
            "blur_pong",
        );

        self.scene_color_tex = scene_color_tex;
        self.scene_color_view = scene_color_view;
        self.blur_ping_tex = blur_ping_tex;
        self.blur_ping_view = blur_ping_view;
        self.blur_pong_tex = blur_pong_tex;
        self.blur_pong_view = blur_pong_view;
    }

    fn create_color_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        label: &str,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("mindset_material_{label}_tex")),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("mindset_material_{label}_view")),
            ..Default::default()
        });
        (tex, view)
    }
}
