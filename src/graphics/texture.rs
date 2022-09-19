use std::{collections::HashMap, path::Path};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a> {
    texture_map: HashMap<String, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> TextureManager<'a> {
        TextureManager {
            texture_map: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        name: &dyn ToString,
        path: &Path,
    ) {
        match texture_creator.load_texture(path) {
            Ok(it) => {
                self.texture_map.insert(name.to_string(), it);
            }
            _ => {
                println!("Cannot found file {:?}", path);
            }
        };
    }

    pub fn get_texture(&self, name: &dyn ToString) -> Option<&Texture<'a>> {
        self.texture_map.get(&name.to_string())
    }
}
