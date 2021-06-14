use std::{collections::HashMap, path::Path};

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager<'a, 'b> {
    texture_map: HashMap<&'a str, Texture<'b>>,
}

impl<'a, 'b> TextureManager<'a, 'b> {
    pub fn new() -> TextureManager<'a, 'b> {
        TextureManager {
            texture_map: HashMap::new(),
        }
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &'b TextureCreator<WindowContext>,
        name: &'a str,
        path: &Path,
    ) {
        match texture_creator.load_texture(path) {
            Ok(it) => {
                println!("{}", name);
                self.texture_map.insert(name, it);
            }
            _ => {
                println!("Cannot found file {:?}", path);
            }
        };
    }

    pub fn get_texture(&self, name: &'a str) -> Option<&Texture<'b>> {
        self.texture_map.get(name)
    }
}
