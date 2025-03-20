use noise::NoiseFn;

pub struct HeightMapGenerator {
    perlin: noise::Perlin,
}

impl Default for HeightMapGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl HeightMapGenerator {
    pub fn new() -> Self {
        let perlin: noise::Perlin = noise::Perlin::new(1);

        Self { perlin }
    }

    pub fn generate(&self, details: HeightMapDetails) -> HeightMap {
        let distance = details.point_distance;
        let size_x = details.size_0;
        let size_y = details.size_0;
        let p_x = details.pos_0.x;
        let p_y = details.pos_0.y;

        let size = size_x * size_y;

        let mut heights = Vec::with_capacity(size);
        for y in 0..size_y {
            for x in 0..size_x {
                let mut height = (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 128.0,
                    (p_y + y as isize * distance as isize) as f64 / 128.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 64.0,
                    (p_y + y as isize * distance as isize) as f64 / 64.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 32.0,
                    (p_y + y as isize * distance as isize) as f64 / 32.0,
                ]) * 20.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 16.0,
                    (p_y + y as isize * distance as isize) as f64 / 16.0,
                ]) * 8.0)
                    .max(0.0);

                height += (self.perlin.get([
                    (p_x + x as isize * distance as isize) as f64 / 8.0,
                    (p_y + y as isize * distance as isize) as f64 / 8.0,
                ]) * 2.0)
                    .max(0.0);

                // create canyon
                let a = Self::depth_to_distance(7, 8);
                height *= Self::canyon(
                    (p_x + x as isize * distance as isize - a as isize / 2) as f32 / 20.0,
                ) as f64;

                heights.push(height as f32);
            }
        }

        HeightMap { heights, details }
    }

    fn canyon(x: f32) -> f32 {
        1.0 - 1.0 / (1.0 + x * x * x * x * x * x)
    }

    pub fn depth_to_distance(depth: usize, max_depth: usize) -> usize {
        let exponent = max_depth - 1 - depth;
        2usize.pow(exponent as u32)
    }
}

#[derive(Debug)]
pub struct HeightMap {
    pub heights: Vec<f32>,
    pub details: HeightMapDetails,
}

#[derive(Clone, Debug)]
pub struct HeightMapDetails {
    pub pos_0: cgmath::Vector2<isize>, // texture world position at index (0/0)
    pub pos_1: cgmath::Vector2<isize>, // texture position at index (1/1)
    pub point_distance: usize,         // distance between pos_1.x - pos_0.x

    pub size_0: usize, // nr points between (0/0) and (N/N)
    pub size_1: usize, // nr points between (1/1) and ((N-1)/(N-1)), (size_0 - 2)

    pub nr_tiles: usize, // size_0 - 3

    // pub data_index: usize, // Index in the Data.data array
    pub depth: usize,      // Depth of the Node in the quad tree
    pub node_index: usize, // Index of the Node in the quad tree
}

// #[test]
// fn test_heightmap_generator() {
//     let generator = HeightMapGenerator::new();

//     let res_0 = generator.generate(HeightMapDetails {
//         distance: 1,
//         size_x: 4,
//         size_y: 4,
//         x: 0,
//         y: 0,
//         index: 0,
//         lod: 0,
//     });

//     let res_1 = generator.generate(HeightMapDetails {
//         distance: 2,
//         size_x: 2,
//         size_y: 2,
//         x: 0,
//         y: 0,
//         index: 0,
//         lod: 0,
//     });

//     println!("{:#?}", res_0);
//     println!("{:#?}", res_1);
// }

// #[test]
// fn test_perlin_noise() {
//     let perlin: noise::Perlin = noise::Perlin::new(1);
//     {
//         let noise_0 = perlin.get([0.0, 0.0]);
//         let noise_1 = perlin.get([0.0, 0.0]);
//         assert_eq!(noise_0, noise_1);
//     }

//     {
//         let noise_0 = perlin.get([100.5, 100.5]);
//         let noise_1 = perlin.get([100.5, 100.5]);
//         assert_eq!(noise_0, noise_1);
//     }
// }
