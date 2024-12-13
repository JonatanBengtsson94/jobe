use crate::renderer_backend::bind_group_layout;
use crate::renderer_backend::mesh::Vertex;
use crate::renderer_backend::pipeline;
use std::sync::Arc;
use winit::window::Window;

pub struct Context<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl<'a> Context<'a> {
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
        pipeline_builder.add_vertex_buffer_layout(Vertex::get_layout());
        pipeline_builder.add_bind_group_layout(&material_bind_group_layout);
        let render_pipeline = pipeline_builder.build_pipeline("render_pipeline");

        Self {
            surface,
            device,
            queue,
            render_pipeline,
        }
    }
}
