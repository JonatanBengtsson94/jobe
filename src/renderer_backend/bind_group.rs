pub struct Builder<'device, 'a> {
    entries: Vec<wgpu::BindGroupEntry<'a>>,
    layout: &'a wgpu::BindGroupLayout,
    device: &'device wgpu::Device,
}

impl<'device, 'a> Builder<'device, 'a> {
    pub fn new(device: &'device wgpu::Device, layout: &'a wgpu::BindGroupLayout) -> Self {
        Builder {
            entries: Vec::new(),
            device,
            layout,
        }
    }

    pub fn add_material(&mut self, view: &'a wgpu::TextureView, sampler: &'a wgpu::Sampler) {
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::TextureView(view),
        });

        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Sampler(sampler),
        });
    }

    pub fn build(&mut self, label: &str) -> wgpu::BindGroup {
        let layout = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: self.layout,
            entries: &self.entries,
        });

        self.entries.clear();

        return layout;
    }
}
