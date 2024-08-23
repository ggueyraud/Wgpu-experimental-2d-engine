use std::{collections::HashMap, sync::OnceLock};

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "graphics")]
pub mod graphics;

#[cfg(feature = "ui")]
pub mod ui;

pub mod math;

pub use glam;
pub use wgpu;
pub use winit;

pub static mut GL_CONTEXT: OnceLock<Context> = OnceLock::new();

#[derive(Debug)]
pub struct Context {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipelines: HashMap<String, wgpu::RenderPipeline>,
    pub bind_group_layouts: HashMap<String, wgpu::BindGroupLayout>,
}

impl Context {
    pub fn init(device: wgpu::Device, queue: wgpu::Queue, config: wgpu::SurfaceConfiguration) {
        unsafe {
            GL_CONTEXT
                .set(Self {
                    device,
                    queue,
                    config,
                    render_pipelines: HashMap::new(),
                    bind_group_layouts: HashMap::new(),
                })
                .unwrap()
        }
    }

    pub fn get() -> &'static Self {
        unsafe { GL_CONTEXT.get().unwrap() }
    }

    pub fn get_mut() -> &'static mut Self {
        unsafe { GL_CONTEXT.get_mut().unwrap() }
    }
}
