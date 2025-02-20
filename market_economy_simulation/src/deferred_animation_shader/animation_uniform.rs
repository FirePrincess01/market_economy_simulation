//! The AnimationUniform struct used in the shader
//!

use cgmath::prelude::*;

const MAX_JOINTS: usize = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AnimationUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub joint_transform: [[[f32; 4]; 4]; MAX_JOINTS],
}

impl AnimationUniform {
    pub fn zero() -> Self {
        // use cgmath::SquareMatrix;

        let uniform_mat: [[f32; 4]; MAX_JOINTS] = cgmath::Matrix4::identity().into();

        let joint_transform: [[[f32; 4]; 4]; MAX_JOINTS] = [uniform_mat; MAX_JOINTS];

        Self { joint_transform }
    }

    // fn update_view_proj(&mut self, camera: &Camera) {
    //     self.view_proj = camera.build_view_projection_matrix().into();
    // }

    // pub fn update_view_proj(
    //     &mut self,
    //     camera: &renderer::camera::Camera,
    //     projection: &renderer::camera::Projection,
    // ) {
    //     self.view_position = camera.position.to_homogeneous().into();
    //     self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    // }
}
