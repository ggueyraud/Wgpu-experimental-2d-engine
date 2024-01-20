use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[cfg(feature = "graphics")]
pub mod graphics;

#[cfg(feature = "ui")]
pub mod ui;

pub mod math;

pub use glam;
pub use wgpu;

#[derive(Debug)]
pub struct Context {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    pub bind_group_layouts: HashMap<String, wgpu::BindGroupLayout>,
}

pub type Ctx = Arc<Mutex<Context>>;