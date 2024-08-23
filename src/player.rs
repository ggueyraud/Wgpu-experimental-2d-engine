use rusty_core::graphics::shape::Shape;
use rusty_core::graphics::Transformable;
use rusty_core::winit::keyboard::KeyCode;
use rusty_core::winit::{event::WindowEvent, keyboard::PhysicalKey};
use rusty_core::{
    graphics::{shape::RectangleShape, texture::Texture},
    math::Rect,
};
use rusty_engine::animation::Animation;
use std::{collections::HashMap, rc::Rc};

const SPEED: f32 = 200.;

#[derive(Default)]
pub enum Direction {
    Left,
    Down,
    Right,
    Up,
    #[default]
    None,
}

pub struct Player {
    pub rect: RectangleShape,
    animations: HashMap<String, Animation>,
    current_animation: String,
    pub texture: Rc<Texture>,
    direction: Direction,
}

impl Player {
    pub fn new(texture: Rc<Texture>) -> Self {
        let width = 33.;
        let height = 36.;
        let rect = RectangleShape::new((width, height).into());
        let mut animations = HashMap::new();
        let current_animation = String::from("down");

        animations.insert(
            current_animation.clone(),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y: 0.,
                        width,
                        height,
                    },
                    Rect {
                        x: 33.,
                        y: 0.,
                        width,
                        height,
                    },
                    Rect {
                        x: 66.,
                        y: 0.,
                        width,
                        height,
                    },
                    Rect {
                        x: 99.,
                        y: 0.,
                        width,
                        height,
                    },
                ],
                0.20,
            ),
        );
        animations.insert(
            "left".to_string(),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y: 36.,
                        width,
                        height,
                    },
                    Rect {
                        x: 33.,
                        y: 36.,
                        width,
                        height,
                    },
                    Rect {
                        x: 66.,
                        y: 36.,
                        width,
                        height,
                    },
                    Rect {
                        x: 99.,
                        y: 36.,
                        width,
                        height,
                    },
                ],
                0.20,
            ),
        );
        animations.insert(
            "up".to_string(),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y: 108.,
                        width,
                        height,
                    },
                    Rect {
                        x: 33.,
                        y: 108.,
                        width,
                        height,
                    },
                    Rect {
                        x: 66.,
                        y: 108.,
                        width,
                        height,
                    },
                    Rect {
                        x: 99.,
                        y: 108.,
                        width,
                        height,
                    },
                ],
                0.20,
            ),
        );
        animations.insert(
            "right".to_string(),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y: 72.,
                        width,
                        height,
                    },
                    Rect {
                        x: 33.,
                        y: 72.,
                        width,
                        height,
                    },
                    Rect {
                        x: 66.,
                        y: 72.,
                        width,
                        height,
                    },
                    Rect {
                        x: 99.,
                        y: 72.,
                        width,
                        height,
                    },
                ],
                0.20,
            ),
        );

        Self {
            rect,
            animations,
            texture,
            current_animation,
            direction: Direction::None,
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { event, .. } = event {
            // match self.direction {
            //     Direction::Left | Direction::Down | Direction::Right | Direction::Up => {
            //         if !event.state.is_pressed() {
            //             if let Some(animation) =
            //                 self.animations.get_mut(&self.current_animation)
            //             {
            //                 animation.reset();
            //                 self.direction = Direction::None;
            //             }
            //         }
            //     }
            //     _ => {}
            // }

            let (animation, direction) = match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowUp) => ("up", Direction::Up),
                PhysicalKey::Code(KeyCode::ArrowLeft) => ("left", Direction::Left),
                PhysicalKey::Code(KeyCode::ArrowRight) => ("right", Direction::Right),
                PhysicalKey::Code(KeyCode::ArrowDown) => ("down", Direction::Down),
                _ => {
                    return;
                }
            };

            if !event.state.is_pressed() {
                if let Some(animation) = self.animations.get_mut(&self.current_animation) {
                    animation.reset();
                    self.direction = Direction::None;
                    return;
                }
            }

            self.current_animation = animation.to_string();
            self.direction = direction;
        }
    }

    pub fn update(&mut self, dt: f32) {
        let offset = match self.direction {
            Direction::Left => (-SPEED, 0.),
            Direction::Down => (0., SPEED),
            Direction::Right => (SPEED, 0.),
            Direction::Up => (0., -SPEED),
            _ => {
                return;
            }
        };
        let offset = (offset.0 * dt, offset.1 * dt);

        if let Some(animation) = self.animations.get_mut(&self.current_animation) {
            if animation.update(dt) {
                println!("Should update animation");
                if let Some(frame) = animation.get_frame() {
                    self.rect.set_texture_rect(*frame);
                }
            }
        }

        self.rect.r#move(offset.into());
    }
}
