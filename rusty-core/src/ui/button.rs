use crate::Ctx;
use crate::graphics::shape::RectangleShape;

struct Button {
    shape: RectangleShape, // text:
}

impl Button {
    pub fn new(context: Ctx, text: &str) -> Self {
        Self {
            shape: RectangleShape::new(context.clone(), (0., 0.).into()),
        }
    }
}
