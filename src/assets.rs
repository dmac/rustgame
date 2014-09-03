use rsfml::graphics::{Font, Texture};

pub struct Assets {
    pub textures: Textures,
    pub font: Font,
}

struct Textures {
    pub player: Texture,
    pub moblin: Texture,
    pub wall: Texture,
    pub sword: Texture,
}

impl Assets {
    pub fn new() -> Assets {
        Assets{
            textures: Textures{
                          player: Texture::new_from_file("resources/link.gif").unwrap(),
                          moblin: Texture::new_from_file("resources/moblin.gif").unwrap(),
                          wall: Texture::new_from_file("resources/block.gif").unwrap(),
                          sword: Texture::new_from_file("resources/sword.gif").unwrap(),
                      },
                      font: Font::new_from_file("resources/Inconsolata-Regular.ttf").unwrap(),
        }
    }
}
