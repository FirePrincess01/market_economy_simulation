use noise::NoiseFn;

pub struct HeightMapGenerator {
    perlin: noise::Perlin,
}

impl HeightMapGenerator {
    pub fn new() -> Self {
        let perlin: noise::Perlin = noise::Perlin::new(1);

        Self { perlin }
    }

    pub fn generate(&self, details: HeightMapDetails) -> HeightMap {
        let distance = details.distance;
        let size_x = details.size_x;
        let size_y = details.size_y;
        let p_x = details.x;
        let p_y = details.y;

        let size = size_x * size_y;

        let mut heights = Vec::with_capacity(size);
        for y in 0..size_y {
            for x in 0..size_x {
                let height = self
                    .perlin
                    // .get([(p_x + x) as f64 / 128.0, (p_y + y) as f64 / 128.0])
                    .get([
                        (p_x + x * distance) as f64 / 10.0,
                        (p_y + y * distance) as f64 / 10.0,
                    ])
                    * 10.0;

                // let height = x as f32 * 0.1;

                heights.push(height as f32);
            }
        }

        HeightMap { heights, details }
    }
}

#[derive(Debug)]
pub struct HeightMap {
    pub heights: Vec<f32>,
    pub details: HeightMapDetails,
}

#[derive(Clone, Debug)]
pub struct HeightMapDetails {
    pub distance: usize,
    pub size_x: usize,
    pub size_y: usize,
    pub x: usize,
    pub y: usize,

    pub index: usize, // index of the structure on the client
    pub lod: usize,   // level of detail of the structure on the client
}

#[test]
fn test_heightmap_generator() {
    let generator = HeightMapGenerator::new();

    let res_0 = generator.generate(HeightMapDetails {
        distance: 1,
        size_x: 4,
        size_y: 4,
        x: 0,
        y: 0,
        index: 0,
        lod: 0,
    });

    let res_1 = generator.generate(HeightMapDetails {
        distance: 2,
        size_x: 2,
        size_y: 2,
        x: 0,
        y: 0,
        index: 0,
        lod: 0,
    });

    println!("{:#?}", res_0);
    println!("{:#?}", res_1);
}

#[test]
fn test_perlin_noise() {
    let perlin: noise::Perlin = noise::Perlin::new(1);
    {
        let noise_0 = perlin.get([0.0, 0.0]);
        let noise_1 = perlin.get([0.0, 0.0]);
        assert_eq!(noise_0, noise_1);
    }

    {
        let noise_0 = perlin.get([100.5, 100.5]);
        let noise_1 = perlin.get([100.5, 100.5]);
        assert_eq!(noise_0, noise_1);
    }
}
