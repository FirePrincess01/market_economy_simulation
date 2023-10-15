

use crate::deferred_color_shader::{DeferredShaderDraw, self, GBuffer};
use crate::deferred_light_shader::DeferredLightShaderDraw;
use crate::performance_monitor::PerformanceMonitor;
use wgpu_renderer::renderer::{WgpuRenderer, self};
use wgpu_renderer::vertex_color_shader::{self, VertexColorShaderDraw};
use winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta};

use crate::{deferred_light_shader};

pub struct Renderer 
{   
    // wgpu_renderer
    pub wgpu_renderer: WgpuRenderer,

    _pipeline_color: vertex_color_shader::Pipeline,
    pipeline_lines: vertex_color_shader::Pipeline,

    g_buffer_bind_group_layout: deferred_light_shader::GBufferBindGroupLayout,
    g_buffer: deferred_color_shader::GBuffer,
    pipeline_deferred_color: deferred_color_shader::Pipeline,
    pipeline_deferred_light: deferred_light_shader::Pipeline,

    // camera
    camera: renderer::camera::Camera,
    camera_controller: super::camera_controller::CameraController,
    projection: renderer::camera::Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    camera_uniform_orthographic: vertex_color_shader::CameraUniform,
    camera_uniform_orthographic_buffer: vertex_color_shader::CameraUniformBuffer,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self 
    {   
        // wgpu renderer
        let mut wgpu_renderer = WgpuRenderer::new(window).await; 
        let _surface_format = wgpu_renderer.config().format;
        let surface_width = wgpu_renderer.config().width;
        let surface_height = wgpu_renderer.config().height;
        let surface_format = wgpu_renderer.config().format;
        
        // pipeline color
        let camera_bind_group_layout = vertex_color_shader::CameraBindGroupLayout::new(wgpu_renderer.device());
        let pipeline_color = vertex_color_shader::Pipeline::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );

        // pipeline lines
        let pipeline_lines = vertex_color_shader::Pipeline::new_lines(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );

        // g_buffer
        let g_buffer_bind_group_layout = deferred_light_shader::GBufferBindGroupLayout::new(wgpu_renderer.device());
        let g_buffer = deferred_color_shader::GBuffer::new(&mut wgpu_renderer, 
            &g_buffer_bind_group_layout,
            surface_width, 
            surface_height);

        // pipeline deferred color
        let pipeline_deferred_color = deferred_color_shader::Pipeline::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            surface_format,
        );

        // pipeline deferred light
        let pipeline_deferred_light = deferred_light_shader::Pipeline::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout, 
            &g_buffer_bind_group_layout,
            surface_format,
        );

        // camera 
        let position = cgmath::Point3::new(0.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let mut camera = renderer::camera::Camera::new(position, yaw, pitch);
        // Self::top_view_point(&mut camera);
        Self::side_view_point(&mut camera);

        let speed = 4.0;
        let sensitivity = 1.0;
        let sensitivity_scroll = 1.0;
        let camera_controller = super::camera_controller::CameraController::new(speed, sensitivity, sensitivity_scroll);

        let width = wgpu_renderer.config().width;
        let height = wgpu_renderer.config().height;
        let fovy = cgmath::Deg(45.0);
        let znear = 0.1;
        let zfar = 100.0;
        let projection = renderer::camera::Projection::new(width, height, fovy, znear, zfar);

        let camera_uniform = vertex_color_shader::CameraUniform::new();

        let camera_uniform_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(), 
            &camera_bind_group_layout);

        let camera_uniform_orthographic: vertex_color_shader::CameraUniform = vertex_color_shader::CameraUniform::new_orthographic(width, height);
        let mut camera_uniform_orthographic_buffer = vertex_color_shader::CameraUniformBuffer::new(
                wgpu_renderer.device(), 
                &camera_bind_group_layout);

        camera_uniform_orthographic_buffer.update(wgpu_renderer.queue(), camera_uniform_orthographic);   // add uniform identity matrix

        Self {
            wgpu_renderer,

            _pipeline_color: pipeline_color,
            pipeline_lines,

            g_buffer_bind_group_layout,
            g_buffer,
            pipeline_deferred_color,
            pipeline_deferred_light,

            camera,
            camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            camera_uniform_orthographic,
            camera_uniform_orthographic_buffer,
        } 
    }

    fn _top_view_point(camera: &mut renderer::camera::Camera) {
        let position = cgmath::Point3::new(0.0, 0.0, 10.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(0.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    fn side_view_point(camera: &mut renderer::camera::Camera) {
        let position = cgmath::Point3::new(0.0, -2.0, 5.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(30.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // self.size = new_size;
        
        self.projection.resize(new_size.width, new_size.height);
        self.wgpu_renderer.resize(new_size);
        self.g_buffer = GBuffer::new(&mut self.wgpu_renderer,
            &self.g_buffer_bind_group_layout,
            new_size.width,
            new_size.height);
    
        self.camera_uniform_orthographic.resize_orthographic(new_size.width, new_size.height);
        self.camera_uniform_orthographic_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform_orthographic);
    }

    pub fn update(&mut self, dt: instant::Duration) {

        // camera
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.camera_uniform_buffer.update(self.wgpu_renderer.queue(), self.camera_uniform);
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool 
    {
        self.camera_controller.process_keyboard(key, state)
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) 
    {
        self.camera_controller.process_scroll(delta);
    }


    fn render_deferred(&self, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        meshes: & [&dyn DeferredShaderDraw],
    )
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("Deferred Render Pass"), 
            color_attachments: &[
            Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.02,
                        b: 0.03,
                        a: 1.0,
                    }),
                    store: true,
                }
            }),
            Some(wgpu::RenderPassColorAttachment {
                view: &self.g_buffer.position.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                }
            }),
            Some(wgpu::RenderPassColorAttachment {
                view: &self.g_buffer.normal.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                }
            }),
            Some(wgpu::RenderPassColorAttachment {
                view: &self.g_buffer.albedo.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                }
            }),
            Some(wgpu::RenderPassColorAttachment {
                view: &self.g_buffer.entity.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                }
            }),
            ], 
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.wgpu_renderer.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }) 
        });

        self.pipeline_deferred_color.bind(&mut render_pass);
        self.camera_uniform_buffer.bind(&mut render_pass);

        // meshes
        for mesh in meshes {
            mesh.draw(&mut render_pass);
        }        
    }

    fn render_light(&self, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        mesh: & impl DeferredLightShaderDraw,
    )
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("Light Render Pass"), 
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // load: wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.02,
                        b: 0.03,
                        a: 1.0,
                    }),
                    store: true,
                }
            })], 
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.wgpu_renderer.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    // load: wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }) 
        });

        // lights
        self.pipeline_deferred_light.bind(&mut render_pass);
        self.camera_uniform_buffer.bind(&mut render_pass);
        self.g_buffer.bind(&mut render_pass);
        mesh.draw_lights(&mut render_pass);
    }

    fn render_forward(&self, 
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        performance_monitor: & impl VertexColorShaderDraw,
    )
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("Forward Render Pass"), 
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                }
            })], 
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.wgpu_renderer.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                }),
                stencil_ops: None,
            }) 
        });

        // performance monitor
        self.pipeline_lines.bind(&mut render_pass);
        self.camera_uniform_orthographic_buffer.bind(&mut render_pass);
        performance_monitor.draw(&mut render_pass);
    }


    pub fn render(&mut self, 
        mesh_deferred: & impl DeferredShaderDraw,
        mesh: & (impl DeferredShaderDraw + DeferredLightShaderDraw), 
        performance_monitor: &mut PerformanceMonitor) -> Result<(), wgpu::SurfaceError>
    {
        performance_monitor.watch.start(0);
        let output = self.wgpu_renderer.get_current_texture()?;
        performance_monitor.watch.stop(0);

        performance_monitor.watch.start(1);

        let view: wgpu::TextureView = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: wgpu::CommandEncoder = self.wgpu_renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder"),
        });

        self.render_deferred(&view, &mut encoder, &[mesh_deferred, mesh]);
        self.render_light(&view, &mut encoder, mesh);
        self.render_forward(&view, &mut encoder, performance_monitor);

        self.wgpu_renderer.queue().submit(std::iter::once(encoder.finish()));
        output.present();

        performance_monitor.watch.stop(1);
        
        Ok(())
    }
}


