use cgmath::InnerSpace;

use crate::deferred_terrain_shader;

pub fn create_vertices(
    terrain: &market_economy_simulation_server::terrain::Terrain,
) -> (Vec<deferred_terrain_shader::Vertex>, Vec<u32>) {
    let mut vertices: Vec<deferred_terrain_shader::Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for y in 0..terrain.size_y - 1 {
        for x in 0..terrain.size_x - 1 {
            let height_0 = terrain.heights[y * terrain.size_x + x];
            let height_1 = terrain.heights[y * terrain.size_x + (x + 1)];
            let height_2 = terrain.heights[(y + 1) * terrain.size_x + (x + 1)];
            let height_3 = terrain.heights[(y + 1) * terrain.size_x + x];

            let pos_0 = cgmath::Vector3::new(
                x as f32 * terrain.distance - terrain.distance * terrain.size_x as f32 / 2.0,
                y as f32 * terrain.distance,
                height_0,
            );
            let pos_1 = cgmath::Vector3::new(
                (x + 1) as f32 * terrain.distance - terrain.distance * terrain.size_x as f32 / 2.0,
                y as f32 * terrain.distance,
                height_1,
            );
            let pos_2 = cgmath::Vector3::new(
                (x + 1) as f32 * terrain.distance - terrain.distance * terrain.size_x as f32 / 2.0,
                (y + 1) as f32 * terrain.distance,
                height_2,
            );
            let pos_3 = cgmath::Vector3::new(
                x as f32 * terrain.distance - terrain.distance * terrain.size_x as f32 / 2.0,
                (y + 1) as f32 * terrain.distance,
                height_3,
            );

            let pos_middle = (pos_0 + pos_1 + pos_2 + pos_3) / 4.0;

            let normal_0 = (pos_1 - pos_0).cross(pos_3 - pos_0).normalize();
            let normal_1 = (pos_2 - pos_1).cross(pos_0 - pos_1).normalize();
            let normal_2 = (pos_3 - pos_2).cross(pos_1 - pos_2).normalize();
            let normal_3 = (pos_0 - pos_3).cross(pos_2 - pos_3).normalize();

            let normal_middle = (normal_0 + normal_1 + normal_2 + normal_3).normalize();

            let indices_local: [u32; 12] = [
                0, 1, 4, // triangle 0
                1, 2, 4, // triangle 1
                2, 3, 4, // triangle 2
                3, 0, 4, // triangle 3
            ];

            let barycentric_coordinate_0 = cgmath::Vector3::new(1.0, 0.0, 0.0);
            let barycentric_coordinate_1 = cgmath::Vector3::new(1.0, 0.0, 0.0);
            let barycentric_coordinate_2 = cgmath::Vector3::new(1.0, 0.0, 0.0);
            let barycentric_coordinate_3 = cgmath::Vector3::new(1.0, 0.0, 0.0);

            let barycentric_coordinate_middle = cgmath::Vector3::new(0.0, 0.0, 0.0);

            let vertex_0 = deferred_terrain_shader::Vertex {
                position: pos_0.into(),
                normal: normal_0.into(),
                barycentric_coordinate: barycentric_coordinate_0.into(),
            };

            let vertex_1 = deferred_terrain_shader::Vertex {
                position: pos_1.into(),
                normal: normal_1.into(),
                barycentric_coordinate: barycentric_coordinate_1.into(),
            };

            let vertex_2 = deferred_terrain_shader::Vertex {
                position: pos_2.into(),
                normal: normal_2.into(),
                barycentric_coordinate: barycentric_coordinate_2.into(),
            };

            let vertex_3 = deferred_terrain_shader::Vertex {
                position: pos_3.into(),
                normal: normal_3.into(),
                barycentric_coordinate: barycentric_coordinate_3.into(),
            };

            let vertex_4 = deferred_terrain_shader::Vertex {
                position: pos_middle.into(),
                normal: normal_middle.into(),
                barycentric_coordinate: barycentric_coordinate_middle.into(),
            };

            vertices.push(vertex_0);
            vertices.push(vertex_1);
            vertices.push(vertex_2);
            vertices.push(vertex_3);
            vertices.push(vertex_4);

            let current_index = (y * (terrain.size_x - 1) + x) * 5;
            for index in indices_local {
                indices.push(current_index as u32 + index);
            }
        }
    }

    (vertices, indices)
}
