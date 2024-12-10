use std::{env::current_dir, fs};

pub struct Builder<'device> {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    bind_group_layouts: Vec<&'device wgpu::BindGroupLayout>,
    device: &'device wgpu::Device,
}

impl<'device> Builder<'device> {
    pub fn new(
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
        pixel_format: wgpu::TextureFormat,
        device: &'device wgpu::Device,
    ) -> Self {
        Builder {
            shader_filename: shader_filename.to_string(),
            vertex_entry: vertex_entry.to_string(),
            fragment_entry: fragment_entry.to_string(),
            pixel_format,
            vertex_buffer_layouts: Vec::new(),
            bind_group_layouts: Vec::new(),
            device,
        }
    }

    pub fn add_vertex_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout);
    }

    pub fn add_bind_group_layout(&mut self, layout: &'device wgpu::BindGroupLayout) {
        self.bind_group_layouts.push(layout);
    }

    pub fn build_pipeline(&mut self, label: &str) -> wgpu::RenderPipeline {
        let mut filepath = current_dir().expect("Could not find current directory.");
        filepath.push("shaders");
        filepath.push(&self.shader_filename);
        let filepath = filepath
            .into_os_string()
            .into_string()
            .expect("Unicode conversion failure");
        println!("{}", filepath);
        let source_code = fs::read_to_string(filepath).expect("Could not read source code.");

        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Shader module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = self.device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &self.bind_group_layouts,
            push_constant_ranges: &[],
        };
        let render_pipeline_layout = self
            .device
            .create_pipeline_layout(&pipeline_layout_descriptor);

        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: Default::default(),
                buffers: &self.vertex_buffer_layouts,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: Default::default(),
                targets: &render_targets,
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        };

        let render_pipeline = self
            .device
            .create_render_pipeline(&render_pipeline_descriptor);

        self.vertex_buffer_layouts.clear();
        self.bind_group_layouts.clear();

        return render_pipeline;
    }
}
