use wgpu::util::DeviceExt;

use crate::{math::Rect, Ctx};

use super::{color, shape::ShapeVertex, texture::Texture, Mesh, Transformable, Vertex};

pub struct Sprite {
    mesh: Mesh,
    vertices: Vec<ShapeVertex>,
    color: Option<color::Color>,
    texture: Texture,
    context: Ctx,
    texture_rect: Rect,
}

impl Sprite {
    pub fn new(context: Ctx, texture: Texture) -> Self {
        // Generating the mesh
        let texture_rect = Rect {
            x: 0.,
            y: 0.,
            width: texture.texture.size().width as f32,
            height: texture.texture.size().height as f32,
        };

        let vertices = vec![
            ShapeVertex {
                position: [0., 0., 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [0., texture_rect.height, 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [texture_rect.width, texture_rect.height, 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [texture_rect.width, 0., 0.],
                color: [1., 1., 1., 1.0],
            },
        ];
        let indices: Vec<u16> = vec![0, 1, 3, 1, 2, 3];
        let ctx = context.lock().unwrap();
        let vertex_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        let index_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        drop(ctx);

        let mesh = Mesh::new(
            context.clone(),
            vertex_buffer,
            index_buffer,
            indices.len() as u32,
        );

        Self {
            color: None,
            texture,
            context,
            vertices,
            mesh,
            texture_rect,
        }
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }
}

// impl Transformable for Sprite {
//     fn set_position(&mut self, position: glam::Vec2) {
//         self.mesh.transform.position = position;
//     }
// }