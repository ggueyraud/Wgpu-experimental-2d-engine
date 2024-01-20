use winit::event::WindowEvent;

pub mod button;
pub mod label;

pub trait Widget {
    fn process_events(&mut self, event: &WindowEvent);
    fn update(&mut self);
}
