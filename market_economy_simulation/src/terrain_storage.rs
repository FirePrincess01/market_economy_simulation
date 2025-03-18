//! Manages the data on the gpu of the terrain
//!

use std::sync::mpsc;

use cgmath::{MetricSpace, Zero};
use market_economy_simulation_server::{game_logic::game_logic_interface::GameLogicMessageRequest, heightmap_generator};
use wgpu_renderer::{
    shape::{self, MeshDataInterface},
    vertex_heightmap_shader,
    wgpu_renderer::WgpuRendererInterface,
};

use crate::deferred_heightmap_shader::{self, DeferredHeightMapShaderDraw};

const LOD: usize = 4;

type HeightmapInstanceBuffer =
    deferred_heightmap_shader::InstanceBuffer<deferred_heightmap_shader::Instance>;

pub struct TerrainSettings {
    pub terrain_tile_size: usize,
    pub terrain_size: (usize, usize),
}

pub struct TerrainStorage {
    settings: TerrainSettings,

    mesh: [deferred_heightmap_shader::Mesh; LOD],
    texture: deferred_heightmap_shader::Texture,

    instances: Vec<[HeightmapInstanceBuffer; LOD]>,
    height_textures: Vec<[Option<deferred_heightmap_shader::HeightmapTexture>; LOD]>,
    height_texture_details: Vec<[heightmap_generator::HeightMapDetails; LOD]>,
    height_textures_state: Vec<[HeightTextureState; LOD]>,
    texture_positions: Vec<cgmath::Vector3<f32>>,

    view_position: cgmath::Vector3<f32>,

    width: usize,
    height: usize,
    size: usize,
    tile_size: usize,

    requests: Vec<heightmap_generator::HeightMapDetails>,
}

impl TerrainStorage {
    pub fn new(
        settings: TerrainSettings,
        renderer: &mut dyn WgpuRendererInterface,
        texture_bind_group_layout: &deferred_heightmap_shader::TextureBindGroupLayout,
        heightmap_bind_group_layout: &deferred_heightmap_shader::HeightmapBindGroupLayout,
    ) -> Self {
        let width = settings.terrain_size.0;
        let height = settings.terrain_size.1;
        let size: usize = width * height;
        let texture_side_length = settings.terrain_tile_size + 1;

        // mesh
        let mesh: [deferred_heightmap_shader::Mesh; LOD] = std::array::from_fn(|index| {
            let a = 2u32.pow(index as u32);

            let grid = shape::Grid::new(1.0, settings.terrain_tile_size / (a as usize) + 1);
            let gird_triangles = grid.triangles();
            deferred_heightmap_shader::Mesh::from_shape(renderer.device(), gird_triangles)
        });

        // texture
        let texture_bytes = include_bytes!("../res/tile.png");
        // let texture_bytes = include_bytes!("../res/pony2.png");
        let texture_image = image::load_from_memory(texture_bytes).unwrap();
        let texture_rgba = texture_image.to_rgba8();

        let texture = deferred_heightmap_shader::Texture::new_with_mipmaps(
            renderer,
            &texture_bind_group_layout,
            &texture_rgba,
            Some("tile.png"),
            9,
        )
        .unwrap();

        // position
        let offset_y = (texture_side_length - 1) * height / 2;
        let offset_x = (texture_side_length - 1) * width / 2;
        let mut texture_positions: Vec<cgmath::Vector3<f32>> = Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                let pos = cgmath::Vector3::new(
                    (x * (texture_side_length - 1)) as f32 - offset_x as f32,
                    (y * (texture_side_length - 1)) as f32 - offset_y as f32,
                    0.0,
                );
                texture_positions.push(pos);
            }
        }

        let mut instances: Vec<[HeightmapInstanceBuffer; LOD]> = Vec::with_capacity(size);
        for i in 0..size {
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
            Vec::with_capacity(size);
        for i in 0..size {
            let height_texture_array: [Option<deferred_heightmap_shader::HeightmapTexture>; LOD] =
                std::array::from_fn(|index| None);
            height_textures.push(height_texture_array);
        }

        let mut height_texture_details: Vec<[heightmap_generator::HeightMapDetails; LOD]> =
            Vec::with_capacity(size);
        for y in 0..height {
            for x in 0..width {
                let height_texture_details_array: [heightmap_generator::HeightMapDetails; LOD] =
                    std::array::from_fn(|lod| {
                        let a = 2u32.pow(lod as u32);

                        heightmap_generator::HeightMapDetails {
                            distance: a as usize,
                            size_x: texture_side_length / a as usize,
                            size_y: texture_side_length / a as usize,
                            x: x * (texture_side_length - 1),
                            y: y * (texture_side_length - 1),
                            index: y * width + x,
                            lod,
                        }
                    });
                    height_texture_details.push(height_texture_details_array);
            }
        }

        let mut height_textures_state: Vec<[HeightTextureState; LOD]> = Vec::with_capacity(size);
        for i in 0..size {
            let height_textures_state_array: [HeightTextureState; LOD] =
                std::array::from_fn(|index| HeightTextureState::NotRequested);
            height_textures_state.push(height_textures_state_array);
        }

        // insert zero value at level 0
        // for i in 0..size {
        //     {
        //         let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> =
        //             Vec::with_capacity(texture_side_length * texture_side_length);
        //         heightmap.resize(
        //             texture_side_length * texture_side_length,
        //             deferred_heightmap_shader::Heightmap::zero(),
        //         );

        //         let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
        //             renderer.device(),
        //             heightmap_bind_group_layout,
        //             &heightmap,
        //             width as u32,
        //             height as u32,
        //             Some("terrain"),
        //         );

        //         height_textures[i][0] = Some(height_texture);
        //     }
        //     {
        //         let side_length = texture_side_length / 2;
        //         let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> =
        //             Vec::with_capacity(side_length * side_length);
        //         heightmap.resize(
        //             side_length * side_length,
        //             deferred_heightmap_shader::Heightmap::zero(),
        //         );

        //         let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
        //             renderer.device(),
        //             heightmap_bind_group_layout,
        //             &heightmap,
        //             width as u32,
        //             height as u32,
        //             Some("terrain"),
        //         );

        //         height_textures[i][1] = Some(height_texture);
        //     }

        //     {
        //         let side_length = texture_side_length / 4;
        //         let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> =
        //             Vec::with_capacity(side_length * side_length);
        //         heightmap.resize(
        //             side_length * side_length,
        //             deferred_heightmap_shader::Heightmap::zero(),
        //         );

        //         let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
        //             renderer.device(),
        //             heightmap_bind_group_layout,
        //             &heightmap,
        //             width as u32,
        //             height as u32,
        //             Some("terrain"),
        //         );

        //         height_textures[i][2] = Some(height_texture);
        //     }

        //     {
        //         let side_length = texture_side_length / 8;
        //         let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> =
        //             Vec::with_capacity(side_length * side_length);
        //         heightmap.resize(
        //             side_length * side_length,
        //             deferred_heightmap_shader::Heightmap::zero(),
        //         );

        //         let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
        //             renderer.device(),
        //             heightmap_bind_group_layout,
        //             &heightmap,
        //             width as u32,
        //             height as u32,
        //             Some("terrain"),
        //         );

        //         height_textures[i][3] = Some(height_texture);
        //     }
        // }

        let view_position: cgmath::Vector3<f32> = cgmath::Vector3::zero();

        let tile_size = settings.terrain_tile_size;

        Self {
            settings,
            mesh,
            texture,
            instances,
            height_textures,
            height_texture_details,
            height_textures_state,
            texture_positions,
            view_position,
            size,
            width,
            height,
            tile_size,
            requests: Vec::new(),
        }
    }

    fn calculate_lod_index(
        view_position: cgmath::Vector3<f32>,
        position: cgmath::Vector3<f32>,
        tile_size: usize,
    ) -> usize {
        let distance = view_position.distance(position);
        if distance > tile_size as f32 * 6.0 {
            3
        } else if distance > tile_size as f32 * 4.0 {
            2
        } else if distance > tile_size as f32 * 2.0 {
            1
        } else {
            0
        }
    }

    pub fn update_view_position(&mut self, view_position: &cgmath::Vector3<f32>) {
        self.view_position = *view_position;
    }

    pub fn submit_requests(&mut self, sender: &mpsc::Sender<GameLogicMessageRequest>) {
        for elem in &self.requests {
            sender.send(GameLogicMessageRequest::GetTerrain(elem.clone()));
        }

        self.requests.clear();
    }

    pub(crate) fn update_height_map(
        &mut self,
        renderer: &mut dyn WgpuRendererInterface,
        heightmap_bind_group_layout: &deferred_heightmap_shader::HeightmapBindGroupLayout,
        height_map: market_economy_simulation_server::heightmap_generator::HeightMap,
    ) {
        let distance = height_map.details.distance;
        let size_x = height_map.details.size_x;
        let size_y = height_map.details.size_y;
        let p_x = height_map.details.x;
        let p_y = height_map.details.y;

        let size = size_x * size_y;

        // create host data
        let mut heightmap: Vec<deferred_heightmap_shader::Heightmap> = Vec::with_capacity(size);

        assert_eq!(height_map.heights.len(), size);

        for elem in height_map.heights {
            heightmap.push(vertex_heightmap_shader::Heightmap { height: elem });
        }

        // create device data
        let height_texture = deferred_heightmap_shader::HeightmapTexture::new(
            renderer,
            heightmap_bind_group_layout,
            &heightmap,
            size_x as u32,
            size_y as u32,
            Some("terrain"),
        );

        // set result
        let index = height_map.details.index;
        let lod = height_map.details.lod;
        self.height_textures[index][lod] = Some(height_texture);
        self.height_textures_state[index][lod] = HeightTextureState::Available;
    }
}

impl DeferredHeightMapShaderDraw for TerrainStorage {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        for i in 0..self.size {
            // get data at index
            let position = &self.texture_positions[i];

            // level of detail
            let lod = Self::calculate_lod_index(self.view_position, *position, self.tile_size);
            let mesh = &self.mesh[lod];
            let texture = &self.texture;
            let height_texture = &self.height_textures[i][lod];
            let instance = &self.instances[i][lod];

            // draw
            match self.height_textures_state[i][lod] {
                HeightTextureState::NotRequested => {
                    self.requests.push(self.height_texture_details[i][lod].clone());
                    self.height_textures_state[i][lod] = HeightTextureState::IsRequested;
                }
                HeightTextureState::IsRequested => {
                    // nothing to do, waiting for a result
                }
                HeightTextureState::Available => {
                    // draw
                    if let Some(height_texture) = height_texture {
                        mesh.draw(render_pass, &height_texture, &texture, instance);
                    }
                }
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

enum HeightTextureState {
    NotRequested,
    IsRequested,
    Available,
}
