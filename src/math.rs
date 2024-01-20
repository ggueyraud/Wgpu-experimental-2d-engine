use glam::Vec2;

#[derive(Copy, Clone, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Get the position of the rectangle's top-left corner.
    pub fn position(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Check if a point is contained by `Rect`
    ///
    ///  # Arguments
    ///
    /// * `point` - The point to test
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use glam::Vec2;

    #[test]
    fn rect_contains_point() {
        let rect = Rect {
            x: 0.,
            y: 0.,
            width: 32.,
            height: 32.,
        };
        assert!(rect.contains(Vec2 { x: 10., y: 10. }))
    }

    #[test]
    fn rect_contains_limits() {
        let rect = Rect {
            x: 32.,
            y: 32.,
            width: 32.,
            height: 32.,
        };
        assert!(rect.contains(Vec2 { x: 32., y: 32. }));

        assert!(rect.contains(Vec2 { x: 64., y: 64. }));
    }

    #[test]
    fn rect_position() {
        let rect = Rect {
            x: 32.,
            y: 32.,
            width: 32.,
            height: 32.,
        };
        assert_eq!(rect.position(), Vec2 { x: 32., y: 32. });
        let rect = Rect {
            x: 300.,
            y: 32.,
            width: 32.,
            height: 32.,
        };
        assert_eq!(rect.position(), Vec2 { x: 300., y: 32. })
    }
}
