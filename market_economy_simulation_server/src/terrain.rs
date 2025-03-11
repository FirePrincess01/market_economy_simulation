#[derive(Clone)]
pub struct Terrain {
    pub heights: Vec<f32>, // Two dimensional array of all heights
    pub distance: f32,          // Distance between two points

    pub size_x: usize,
    pub size_y: usize,
}

impl Terrain {
    pub fn new(size_x: usize, size_y: usize, distance: f32) -> Self {
        let _amplitude = 10.0;
        let _scale = 0.1;

        // user perlin noise to generate a terrain
        let mut heights: Vec<f32> = Vec::new();
        heights.resize(size_x * size_y, 0.0);

        let perlin = noise::Perlin::new(1);

        for y in 0..size_y {
            for x in 0..size_x {
                let mut height =  (noise::NoiseFn::get(&perlin, [x as f64 / 32.0, y as f64 / 32.0]) * 20.0).max(0.0) as f32;
                height +=  (noise::NoiseFn::get(&perlin, [x as f64 / 16.0, y as f64 / 16.0]) * 10.0).max(0.0) as f32;
                height +=  (noise::NoiseFn::get(&perlin, [x as f64 / 8.0, y as f64 / 8.0]) * 5.0).max(0.0) as f32;
                height +=  (noise::NoiseFn::get(&perlin, [x as f64 / 4.0, y as f64 / 4.0]) * 2.0).max(0.0) as f32;

                // create canyon 
                height *= Terrain::canyon((x as f32 - size_x as f32 / 2.0) / 10.0);

                heights[y * size_x + x] = height;
            }
        }
   
        Self {
            heights,
            distance,
            size_x,
            size_y,
        }
    }

    fn canyon(x: f32) -> f32 {
        1.0 - 1.0 / (1.0 + x *x *x *x *x *x)
    }
}
