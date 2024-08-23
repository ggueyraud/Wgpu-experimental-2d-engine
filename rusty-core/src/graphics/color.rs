use derive_more::From;

/// Describe color as RGB format
#[derive(From, Clone, Copy, Debug)]
pub struct Color(u8, u8, u8);

#[allow(dead_code)]
pub const BLACK: Color = Color(0, 0, 0);
#[allow(dead_code)]
pub const WHITE: Color = Color(255, 255, 255);
#[allow(dead_code)]
pub const RED: Color = Color(255, 0, 0);
#[allow(dead_code)]
pub const GREEN: Color = Color(0, 255, 0);
#[allow(dead_code)]
pub const BLUE: Color = Color(0, 0, 255);

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            (self.0 / 255) as f32,
            (self.1 / 255) as f32,
            (self.2 / 255) as f32,
            1.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn into_accepted_wgsl_color_format() {
        let color: [f32; 4] = Color(255, 255, 255).into();
        assert_eq!(color, [1., 1., 1., 1.0]);
        let color: [f32; 4] = Color(155, 155, 155).into();
        assert_eq!(color, [0., 0., 0., 1.]);
    }
}
