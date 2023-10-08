

pub trait DeferredLightShaderDraw
{
    fn draw_lights<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}