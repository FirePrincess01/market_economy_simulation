

pub trait DeferredShaderMeshDraw
{
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}