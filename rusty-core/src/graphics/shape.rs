use std::f32::consts::PI;

use super::{color, Mesh, Transform, Vertex};
use crate::{math::Rect, Ctx};
use glam::Vec2;
use wgpu::{util::DeviceExt, BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShapeVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    // pub tex_coords: [f32; 2]
}

impl Vertex for ShapeVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<ShapeVertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub trait Shape: super::Transformable {
    fn set_fill_color(&mut self, color: color::Color);

    fn point(&self, index: usize) -> Vec2;

    fn point_count(&self) -> usize;
}

pub struct RectangleShape {
    // transform: super::Transform,
    size: Vec2,
    mesh: Mesh,
    vertices: Vec<ShapeVertex>,
    color: color::Color,
    context: Ctx,
}

impl RectangleShape {
    pub fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn new(context: Ctx, size: Vec2) -> Self {
        // Generate buffer
        let vertices = vec![
            ShapeVertex {
                position: [0., 0., 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [0., size.y, 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [size.x, size.y, 0.],
                color: [1., 1., 1., 1.0],
            },
            ShapeVertex {
                position: [size.x, 0., 0.],
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

        // let transform = Transform::default();
        // let transform_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Transform buffer"),
        //     contents: bytemuck::cast_slice(&[transform.to_model_matrix()]),
        //     usage: wgpu::BufferUsages::UNIFORM
        // });
        // let transform_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     entries: &[wgpu::BindGroupLayoutEntry {
        //         binding: 0,
        //         visibility: wgpu::ShaderStages::VERTEX,
        //         ty: wgpu::BindingType::Buffer {
        //             ty: wgpu::BufferBindingType::Uniform,
        //             has_dynamic_offset: false,
        //             min_binding_size: None
        //         },
        //         count: None
        //     }],
        //     label: Some("transform bind group layout")
        // });
        // let transform_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &transform_bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: transform_buffer.as_entire_binding()
        //     }],
        //     label: Some("transform bind group")
        // });

        drop(ctx);

        // let mesh = Mesh {
        //     vertex_buffer,
        //     index_buffer,
        //     num_elements: indices.len() as u32,
        // };
        let mesh = Mesh::new(
            context.clone(),
            vertex_buffer,
            index_buffer,
            indices.len() as u32,
            // transform_buffer,
            // transform_bind_group
        );

        let rect = Self {
            // transform,
            size,
            mesh,
            // vertex_buffer,
            vertices,
            color: color::WHITE,
            context,
        };

        // rect.update();

        rect
    }

    pub fn bounds(&self) -> Rect {
        Rect {
            x: self.mesh.transform.position.x,
            y: self.mesh.transform.position.y,
            width: self.size.x,
            height: self.size.y,
        }
    }

    fn update(&mut self) {
        for i in 0..self.point_count() {
            let point = self.point(i);

            if let Some(vertex) = self.vertices.get_mut(i) {
                vertex.position = [point.x, point.y, 0.0];
            }

            // TODO : writter buffer
            let ctx = self.context.lock().unwrap();
            ctx.queue.write_buffer(
                &self.mesh.vertex_buffer,
                0,
                bytemuck::cast_slice(&self.vertices),
            );
            drop(ctx);
        }
    }

    pub fn size(&self) -> &Vec2 {
        &self.size
    }
}

impl Shape for RectangleShape {
    fn point(&self, index: usize) -> Vec2 {
        match index {
            1 => (0., self.size.x).into(),
            2 => self.size,
            3 => (self.size.x, 0.).into(),
            _ => (0., 0.).into(),
        }
    }

    fn point_count(&self) -> usize {
        self.vertices.len()
    }

    fn set_fill_color(&mut self, color: color::Color) {
        self.color = color;
        self.update();
    }
}

impl super::Transformable for RectangleShape {
    fn set_position(&mut self, position: Vec2) {
        self.mesh.transform.position = position;

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);

        self.update();
    }

    fn position(&self) -> &Vec2 {
        &self.mesh.transform.position
    }

    fn r#move(&mut self, offset: Vec2) {
        self.mesh.transform.position += offset;

        self.update();
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.mesh.transform.rotation = rotation;

        self.update();

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);
    }

    fn rotation(&self) -> f32 {
        self.mesh.transform.rotation
    }

    fn rotate(&mut self, angle: f32) {
        // let deg =
        // let radian = deg * PI / 180;

        if self.mesh.transform.rotation + angle > 360. {
            self.mesh.transform.rotation = 0.;
        } else {
            self.mesh.transform.rotation += angle;
        };

        // println!("Rotation deg: {}", self.mesh.transform.rotation * 180. / PI);
        // self.update();

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);
    }

    fn set_scale(&mut self, scale: f32) {
        self.mesh.transform.scale = scale;
        self.update();
    }

    fn scale(&self) -> f32 {
        self.mesh.transform.scale
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.mesh.transform.origin = origin;
        self.update();
    }

    fn origin(&self) -> &Vec2 {
        &self.mesh.transform.origin
    }
}

pub struct CircleShape {
    radius: f32,
    point_count: u8,
    pub mesh: Mesh,
    vertices: Vec<ShapeVertex>,
    color: color::Color,
    context: Ctx,
}

impl CircleShape {
    pub fn new(context: Ctx, radius: f32, point_count: u8) -> Self {
        let mut vertices = Vec::with_capacity(point_count as usize + 1);
        let mut indices: Vec<u16> = Vec::with_capacity(point_count as usize * 3);

        let center = [0.0, 0.0, 0.0];
        let color = [1.0, 1.0, 1.0, 1.0];

        vertices.push(ShapeVertex {
            position: center,
            color,
        });

        for i in 0..point_count {
            let theta = -2.0 * std::f32::consts::PI * i as f32 / point_count as f32;
            let x = center[0] + radius * theta.cos();
            let y = center[1] + radius * theta.sin();

            vertices.push(ShapeVertex {
                position: [x, y, 0.0],
                color,
            });

            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push((i + 1) as u16 % point_count as u16 + 1);
        }

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
            radius,
            point_count,
            mesh,
            vertices,
            color: color::WHITE,
            context,
        }
    }

    fn update(&mut self) {}
}

impl Shape for CircleShape {
    fn point(&self, index: usize) -> Vec2 {
        let angle = (index as f32 / self.point_count as f32) * 2.0 * PI - (PI / 2.0);

        Vec2 {
            x: self.radius,
            y: self.radius,
        } + Vec2 {
            x: self.radius * angle.cos(),
            y: self.radius * angle.sin(),
        }
    }

    fn point_count(&self) -> usize {
        self.vertices.len()
    }

    fn set_fill_color(&mut self, color: color::Color) {}
}

impl super::Transformable for CircleShape {
    fn set_position(&mut self, position: Vec2) {
        self.mesh.transform.position = position;

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);

        self.update();
    }

    fn position(&self) -> &Vec2 {
        &self.mesh.transform.position
    }

    fn r#move(&mut self, offset: Vec2) {
        self.mesh.transform.position += offset;

        self.update();
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.mesh.transform.rotation = rotation;

        self.update();

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);
    }

    fn rotation(&self) -> f32 {
        self.mesh.transform.rotation
    }

    fn rotate(&mut self, angle: f32) {
        // let deg =
        // let radian = deg * PI / 180;

        if self.mesh.transform.rotation + angle > 360. {
            self.mesh.transform.rotation = 0.;
        } else {
            self.mesh.transform.rotation += angle;
        };

        println!("Rotation deg: {}", self.mesh.transform.rotation * 180. / PI);
        // self.update();

        let ctx = self.context.lock().unwrap();
        ctx.queue.write_buffer(
            &self.mesh.buffer,
            0,
            bytemuck::cast_slice(&[self.mesh.transform.to_model_matrix()]),
        );
        drop(ctx);
    }

    fn set_scale(&mut self, scale: f32) {
        self.mesh.transform.scale = scale;
        self.update();
    }

    fn scale(&self) -> f32 {
        self.mesh.transform.scale
    }

    fn set_origin(&mut self, origin: Vec2) {
        self.mesh.transform.origin = origin;
        self.update();
    }

    fn origin(&self) -> &Vec2 {
        &self.mesh.transform.origin
    }
}