
pub struct GBufferFormat {
    pub position: wgpu::TextureFormat,
    pub normal: wgpu::TextureFormat,
    pub albedo: wgpu::TextureFormat,
    // pub specular: wgpu::TextureFormat,
}

impl GBufferFormat {
    pub fn new() -> Self {

        let position = wgpu::TextureFormat::Rgba16Float;
        let normal = wgpu::TextureFormat::Rgba16Float;
        let albedo = wgpu::TextureFormat::Rgba8UnormSrgb;
        // let specular = wgpu::TextureFormat::R16Float;

        Self {
            position,
            normal,
            albedo,
            // specular,
        }
    }
}

pub struct GBufferTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl GBufferTexture {
    pub fn new(device: &wgpu::Device, 
        surface_width: u32, surface_height: u32,
        format: wgpu::TextureFormat,
        label: &str) -> Self
    {
        let size = wgpu::Extent3d {
            width: surface_width,
            height: surface_height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: Default::default(),
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Self {
            texture,
            view,
            sampler,
        }
    }
}

pub struct GBuffer {
    format: GBufferFormat,

    pub position: GBufferTexture,
    pub normal: GBufferTexture,
    pub albedo: GBufferTexture,
    // pub specular: GBufferTexture,
}

impl GBuffer {
    pub fn new(device: &wgpu::Device, surface_width: u32, surface_height: u32) -> Self 
    {
        let format = GBufferFormat::new();

        let (position, normal, albedo) = 
        Self::create_buffers(device, surface_width, surface_height, &format);

        

        Self {
            position,
            normal,
            albedo,
            // specular,

            format
        }
    }

    fn create_buffers(device: &wgpu::Device, 
        surface_width: u32, surface_height: u32,
        format: &GBufferFormat) 
        -> (GBufferTexture, GBufferTexture, GBufferTexture)
    {
        let position = GBufferTexture::new(device, 
            surface_width, surface_height, 
            format.position, 
            "GBuffer Position");

        let normal = GBufferTexture::new(device, 
            surface_width, surface_height, 
            format.normal, 
            "GBuffer Normal");

        let albedo = GBufferTexture::new(device, 
            surface_width, surface_height, 
            format.albedo, 
            "GBuffer Albedo");

        // let specular = GBufferTexture::new(device, 
        //     surface_width, surface_height, 
        //     format.specular, 
        //     "GBuffer Specular");

        (position, normal, albedo)
    }

    pub fn resize(&mut self, device: &wgpu::Device, surface_width: u32, surface_height: u32) {
        let (position, normal, albedo) = 
            Self::create_buffers(device, surface_width, surface_height, &self.format);

        self.position = position;
        self.normal = normal;
        self.albedo = albedo;
        // self.specular = specular;
    }

    pub fn get_format(&self) -> &GBufferFormat {
        &self.format
    }

}