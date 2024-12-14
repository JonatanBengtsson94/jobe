use crate::context::Context;

use crate::ecs::component::signatures::{SPRITE, TRANSFORM};
use crate::ecs::components::Sprite;
use crate::ecs::entity::MAX_ENTITIES;
use crate::ecs::Signature;

pub struct Render;

impl Render {
    pub const SIGNATURE: Signature = TRANSFORM | SPRITE;

    pub fn render(
        context: &Context,
        sprites: &Vec<Option<Sprite>>,
        entity_signatures: &[Signature; MAX_ENTITIES],
    ) -> Result<(), wgpu::SurfaceError> {
        let current_frame = context.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = current_frame.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Encoder"),
        };
        let mut command_encoder = context
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
            renderpass.set_pipeline(&context.render_pipeline);

            for (index, signature) in entity_signatures.iter().enumerate() {
                if (*signature & Render::SIGNATURE) == Render::SIGNATURE {
                    if let Some(sprite) = &sprites[index] {
                        renderpass.set_bind_group(0, &sprite.material.bind_group, &[]);
                        renderpass.set_vertex_buffer(0, sprite.quad.vertex_buffer.slice(..));
                        renderpass.set_index_buffer(
                            sprite.quad.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        renderpass.draw_indexed(0..6, 0, 0..1);
                    }
                }
            }
        }

        context.queue.submit(Some(command_encoder.finish()));

        current_frame.present();
        Ok(())
    }
}
