use std::mem;
use std::slice;

use wgpu::util::DeviceExt;

#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices = [
        Vertex {
            position: [-0.75, -0.75],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.75, -0.75],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.75],
            color: [0.0, 0.0, 1.0],
        },
    ];
    let bytes = vertex_array_to_byte_slice(&vertices);

    let buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: &bytes,
        usage: wgpu::BufferUsages::VERTEX,
    };
    let buffer = device.create_buffer_init(&buffer_descriptor);
    buffer
}

fn vertex_array_to_byte_slice(vertices: &[Vertex]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * mem::size_of::<Vertex>(),
        )
    }
}
