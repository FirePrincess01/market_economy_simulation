
#[derive(Clone)]
pub struct Terrain {
    pub heights: Vec<Vec<f32>>, // Two dimensional array of all heights
    pub distance: f32,          // Distance between two points

    pub size_x: usize,
    pub size_y: usize,
}

impl Terrain {
    pub fn new(steps_x: usize, steps_y: usize, distance: f32) -> Self {
        // user perlin noise to generate a terrain
        let mut heights: Vec<Vec<f32>> = Vec::new();

        let perlin = noise::Perlin::new(1);
        for y in 0..steps_y {
            let mut x_values: Vec<f32> = Vec::new();
            for x in 0..steps_x {
                let height = noise::NoiseFn::get(
                    &perlin,
                    [x as f64 / steps_x as f64, y as f64  / steps_y as f64],
                );

                x_values.push(height as f32);
                // x_values.push(0.0);
            }
            heights.push(x_values);
        }

        Self {
            heights,
            distance,
            size_x: steps_x,
            size_y: steps_y,
        }
    }
}
