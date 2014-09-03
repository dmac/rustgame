use rsfml::graphics::{Font, Texture, Image, IntRect};

pub struct Assets {
    pub textures: Textures,
    pub font: Font,
}

struct Textures {
    pub player: Texture,
    pub enemy: Texture,
    pub wall: Texture,
    pub sword: Texture,
}

static spritesheet_unit_size: uint = 32;

impl Assets {
    pub fn new() -> Assets {
        let ss = Image::new_from_file("resources/spritesheet.png").unwrap();
        Assets{
            textures:
            Textures{
                player: Assets::ss_texture(&ss, 0, 0, 1, 1),
                enemy: Assets::ss_texture(&ss, 1, 0, 1, 1),
                wall: Assets::ss_texture(&ss, 3, 0, 1, 1),
                sword: Assets::ss_texture(&ss, 2, 0, 1, 1),
            },
            font: Font::new_from_file("resources/Inconsolata-Regular.ttf").unwrap(),
        }
    }

    fn ss_texture(ss: &Image, row: uint, col: uint, nrows: uint, ncols: uint) -> Texture {
        let rect = IntRect::new(
            (col*spritesheet_unit_size) as i32, (row*spritesheet_unit_size) as i32,
            (ncols*spritesheet_unit_size) as i32, (nrows*spritesheet_unit_size) as i32);
        Texture::new_from_image_with_rect(ss, &rect).unwrap()
    }
}
