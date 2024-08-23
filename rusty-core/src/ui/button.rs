use crate::graphics::shape::RectangleShape;

struct Button {
    shape: RectangleShape, // text:
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            shape: RectangleShape::new((0., 0.).into()),
        }
    }
}
