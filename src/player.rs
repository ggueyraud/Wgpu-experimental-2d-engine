use rusty_core::{
    graphics::{shape::RectangleShape, texture::Texture},
    math::Rect,
    Ctx,
};
use rusty_engine::animation::Animation;
use std::{collections::HashMap, rc::Rc};

pub struct Player {
    rect: RectangleShape,
    animations: HashMap<String, Animation>,
    current_animation: String,
    texture: Rc<Texture>,
}

impl Player {
    pub fn new(context: Ctx, texture: Rc<Texture>) -> Self {
        let rect = RectangleShape::new(context, (32., 32.).into());
        let mut animations = HashMap::new();
        let width = 33.;
        let height = 36.;

        animations.insert(
            String::from("down"),
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

        Self {
            rect,
            animations,
            texture,
            current_animation: String::new(),
        }
    }

    pub fn process_event(&mut self) {}
}
