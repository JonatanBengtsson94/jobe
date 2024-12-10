use crate::renderer_backend::material::Material;
use crate::renderer_backend::mesh_builder::Mesh;
use crate::renderer_backend::pipeline;
use crate::renderer_backend::{bind_group_layout, mesh_builder};
use std::sync::Arc;
use wgpu::util::RenderEncoder;
use winit::window::Window;

pub struct Context<'window> {
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    triangle_mesh: wgpu::Buffer,
    quad_mesh: Mesh,
    triangle_material: Material,
}

impl<'window> Context<'window> {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("Failed to create surface");
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance
            .request_adapter(&adapter_descriptor)
            .await
            .expect("Failed to find adapter");
        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("Failed to get device");
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("Surface not supported by adapter");
        surface.configure(&device, &surface_config);

        let triangle_mesh = mesh_builder::make_triangle(&device);
        let quad_mesh = mesh_builder::make_quad(&device);

        let mut bind_group_layout_builder = bind_group_layout::Builder::new(&device);
        bind_group_layout_builder.add_material();
        let material_bind_group_layout =
            bind_group_layout_builder.build("Material Bind Group Layout");

        let mut pipeline_builder = pipeline::Builder::new(
            "shader.wgsl",
            "vs_main",
            "fs_main",
            surface_config.format,
            &device,
        );
        pipeline_builder.add_vertex_buffer_layout(mesh_builder::Vertex::get_layout());
        pipeline_builder.add_bind_group_layout(&material_bind_group_layout);
        let render_pipeline = pipeline_builder.build_pipeline("render_pipeline");

        let triangle_material = Material::new(
            "assets/racket.png",
            &device,
            &queue,
            &material_bind_group_layout,
        );

        Self {
            surface,
            adapter,
            device,
            queue,
            surface_config,
            render_pipeline,
            triangle_mesh,
            quad_mesh,
            triangle_material,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = surface_texture.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Encoder"),
        };
        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut renderpass = command_encoder.begin_render_pass(&render_pass_descriptor);
            renderpass.set_pipeline(&self.render_pipeline);

            /*
            renderpass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
            renderpass.set_index_buffer(
                self.quad_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            renderpass.draw_indexed(0..6, 0, 0..1);
            */

            renderpass.set_bind_group(0, &self.triangle_material.bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            renderpass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(command_encoder.finish()));

        surface_texture.present();
        Ok(())
    }
}
