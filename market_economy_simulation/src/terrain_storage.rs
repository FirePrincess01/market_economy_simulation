//! Manages the data on the gpu of the terrain
//!

use cgmath::{MetricSpace, Zero};
use wgpu_renderer::{
    shape::{self, MeshDataInterface},
    wgpu_renderer::WgpuRendererInterface,
};

use crate::deferred_heightmap_shader::{self, DeferredHeightMapShaderDraw};

const LOD: usize = 4;

type HeightmapInstanceBuffer =
    deferred_heightmap_shader::InstanceBuffer<deferred_heightmap_shader::Instance>;

pub struct TerrainStorage {
    mesh: [deferred_heightmap_shader::Mesh; LOD],
    texture: deferred_heightmap_shader::Texture,

    instances: Vec<[HeightmapInstanceBuffer; LOD]>,
    height_textures: Vec<[Option<deferred_heightmap_shader::HeightmapTexture>; LOD]>,
    texture_positions: Vec<cgmath::Vector3<f32>>,

    view_position: cgmath::Vector3<f32>,

    width: usize,
    height: usize,
    size: usize,
}

impl TerrainStorage {
    pub fn new(
        renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &deferred_heightmap_shader::TextureBindGroupLayout,
        heightmap_bind_group_layout: &deferred_heightmap_shader::HeightmapBindGroupLayout,
        width: usize,
        height: usize,
    ) -> Self {
        let size: usize = width * height;
        let texture_side_length = 32;

        // mesh
        let mesh: [deferred_heightmap_shader::Mesh; LOD] = std::array::from_fn(|index| {
            let a = 2u32.pow(index as u32);

            let grid = shape::Grid::new(a as f32, texture_side_length);
            let gird_triangles = grid.triangles();
            deferred_heightmap_shader::Mesh::from_shape(renderer.device(), gird_triangles)
        });

        // texture
        let texture_bytes = include_bytes!("../res/tile.png");
        let texture_image = image::load_from_memory(texture_bytes).unwrap();
        let texture_rgba = texture_image.to_rgba8();

        let texture = deferred_heightmap_shader::Texture::new(
            renderer,
            &texture_bind_group_layout,
            &texture_rgba,
            Some("tile.png"),
        )
        .unwrap();

        let mut texture_positions: Vec<cgmath::Vector3<f32>> = Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                let pos = cgmath::Vector3::new((x * width) as f32, (y * height) as f32, 0.0);
                texture_positions.push(pos);
            }
        }

        let mut instances: Vec<[HeightmapInstanceBuffer; LOD]> = Vec::with_capacity(size);
        for i in 0..texture_side_length {
            let instance_array: [HeightmapInstanceBuffer; LOD] = std::array::from_fn(|index| {
                let a = 2u32.pow(index as u32);

                let instance = deferred_heightmap_shader::Instance {
                    position: texture_positions[i].into(),
                    color: cgmath::Vector3::new(0.2, 0.2, 0.8).into(),
                    entity: i as u32,
                    distance: a as f32,
                };
                HeightmapInstanceBuffer::new(renderer.device(), &[instance])
            });

            instances.push(instance_array);
        }

        let mut height_textures: Vec<[Option<deferred_heightmap_shader::HeightmapTexture>; LOD]> =
            Vec::with_capacity(texture_side_length);
        for i in 0..texture_side_length {
            let height_texture_array: [Option<deferred_heightmap_shader::HeightmapTexture>; LOD] =
                std::array::from_fn(|index| None);
            height_textures.push(height_texture_array);
        }

        // insert zero value at level 0
        for i in 0..texture_side_length {
            let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> = Vec::with_capacity(texture_side_length * texture_side_length);
            heightmap.resize(texture_side_length * texture_side_length, deferred_heightmap_shader::Heightmap::zero());

            let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
                renderer.device(),
                heightmap_bind_group_layout,
                &heightmap,
                width as u32,
                height as u32,
                Some("terrain"),
            );

            height_textures[i][0] = Some(height_texture);
        }

        let view_position: cgmath::Vector3<f32> = cgmath::Vector3::zero();

        Self {
            mesh,
            texture,
            instances,
            height_textures,
            texture_positions,
            view_position,
            size,
            width,
            height,
        }
    }

    fn calculate_lod_index(
        view_postion: cgmath::Vector3<f32>,
        position: cgmath::Vector3<f32>,
    ) -> usize {
        0
    }

    pub fn update_view_position(&mut self, view_position: &cgmath::Vector3<f32>) {
        self.view_position = *view_position;
    }

    pub fn update(&mut self) {
        
    }
}

impl DeferredHeightMapShaderDraw for TerrainStorage {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for i in 0..self.size {
            // get data at index
            let height_texture = &self.height_textures[i];
            let position = &self.texture_positions[i];
            let instances = &self.instances[i];

            // level of detail
            let lod = Self::calculate_lod_index(self.view_position, *position);
            let mesh = &self.mesh[lod];
            let height_texture = &height_texture[lod];
            let instance = &instances[lod];

            let texture = &self.texture;

            // draw
            if let Some(height_texture) = height_texture {
                mesh.draw(render_pass, &height_texture, &texture, instance);
            }
        }
    }
}

struct TextureLod {
    texture: [Option<deferred_heightmap_shader::Texture>; LOD],

    position: cgmath::Vector3<f32>,
}

impl TextureLod {
    fn get(
        &self,
        view_position: &cgmath::Vector3<f32>,
    ) -> &Option<deferred_heightmap_shader::Texture> {
        let distance = view_position.distance(self.position);

        return &self.texture[0];
    }
}
