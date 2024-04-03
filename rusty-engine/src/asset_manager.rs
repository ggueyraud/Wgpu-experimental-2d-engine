use anyhow::Result;
use rusty_core::{graphics::texture::Texture, Ctx};
use std::{collections::HashMap, rc::Rc};

pub struct AssetManager {
    // textures: HashMap<String, Texture>,
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
    // pub fn get_texture(&self, name: &str) -> Option<&Texture> {
    //     self.textures.get(name)
    // }

    pub fn load_texture(&mut self, ctx: Ctx, path: &std::path::Path) -> Result<Rc<Texture>> {
        let texture = Texture::from_path(ctx, path, None)?;
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let texture = self.textures.insert(name.clone(), Rc::new(texture));
        let texture = self.get_texture(&name);

        Ok(texture.unwrap())
    }
    // pub fn load_texture(&mut self, ctx: Ctx, path: &std::path::Path) -> Result<Option<&Texture>> {
    //     let texture = Texture::from_path(ctx, path, None)?;
    //     let name = path.file_name().unwrap().to_string_lossy().to_string();
    //     let _ = self.textures.insert(name.clone(), texture);

    //     Ok(self.get_texture(&name))
    // }
}
