use winit::event::Event;
use std::collections::HashMap;

pub trait Scene {
    fn update(&self);
    fn process_event(&mut self, event: &Event);
    fn draw(&self);
}

enum Action {
    Push,
    Pop,
    Clear
}

struct PendingChange {
    action: Action,
    id: usize
}

pub struct Manager<F>
    where F: Fn() {
    scenes: Vec<Box<dyn Scene>>,
    pending_list: Vec<PendingChange>,
    factories: HashMap<String, F>
}

impl Manager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new()
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        for scene in &mut self.scenes {
            scene.process_event(event);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for scene in self.scenes {
            scene.update(dt);
        }
    }

    pub fn draw(&self) {
        for scene in &self.scenes {
            scene.draw();
        }
    }

    fn apply_pending_changes(&mut self) {
        for change in self.pending_list {
            match change.action {
                Action::Push => {
                    // self.scenes.push(change.)
                },
                Action::Pop => {
                    self.scenes.pop();
                },
                Action::Clear => {
                    self.scenes.clear();
                }
            }
        }
    }

    fn create_scene() {

    }
}