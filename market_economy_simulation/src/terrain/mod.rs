
use noise::{NoiseFn, Perlin, Seedable};



struct Terrain {
    heights: Vec<Vec<f32>>, // Two dimensional array of all heights
    distance: f32, // Distance between two points
}

impl Terrain {
    fn new(steps_x: u32, steps_y: u32, distance: f32) -> Self {

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

        Self { heights, distance  }
    }



    
}