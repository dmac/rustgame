use std::collections::HashMap;
use rsfml::graphics::{Font, Texture};

pub struct Assets {
    textures: HashMap<&'static str, Texture>,
    font: Font,
}

impl Assets {
    pub fn new() -> Assets {
        let mut textures: HashMap<&str, Texture> = HashMap::new();
        textures.insert("player", Texture::new_from_file("resources/link.gif").unwrap());
        textures.insert("moblin", Texture::new_from_file("resources/moblin.gif").unwrap());
        textures.insert("wall", Texture::new_from_file("resources/block.gif").unwrap());
        textures.insert("sword", Texture::new_from_file("resources/sword.gif").unwrap());

        let font = Font::new_from_file("resources/Inconsolata-Regular.ttf").unwrap();
        Assets{
            textures: textures,
            font: font,
        }
    }

    pub fn get_texture(&self, s: &'static str) -> &Texture {
        &self.textures[s]
    }

    pub fn get_font(&self) -> &Font {
        &self.font
    }
}
