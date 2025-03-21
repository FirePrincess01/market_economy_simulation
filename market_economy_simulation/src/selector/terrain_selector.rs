//! Determines the coordinate
//!

use cgmath::Zero;

use crate::terrain_storage::terrain_texture_details;

use super::{
    ray_triangle_intersection::{get_intersection_from_ray, ray_triangle_intersect},
    Triangle,
};

pub struct TerrainSelector {}

impl TerrainSelector {
    pub fn new() -> Self {
        Self {  }
    }
    
    pub fn find_intersection(
        &self,
        height_map_detail: &terrain_texture_details::TerrainTextureDetails,
        height_map: &[f32],
        view_position: &cgmath::Vector3<f32>,
        view_direction: &cgmath::Vector3<f32>,
    ) -> Option<Triangle> {
        let x_0 = height_map_detail.pos_0.x;
        let y_0 = height_map_detail.pos_0.y;
        let distance = height_map_detail.point_distance;
        let n = height_map_detail.size_0;

        for y in 0..n - 1 {
            for x in 0..n - 1 {
                let pos_0 = cgmath::Vector3::new(
                    (x_0 + (x * distance) as isize) as f32,
                    (y_0 + (y * distance) as isize) as f32,
                    height_map[y * n + x],
                );
                let pos_1 = cgmath::Vector3::new(
                    (x_0 + ((x + 1) * distance) as isize) as f32,
                    (y_0 + (y * distance) as isize) as f32,
                    height_map[y * n + (x + 1)],
                );
                let pos_2 = cgmath::Vector3::new(
                    (x_0 + ((x + 1) * distance) as isize) as f32,
                    (y_0 + ((y + 1) * distance) as isize) as f32,
                    height_map[(y + 1) * n + (x + 1)],
                );
                let pos_3 = cgmath::Vector3::new(
                    (x_0 + (x * distance) as isize) as f32,
                    (y_0 + ((y + 1) * distance) as isize) as f32,
                    height_map[(y + 1) * n + x],
                );

                {
                    // triangle 0
                    let v0 = pos_0;
                    let v1 = pos_1;
                    let v2 = pos_2;

                    let res = ray_triangle_intersect(view_position, view_direction, &v0, &v1, &v2);
                    if let Some(res) = res {
                        let t = res[0];
                        let u = res[1];
                        let v = res[2];
                        let p = get_intersection_from_ray(view_position, view_direction, t);

                        return Some(Triangle {
                            v0,
                            v1,
                            v2,
                            orig: *view_position,
                            dir: *view_direction,
                            distance: t,
                            u,
                            v,
                            p,
                        });
                    }
                }

                {
                    // triangle 1
                    let v0 = pos_2;
                    let v1 = pos_3;
                    let v2 = pos_0;

                    let res = ray_triangle_intersect(view_position, view_direction, &v0, &v1, &v2);
                    if let Some(res) = res {
                        let t = res[0];
                        let u = res[1];
                        let v = res[2];
                        let p = get_intersection_from_ray(view_position, view_direction, t);

                        return Some(Triangle {
                            v0,
                            v1,
                            v2,
                            orig: *view_position,
                            dir: *view_direction,
                            distance: t,
                            u,
                            v,
                            p,
                        });
                    }
                }
            }
        }

        None
    }
}
