use rusty_core::{graphics::texture::Texture, Ctx};
use std::{collections::HashMap, rc::Rc};

pub struct AssetManager {
    textures: HashMap<String, Rc<Texture>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<Rc<Texture>> {
        self.textures.get(name).cloned()
    }

    // pub fn load_texture(&mut self, ctx: Ctx) ->
}
