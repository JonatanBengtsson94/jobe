use std::{env::current_dir, fs};

use image::GenericImageView;

use super::bind_group;

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        filename: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let mut filepath = current_dir().expect("Could not find current directory.");
        filepath.push(filename);
        let filepath = filepath
            .into_os_string()
            .into_string()
            .expect("Could not get filepath.");
        let bytes = fs::read(filepath).expect("Could not read file.");
        let loaded_image = image::load_from_memory(&bytes).expect("Could not load image.");
        let converted = loaded_image.to_rgba8();
        let (width, height) = loaded_image.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = wgpu::TextureDescriptor {
            label: Some(filename),
            mip_level_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            size: texture_size,
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };
        let texture = device.create_texture(&texture_descriptor);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &converted,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler_descriptor = wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            min_filter: wgpu::FilterMode::Nearest,
            mag_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };
        let sampler = device.create_sampler(&sampler_descriptor);

        let mut bind_group_builder = bind_group::Builder::new(device, layout);
        bind_group_builder.add_material(&view, &sampler);
        let bind_group = bind_group_builder.build(&filename);

        Material { bind_group }
    }
}
