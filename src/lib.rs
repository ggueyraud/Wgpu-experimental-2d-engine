use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

// use graphics::{Transformable, Vertex};
use rusty_core::{
    glam::{f32::Mat4, Vec2},
    graphics::{
        shape::{CircleShape, RectangleShape, ShapeVertex},
        Transformable, Vertex, texture::Texture,
    },
    wgpu, Context, Ctx,
};
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

// mod graphics;
// mod math;
// mod ui;

// #[derive(Debug)]
// pub struct Context {
//     pub device: wgpu::Device,
//     pub queue: wgpu::Queue,
//     pub config: wgpu::SurfaceConfiguration,
//     pub render_pipelines: HashMap<String, wgpu::RenderPipeline>,
//     pub bind_group_layouts: HashMap<String, wgpu::BindGroupLayout>,
// }

// pub type Ctx = Arc<Mutex<Context>>;

struct State {
    surface: wgpu::Surface,
    context: Ctx,
    window: Window,
    render_pipeline: wgpu::RenderPipeline,
    mouse_buffer: wgpu::Buffer,
    mouse_position: Vec2,
    mouse_bind_group: wgpu::BindGroup,
    resolution_buffer: wgpu::Buffer,
    resolution_bind_group: wgpu::BindGroup,
    projection_buffer: wgpu::Buffer,
    projection_bind_group: wgpu::BindGroup,
    rect: RectangleShape,
    rect2: RectangleShape,
    circ: CircleShape,
    rotation: f32,
    // texture: Texture
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Texture uniform
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Uniform mouse
        let mouse_position = Vec2::default();
        let mouse_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mouse buffer"),
            contents: bytemuck::cast_slice(&[mouse_position]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let mouse_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("mouse bind group layout"),
            });
        let mouse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &mouse_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mouse_buffer.as_entire_binding(),
            }],
            label: Some("mouse bind group"),
        });

        // Uniform resolution
        let resolution = Vec2 {
            x: size.width as f32,
            y: size.height as f32,
        };
        let resolution_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("resolution buffer"),
            contents: bytemuck::cast_slice(&[resolution]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let resolution_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("mouse bind group layout"),
            });
        let resolution_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &resolution_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: resolution_buffer.as_entire_binding(),
            }],
            label: Some("resolution bind group"),
        });

        // Projection uniform
        // let projection = glam::f32::Mat4::orthographic_rh(0., size.width as f32, 400., 0., 0., 1.);
        let projection =
            Mat4::orthographic_rh(0.0, size.width as f32, size.height as f32, 0.0, -1.0, 0.0);
        let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("projection buffer"),
            contents: bytemuck::cast_slice(&[projection]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let projection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("projection bind group layout"),
            });
        let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: projection_buffer.as_entire_binding(),
            }],
            label: Some("projection bind group"),
        });

        let mut bind_group_layouts = HashMap::new();
        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("transform bind group layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[
                    &mouse_bind_group_layout,
                    &resolution_bind_group_layout,
                    &projection_bind_group_layout,
                    &transform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        bind_group_layouts.insert("transform".to_string(), transform_bind_group_layout);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ShapeVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let context = Arc::new(Mutex::new(Context {
            config,
            device,
            queue,
            render_pipelines: HashMap::new(),
            bind_group_layouts,
        }));
        let mut rect = RectangleShape::new(context.clone(), (100., 100.).into());
        rect.set_position((200., 200.).into());
        rect.set_origin((50., 50.).into());

        let rect2 = RectangleShape::new(context.clone(), (32., 32.).into());
        // rect.set_position((position))

        let mut circ = CircleShape::new(context.clone(), 50., 30);
        circ.set_position((300., 300.).into());

        // let player_bytes = include_bytes!("../assets/spritesheets/player.png");
        // let texture = Texture::from_bytes(context.clone(), player_bytes, "player").unwrap();

        Self {
            surface,
            context,
            window,
            render_pipeline,
            mouse_position,
            mouse_buffer,
            mouse_bind_group,
            resolution_buffer,
            resolution_bind_group,
            projection_buffer,
            projection_bind_group,
            rect,
            rect2,
            circ,
            rotation: 0.,
            // texture
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32).into();
                let context = self.context.lock().unwrap();
                context.queue.write_buffer(
                    &self.mouse_buffer,
                    0,
                    bytemuck::cast_slice(&[self.mouse_position]),
                );

                true
            }
            _ => false,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let mut context = self.context.lock().unwrap();
            context.config.width = new_size.width;
            context.config.height = new_size.height;

            self.surface.configure(&context.device, &context.config);
        }
    }

    fn update(&mut self, dt: f32) {
        // self.rotation += dt * 360.;
        // println!("{:?}", dt * 360.);
        self.rect.rotate(dt * 1.);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let context = self.context.lock().unwrap();
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.mouse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.resolution_bind_group, &[]);
            render_pass.set_bind_group(2, &self.projection_bind_group, &[]);

            use rusty_core::graphics::Drawable;
            let rect_mesh = self.rect.mesh();
            render_pass.draw_mesh(rect_mesh);
            // render_pass.draw_mesh(&self.rect2.mesh);
            render_pass.draw_mesh(&self.circ.mesh);
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut last_frame_time = Instant::now();
    let mut state = State::new(window).await;

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == winit::event::ElementState::Pressed
                            && event.physical_key
                                == winit::keyboard::PhysicalKey::Code(
                                    winit::keyboard::KeyCode::Escape,
                                )
                        {
                            elwt.exit()
                        }
                    }
                    WindowEvent::Resized(new_physical_size) => state.resize(*new_physical_size),
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
                        last_frame_time = now;

                        state.update(delta_time);

                        match state.render() {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    _ => {}
                }
            }
        }
        Event::AboutToWait => {
            state.window().request_redraw();
        }
        _ => {}
    });
}
