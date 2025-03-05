//! Renders everything
//!

mod camera_controller;

use crate::animated_object::wgpu_animated_object_renderer::WgpuAnimatedObjectStorage;
use crate::deferred_color_shader::{self, DeferredShaderDraw, EntityBuffer, GBuffer};
use crate::deferred_light_shader::DeferredLightShaderDraw;
use crate::performance_monitor::PerformanceMonitor;
use camera_controller::CameraController;
use wgpu_renderer::renderer::camera::{Camera, Projection};
use wgpu_renderer::renderer::WgpuRendererInterface;
use wgpu_renderer::vertex_color_shader::{self, VertexColorShaderDraw};
use wgpu_renderer::vertex_texture_shader::{self, VertexTextureShaderDraw};
use winit::event::{ElementState, MouseScrollDelta};

use crate::{deferred_animation_shader, deferred_light_shader};

pub struct Renderer {
    _pipeline_color: vertex_color_shader::Pipeline,
    pipeline_lines: vertex_color_shader::Pipeline,

    pub texture_bind_group_layout: vertex_texture_shader::TextureBindGroupLayout,
    pipeline_texture_gui: vertex_texture_shader::Pipeline,

    g_buffer_bind_group_layout: deferred_light_shader::GBufferBindGroupLayout,
    g_buffer: deferred_color_shader::GBuffer,
    entity_buffer: deferred_color_shader::EntityBuffer,
    pipeline_deferred_color: deferred_color_shader::Pipeline,
    pipeline_deferred_light: deferred_light_shader::Pipeline,

    pub animation_bind_group_layout: deferred_animation_shader::AnimationBindGroupLayout,
    pipeline_deferred_animated: deferred_animation_shader::Pipeline,

    // camera
    camera: Camera,
    camera_controller: CameraController,
    projection: Projection,

    camera_uniform: vertex_color_shader::CameraUniform,
    camera_uniform_buffer: vertex_color_shader::CameraUniformBuffer,

    camera_uniform_orthographic: vertex_color_shader::CameraUniform,
    camera_uniform_orthographic_buffer: vertex_color_shader::CameraUniformBuffer,
}

impl Renderer {
    pub fn new(wgpu_renderer: &mut dyn WgpuRendererInterface) -> Self {
        // wgpu renderer
        let surface_width = wgpu_renderer.surface_width();
        let surface_height = wgpu_renderer.surface_height();
        let surface_format: wgpu::TextureFormat = wgpu_renderer.surface_format();

        // pipeline color
        let camera_bind_group_layout =
            vertex_color_shader::CameraBindGroupLayout::new(wgpu_renderer.device());
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

        // pipeline texture gui
        let texture_bind_group_layout =
            vertex_texture_shader::TextureBindGroupLayout::new(wgpu_renderer.device());
        let pipeline_texture_gui = vertex_texture_shader::Pipeline::new_gui(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
            &texture_bind_group_layout,
            surface_format,
        );

        // g_buffer
        let g_buffer_bind_group_layout =
            deferred_light_shader::GBufferBindGroupLayout::new(wgpu_renderer.device());
        let g_buffer = deferred_color_shader::GBuffer::new(
            wgpu_renderer,
            &g_buffer_bind_group_layout,
            surface_width,
            surface_height,
        );

        // entity_buffer
        let entity_buffer =
            deferred_color_shader::EntityBuffer::new(wgpu_renderer, surface_width, surface_height);

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

        let animation_bind_group_layout =
            deferred_animation_shader::AnimationBindGroupLayout::new(wgpu_renderer.device());

        // pipeline deferred animated
        let pipeline_deferred_animated = deferred_animation_shader::Pipeline::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
            &animation_bind_group_layout,
            surface_format,
        );

        // camera
        let position = cgmath::Point3::new(0.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let mut camera = Camera::new(position, yaw, pitch);
        // Self::top_view_point(&mut camera);
        Self::side_view_point(&mut camera);

        let speed = 40.0;
        let sensitivity = 1.0;
        let sensitivity_scroll = 1.0;
        let camera_controller = CameraController::new(speed, sensitivity, sensitivity_scroll);

        let width = wgpu_renderer.surface_width();
        let height = wgpu_renderer.surface_height();
        let fovy = cgmath::Deg(45.0);
        let znear = 0.1;
        let zfar = 100.0;
        let projection = Projection::new(width, height, fovy, znear, zfar);

        let camera_uniform = vertex_color_shader::CameraUniform::new();

        let camera_uniform_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
        );

        let camera_uniform_orthographic: vertex_color_shader::CameraUniform =
            vertex_color_shader::CameraUniform::new_orthographic(width, height);
        let mut camera_uniform_orthographic_buffer = vertex_color_shader::CameraUniformBuffer::new(
            wgpu_renderer.device(),
            &camera_bind_group_layout,
        );

        camera_uniform_orthographic_buffer
            .update(wgpu_renderer.queue(), camera_uniform_orthographic); // add uniform identity matrix

        Self {
            _pipeline_color: pipeline_color,
            pipeline_lines,

            texture_bind_group_layout,
            pipeline_texture_gui,

            g_buffer_bind_group_layout,
            g_buffer,
            entity_buffer,
            pipeline_deferred_color,
            pipeline_deferred_light,

            animation_bind_group_layout,
            pipeline_deferred_animated,

            camera,
            camera_controller,
            projection,

            camera_uniform,
            camera_uniform_buffer,

            camera_uniform_orthographic,
            camera_uniform_orthographic_buffer,
        }
    }

    fn _top_view_point(camera: &mut Camera) {
        let position = cgmath::Point3::new(0.0, 0.0, 10.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(0.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    fn side_view_point(camera: &mut Camera) {
        let position = cgmath::Point3::new(20.0, 5.0, 12.0);
        let yaw = cgmath::Deg(-90.0).into();
        let pitch = cgmath::Deg(60.0).into();

        camera.position = position;
        camera.yaw = yaw;
        camera.pitch = pitch;
    }

    pub fn resize(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        // self.size = new_size;

        self.projection.resize(new_size.width, new_size.height);
        // self.wgpu_renderer.resize(new_size);
        self.g_buffer = GBuffer::new(
            renderer_interface,
            &self.g_buffer_bind_group_layout,
            new_size.width,
            new_size.height,
        );
        self.entity_buffer = EntityBuffer::new(renderer_interface, new_size.width, new_size.height);

        self.camera_uniform_orthographic
            .resize_orthographic(new_size.width, new_size.height);
        self.camera_uniform_orthographic_buffer
            .update(renderer_interface.queue(), self.camera_uniform_orthographic);
    }

    pub fn update(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        dt: instant::Duration,
    ) {
        // camera
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.camera_uniform_buffer
            .update(renderer_interface.queue(), self.camera_uniform);
    }

    pub fn process_keyboard(&mut self, key: winit::keyboard::KeyCode, state: ElementState) -> bool {
        self.camera_controller.process_keyboard(key, state)
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_controller.process_scroll(delta);
    }

    fn render_deferred(
        &self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        meshes: &[&dyn DeferredShaderDraw],
        animated_object_storage: &WgpuAnimatedObjectStorage,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Deferred Render Pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.02,
                            b: 0.03,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::default(),
                    },
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
                        store: wgpu::StoreOp::default(),
                    },
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
                        store: wgpu::StoreOp::default(),
                    },
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
                        store: wgpu::StoreOp::default(),
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &self.entity_buffer.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::default(),
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer_interface.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::default(),
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.pipeline_deferred_color.bind(&mut render_pass);
        self.camera_uniform_buffer.bind(&mut render_pass);

        // meshes
        for mesh in meshes {
            mesh.draw(&mut render_pass);
        }

        self.pipeline_deferred_animated.bind(&mut render_pass);
        self.camera_uniform_buffer.bind(&mut render_pass);

        animated_object_storage.draw(&mut render_pass);
    }

    fn render_light(
        &self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        meshes: &[&dyn DeferredLightShaderDraw],
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Light Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // load: wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.02,
                        b: 0.03,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::default(),
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer_interface.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    // load: wgpu::LoadOp::Load,
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::default(),
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // lights
        self.pipeline_deferred_light.bind(&mut render_pass);
        self.camera_uniform_buffer.bind(&mut render_pass);
        self.g_buffer.bind(&mut render_pass);

        for mesh in meshes {
            mesh.draw_lights(&mut render_pass);
        }
    }

    fn render_forward(
        &self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        textured_meshes: &impl VertexTextureShaderDraw,
        performance_monitor: &impl VertexColorShaderDraw,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Forward Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::default(),
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: renderer_interface.get_depth_texture_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::default(),
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // performance monitor
        self.pipeline_lines.bind(&mut render_pass);
        self.camera_uniform_orthographic_buffer
            .bind(&mut render_pass);
        performance_monitor.draw(&mut render_pass);

        // textured meshes
        self.pipeline_texture_gui.bind(&mut render_pass);
        self.camera_uniform_orthographic_buffer
            .bind(&mut render_pass);
        textured_meshes.draw(&mut render_pass);
    }

    pub fn read_entity_index(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        y: u32,
        x: u32,
    ) -> u32 {
        {
            let buffer_slice = self.entity_buffer.map_buffer_async();

            // must wait for the device to finish before the data can be used
            renderer_interface.device().poll(wgpu::Maintain::Wait);

            buffer_slice.get(y, x)
        }
    }

    pub fn render(
        &mut self,
        renderer_interface: &mut dyn WgpuRendererInterface,
        // deferred: & impl DeferredShaderDraw,
        // deferred_light: & impl DeferredLightShaderDraw,
        deferred_combined: &(impl DeferredShaderDraw + DeferredLightShaderDraw),
        animated_object_storage: &WgpuAnimatedObjectStorage,

        mesh_textured_gui: &impl VertexTextureShaderDraw,
        performance_monitor: &mut PerformanceMonitor,
    ) -> Result<(), wgpu::SurfaceError> {
        performance_monitor.watch.start(0);
        let output = renderer_interface.get_current_texture()?;
        performance_monitor.watch.stop(0);

        performance_monitor.watch.start(1);

        let view: wgpu::TextureView = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: wgpu::CommandEncoder =
            renderer_interface
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // draw
        self.render_deferred(
            renderer_interface,
            &view,
            &mut encoder,
            &[deferred_combined],
            animated_object_storage,
        );
        self.render_light(
            renderer_interface,
            &view,
            &mut encoder,
            &[deferred_combined],
        );
        self.render_forward(
            renderer_interface,
            &view,
            &mut encoder,
            mesh_textured_gui,
            performance_monitor,
        );

        // copy entity texture
        self.entity_buffer.copy_texture_to_buffer(&mut encoder);

        renderer_interface
            .queue()
            .submit(std::iter::once(encoder.finish()));
        output.present();

        // wait to see how high the gpu load is
        renderer_interface.device().poll(wgpu::Maintain::Wait);

        performance_monitor.watch.stop(1);

        Ok(())
    }
}
