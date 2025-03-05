
use cgmath::InnerSpace;
use noise::{NoiseFn, Perlin, Seedable};

use crate::deferred_terrain_shader;



struct Terrain {
    heights: Vec<Vec<f32>>, // Two dimensional array of all heights
    distance: f32, // Distance between two points

    size_x: usize,
    size_y: usize,
}

impl Terrain {
    fn new(steps_x: usize, steps_y: usize, distance: f32) -> Self {

        // user perlin noise to generate a terrain
        let mut heights: Vec<Vec<f32>> = Vec::new();

        let perlin = Perlin::new(1);
        for y in 0..steps_y {
            let mut x_values: Vec<f32> = Vec::new();
            for x in 0..steps_x {

                let height = perlin.get([x as f64 * distance as f64, y as f64 * distance as f64]);

                x_values.push(height as f32);
            }
            heights.push(x_values);
        }

        Self { heights, distance, size_x: steps_x, size_y: steps_y }
    }

    fn create_vertices(&self) -> (Vec<deferred_terrain_shader::Vertex>, Vec<u32>) {
        let mut vertices: Vec<deferred_terrain_shader::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for y in (0..self.size_y).step_by(2){
            for x in (0..self.size_x).step_by(2){
                let height_0 = self.heights[y][x];
                let height_1 = self.heights[y][x+1];
                let height_2 = self.heights[y+1][x+1];
                let height_3 = self.heights[y+1][x];

                let pos_0 = cgmath::Vector3::new(x as f32 * self.distance, y as f32 * self.distance, height_0);
                let pos_1 = cgmath::Vector3::new((x+1) as f32 * self.distance, y as f32 * self.distance, height_1);
                let pos_2 = cgmath::Vector3::new((x+1) as f32 * self.distance, (y+1) as f32 * self.distance, height_2);
                let pos_3 = cgmath::Vector3::new(x as f32 * self.distance, (y+1) as f32 * self.distance, height_3);

                let pos_middle = (pos_0 + pos_1 + pos_2 + pos_3) / 4.0;

                let normal_0 = (pos_1 - pos_0).cross(pos_3 - pos_0).normalize();
                let normal_1 = (pos_2 - pos_1).cross(pos_0 - pos_1).normalize();
                let normal_2 = (pos_3 - pos_2).cross(pos_1 - pos_2).normalize();
                let normal_3 = (pos_0 - pos_3).cross(pos_2 - pos_3).normalize();

                let normal_middle = ((normal_0 + normal_1 + normal_2 + normal_3)).normalize();

                let indices_local: [u32; 12] = [
                    0, 1, 4, // triangle 0
                    1, 2, 4, // triangle 1
                    2, 3, 4, // triangle 2
                    3, 0, 4  // triangle 3
                ];

                let barycentric_coordinate_0 = cgmath::Vector3::new(1.0, 0.0, 0.0);
                let barycentric_coordinate_1 = cgmath::Vector3::new(1.0, 0.0, 0.0);
                let barycentric_coordinate_2 = cgmath::Vector3::new(1.0, 0.0, 0.0);
                let barycentric_coordinate_3 = cgmath::Vector3::new(1.0, 0.0, 0.0);

                let barycentric_coordinate_middle = cgmath::Vector3::new(0.0, 0.0, 0.0);

                let vertex_0 = deferred_terrain_shader::Vertex{
                    position: pos_0.into(),
                    normal: normal_0.into(),
                    barycentric_coordinate: barycentric_coordinate_0.into(),
                };

                let vertex_1 = deferred_terrain_shader::Vertex{
                    position: pos_1.into(),
                    normal: normal_1.into(),
                    barycentric_coordinate: barycentric_coordinate_1.into(),
                };

                let vertex_2 = deferred_terrain_shader::Vertex{
                    position: pos_2.into(),
                    normal: normal_2.into(),
                    barycentric_coordinate: barycentric_coordinate_2.into(),
                };

                let vertex_3 = deferred_terrain_shader::Vertex{
                    position: pos_3.into(),
                    normal: normal_3.into(),
                    barycentric_coordinate: barycentric_coordinate_3.into(),
                };

                let vertex_4 = deferred_terrain_shader::Vertex{
                    position: pos_middle.into(),
                    normal: normal_middle.into(),
                    barycentric_coordinate: barycentric_coordinate_middle.into(),
                };

                vertices.push(vertex_0);
                vertices.push(vertex_1);
                vertices.push(vertex_2);
                vertices.push(vertex_3);
                vertices.push(vertex_4);

                let current_index = indices.len();
                for index in indices_local {
                    indices.push(current_index as u32 + index);
                }
        } 
    } 

        (vertices, indices)
    }

}