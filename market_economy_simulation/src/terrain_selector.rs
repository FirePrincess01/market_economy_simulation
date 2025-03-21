//! Determines the coordinate
//!

use cgmath::Zero;

use crate::terrain_storage::terrain_texture_details;

pub struct TerrainSelector {
    view_position: cgmath::Vector3<f32>,
    view_direction: cgmath::Vector3<f32>,
}

impl TerrainSelector {
    pub fn new() -> Self {
        Self {
            view_position: cgmath::Vector3::zero(),
            view_direction: cgmath::Vector3::zero(),
        }
    }

    pub fn calculate_intersection(
        details: &terrain_texture_details::TerrainTextureDetails,
        heightmap: &[f32],
    ) -> cgmath::Vector2<f32> {
        const n: usize = 128;
        assert_eq!(heightmap.len(), n * n);

        cgmath::Vector2::zero()
    }
}

type Vec3 = cgmath::Vector3<f32>;
type Pos3 = cgmath::Point3<f32>;

fn calculate_line_intersection(
    heightmap: &[f32],
    n: usize,
    pos_0: &cgmath::Vector2<isize>,
    view_position: &cgmath::Point3<f32>,
    view_direction: &cgmath::Vector3<f32>,
    min_z: f32,
    max_z: f32,
) {
    // 1) Move grid to the center
    let a = view_position - Vec3::new(pos_0.x as f32, pos_0.y as f32, 0.0);
    let u = view_direction;

    // 2) select the direction we like to move
    let move_x_direction = u.x >= u.y;

    // 3) intersect every point in x direction
    for x in 0..n {
        // 3.1 Check if the point a position x is still in the rage of the hight map
        let is_in_range = is_x_in_range_z(&a, &u, x as f32, min_z, max_z);

        if is_in_range {
            // 3.2 Get the indices
            let pos = at_x_location(&a, &u, x as f32);

            // 3.3 Check if the indices are valid
            let is_x_index_valid = pos.x >= 0.0 && pos.x <= (n - 1) as f32;
            let is_y_index_valid = pos.y >= 0.0 && pos.y <= (n - 1) as f32;

            if is_x_index_valid && is_y_index_valid {
                let pos_x = pos.x as usize;
                let pos_y = pos.y as usize;
                let pos_z = heightmap[pos_y * n + pos_x];

                // 3.4 Check if the height is above or below
                let is_below_plain = pos.z < pos_z; 
            }
        }

    }
}

fn is_x_in_range_z(a: &Pos3, u: &Vec3, x: f32, min_z: f32, max_z: f32) -> bool {
    (x - a.x) * u.z >= (min_z - a.z) * u.z && (x - a.x) * u.z < (max_z - a.z) * u.z
}

fn at_x_location(a: &Pos3, u: &Vec3, x: f32) -> Pos3 {
    let lambda = (x - a.x) / u.x;
    a + lambda * u
}

fn at_y_location(a: &Pos3, u: &Vec3, y: f32) -> Pos3 {
    let lambda = (y - a.y) / u.y;
    a + lambda * u
}

fn at_z_location(a: &Pos3, u: &Vec3, z: f32) -> Pos3 {
    let lambda = (z - a.z) / u.z;
    a + lambda * u
}

fn create_plain(n: usize) -> Vec<f32> {
    let mut res = Vec::new();
    for y in 0..n {
        for x in 0..n {
            res.push(1.0);
        }
    }

    res
}


/// Calculates an intersection of a line with a triangle using the **Möller-Trumbore algorithm**  
/// 
/// Details: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html
/// 
/// Input:  
/// **t0**, **t1**, **t2**: Three points representing a triangle  
/// **l0**, **l1**: Two points representing a lines  
/// 
/// Result:  
/// **true** of the triangle intersects with the lines, **false** otherwise
/// 
/// 
/// ```  
///                    C
///                    x
///   l0         l1   / \
/// --x----------x---/-x-\--------------
///                 /  P  \
///                x-------x 
///                A       B 
/// 
/// P = (1 - u - v)A + uB +vC
/// 
/// P: Intersection point
/// A: First point of the triangle 
/// B: Second point of the triangle 
/// C: Third point of the triangle 
/// 
/// 
/// ``` 
/// 
fn is_line_intersecting_triangle(a: &Pos3, b: &Pos3, c: &Pos3, l0: &Pos3, l1: &Pos3) -> bool {



    false
}

// #[test]
// fn test_west() {
//     let n = 8;
//     let heightmap = create_plain(n);
//     let pos_0 = cgmath::Vector2 { x: 1, y: 1 };

//     let view_position = cgmath::Vector3::new(-1.0, 2.0, 4.0);
//     let view_direction = cgmath::Vector3::new(1.0, 0.0, -0.5);

//     calculate_line_intersection(&heightmap, n, &pos_0, &view_position, &view_direction);
// }
