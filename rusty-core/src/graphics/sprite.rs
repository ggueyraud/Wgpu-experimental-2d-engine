use wgpu::util::DeviceExt;
use glam::Vec2;

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

impl Transformable for Sprite {
    fn set_position(&mut self, position: glam::Vec2) {
        self.mesh.transform.position = position;

        // update model matrix
    }

    fn position(&self) -> &Vec2 {
        &self.mesh.transform.position
    }

    fn r#move(&mut self, offset: Vec2) {
        self.mesh.transform.position += offset;

        // self.update();
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.mesh.transform.rotation = rotation;

        // update model matrix
    }

    fn rotation(&self) -> f32 {
        self.mesh.transform.rotation
    }

    fn rotate(&mut self, angle: f32) {
        if self.mesh.transform.rotation + angle > 360. {
            self.mesh.transform.rotation = 0.;
        } else {
            self.mesh.transform.rotation += angle;
        };

        // update model matrix
    }

    fn set_scale(&mut self, scale: f32) {
        self.mesh.transform.scale = scale;

        
    }

    fn scale(&self) -> f32 {
        self.mesh.transform.scale
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.mesh.transform.origin = origin;
        // self.update();
    }

    fn origin(&self) -> &Vec2 {
        &self.mesh.transform.origin
    }
}