//! Interface to draw objects of this shader
//!

pub trait DeferredLightShaderDraw {
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}
