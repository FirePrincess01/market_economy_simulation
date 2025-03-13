//! Interface to draw objects of this shader
//!

pub trait DeferredLightSphereShaderDraw {
    fn draw_sphere<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
