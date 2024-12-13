use std::mem;
use std::slice;

use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    texture_coords: [f32; 2],
}

pub struct Quad {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

impl Quad {
    pub fn new(device: &wgpu::Device) -> Quad {
        let vertices = [
            Vertex {
                position: [-1.0, -1.0],
                texture_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
                texture_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
                texture_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                texture_coords: [0.0, 0.0],
            },
        ];
        let mut bytes = any_as_u8_slice(&vertices);

        let mut buffer_descriptor = wgpu::util::BufferInitDescriptor {
            label: Some("Quad vertex buffer"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        };
        let vertex_buffer = device.create_buffer_init(&buffer_descriptor);

        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
        bytes = any_as_u8_slice(&indices);

        buffer_descriptor = wgpu::util::BufferInitDescriptor {
            label: Some("Quad index buffer"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        };
        let index_buffer = device.create_buffer_init(&buffer_descriptor);

        Quad {
            vertex_buffer,
            index_buffer,
        }
    }
}

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { slice::from_raw_parts((p as *const T) as *const u8, mem::size_of::<T>()) }
}
