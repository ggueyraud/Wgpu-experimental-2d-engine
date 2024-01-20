use crate::graphics::color;

#[derive(Default)]
pub enum Appearence {
    #[default]
    Regular,
    Bold,
    Italic,
    Underlined,
    StrikeThrough,
}

pub struct Style {
    pub font_size: f32,
    pub color: color::Color,
    pub appearence: Appearence,
    // Font
    pub letter_spacing: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            color: color::WHITE,
            appearence: Appearence::default(),
            letter_spacing: 0.0,
        }
    }
}

struct Label {
    style: Style,
    text: String,
}

impl Label {
    pub fn new(text: &str, style: Style) -> Self {
        Self {
            style,
            text: text.to_owned(),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
