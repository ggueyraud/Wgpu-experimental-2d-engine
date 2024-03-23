use glam::{Mat4, Vec2};
use wgpu::{util::DeviceExt, VertexBufferLayout};

use crate::Ctx;

pub mod color;
pub mod shape;
pub mod sprite;
pub mod texture;

#[derive(Default)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32,
    pub origin: Vec2,
}

impl Transform {
    pub fn to_model_matrix(&self) -> Mat4 {
        let rotation = Mat4::from_rotation_z(self.rotation);
        let translation = Mat4::from_translation((self.position.x, self.position.y, 0.0).into());
        let origin = Mat4::from_translation((-self.origin.x, -self.origin.y, 0.).into());

        translation * rotation * origin
    }
}

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    transform: Transform,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    // texture_bind_group: wgpu::BindGroup,
    num_elements: u32,
}

impl Mesh {
    pub fn new(
        context: Ctx,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        num_elements: u32,
    ) -> Self {
        let ctx = context.lock().unwrap();
        let transform = Transform::default();
        let buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex buffer"),
                contents: bytemuck::cast_slice(&[transform.to_model_matrix()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let bind_group_layout = ctx.bind_group_layouts.get("transform").unwrap();
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("transform bind group"),
        });
        // let bind_group_layout = ctx.bind_group_layouts.get("texture").unwrap();
        // let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: buffer.as_entire_binding(),
        //     }],
        //     label: Some("transform bind group"),
        // });

        Self {
            vertex_buffer,
            index_buffer,
            num_elements,
            buffer,
            bind_group,
            transform: Default::default(),
        }
    }
}

// impl Transformable for Mesh {
//     fn set_position(&mut self, position: Vec2) {
//         self.mesh.transform.position = position;

//         self.update();
//     }

//     fn position(&self) -> &Vec2 {
//         &self.mesh.transform.position
//     }

//     fn r#move(&mut self, offset: Vec2) {
//         self.mesh.transform.position += offset;

//         self.update();
//     }

//     fn set_rotation(&mut self, rotation: f32) {
//         self.mesh.transform.rotation = rotation;

//         self.update();
//     }

//     fn rotation(&self) -> f32 {
//         self.mesh.transform.rotation
//     }

//     fn rotate(&mut self, angle: f32) {
//         self.mesh.transform.rotation += angle;
//         self.update();
//     }

//     fn set_scale(&mut self, scale: f32) {
//         self.mesh.transform.scale = scale;
//         self.update();
//     }

//     fn scale(&self) -> f32 {
//         self.mesh.transform.scale
//     }

//     fn set_origin(&mut self, origin: Vec2) {
//         self.mesh.transform.origin = origin;
//         self.update();
//     }

//     fn origin(&self) -> &Vec2 {
//         &self.mesh.transform.origin
//     }
// }

pub trait Transformable {
    fn set_position(&mut self, position: Vec2);
    fn position(&self) -> &Vec2;
    fn r#move(&mut self, offset: Vec2);

    fn set_rotation(&mut self, rotation: f32);
    fn rotation(&self) -> f32;
    fn rotate(&mut self, angle: f32);

    fn set_scale(&mut self, scale: f32);
    fn scale(&self) -> f32;

    fn set_origin(&mut self, origin: Vec2);
    fn origin(&self) -> &Vec2;
}

pub trait Vertex {
    fn desc() -> VertexBufferLayout<'static>;
}

pub trait Drawable<'a> {
    fn draw_mesh(&mut self, vertex_buffer: &'a Mesh);
}

impl<'a, 'b> Drawable<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(&mut self, mesh: &'b Mesh) {
        self.set_bind_group(3, &mesh.bind_group, &[]);
        self.set_bind_group(3, &mesh.bind_group, &[]);
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }
}
