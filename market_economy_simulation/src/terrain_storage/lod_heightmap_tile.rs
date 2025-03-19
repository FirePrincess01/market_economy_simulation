//! Triangulated texture with level of details

use std::array;

/// Number level of details layers
const LOD: usize = 4;

struct LodHeightmapTile {
    nr_fields_per_side_inner: [usize; LOD],
    nr_fields_per_side_outer: [usize; LOD],
    nr_points_per_side_inner: [usize; LOD],
    nr_points_per_side_outer: [usize; LOD],

    

    position_offset: cgmath::Vector3<f32>,
    position: cgmath::Vector3<f32>,
}

impl LodHeightmapTile {
    fn new(position: cgmath::Vector3<f32>, nr_fields_per_side: usize) -> Self {
        let mut nr_fields_per_side_inner: [usize; LOD] = [0; LOD];
        let mut nr_fields_per_side_outer: [usize; LOD] = [0; LOD];
        let mut nr_points_per_side_inner: [usize; LOD] = [0; LOD];
        let mut nr_points_per_side_outer: [usize; LOD] = [0; LOD];

        for i in 0..LOD {
            nr_fields_per_side_inner[i] = nr_fields_per_side / 2usize.pow(i as u32);
            nr_fields_per_side_outer[i] = nr_fields_per_side_inner[i] + 2;
            nr_points_per_side_inner[i] = nr_fields_per_side_inner[i] + 1;
            nr_points_per_side_outer[i] = nr_points_per_side_inner[i] + 2;
        }

        let position_offset = cgmath::Vector3::new(1.0, 1.0, 0.0);

        Self {
            nr_fields_per_side_inner,
            nr_fields_per_side_outer,
            nr_points_per_side_inner,
            nr_points_per_side_outer,
            position_offset,
            position
        }

    }
}

#[test]
fn create_tile() {
    let tile = LodHeightmapTile::new(cgmath::Vector3::new(0.0, 0.0, 0.0), 8);
}
