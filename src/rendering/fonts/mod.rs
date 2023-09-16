pub mod mtsdf;

mod glyph;
use glyph::Glyph;

use std::{path::Path, fs::File, io::BufReader, collections::HashMap};
use self::mtsdf::MTSDF;

use super::Texture;

pub struct Font<R: FontRendering> {
    pub rendering: R,
    pub glyphs: HashMap<u32, Glyph>,

    pub size: f32,
}

impl<R: FontRendering> Font<R> {
    pub fn new(rendering: R) -> Self {
        let glyphs = rendering.glyphs();

        Self {
            rendering,
            glyphs,
            size: 12.0,
        }
    }

    pub fn glyph(&self, unicode: u32) -> Option<&Glyph> {
        self.glyphs.get(&unicode)
    }
}

pub trait FontRendering {
    fn texture<'t>(&'t self) -> Option<&'t Texture>;
    fn glyphs(&self) -> HashMap<u32, Glyph>;
}

pub struct MTSDFFontRendering {
    texture: Texture,
    data: MTSDF,
}

impl MTSDFFontRendering {
    pub fn load<P: AsRef<Path>>(texture: Texture, data_filepath: P) -> Self {
        let file = File::open(data_filepath).unwrap();
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).unwrap();

        Self {
            texture,
            data,
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn data(&self) -> &MTSDF {
        &self.data
    }
}

impl FontRendering for MTSDFFontRendering {
    fn texture<'t>(&'t self) -> Option<&'t Texture> {
        Some(&self.texture)
    }

    fn glyphs(&self) -> HashMap<u32, Glyph> {
        self.data
            .glyphs
            .iter()
            .map(|(char_code, glyph_data)| (*char_code, (*glyph_data).into()))
            .collect::<HashMap<_, _>>()
    }
}
