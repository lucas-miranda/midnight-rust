use std::{
    path::Path,
    fs::File,
    io::BufReader,
    collections::HashMap,
};

use super::{
    mtsdf::MTSDF,
    Font,
    FontRendering,
    Glyph,
    Texture,
};

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

    fn ascender(&self) -> f32 {
        self.data.metrics.ascender
    }

    fn descender(&self) -> f32 {
        self.data.metrics.descender
    }

    fn nominal_width(&self) -> f32 {
        self.data.atlas.size
    }

    fn line_height(&self) -> f32 {
        self.data.metrics.line_height
    }

    fn has_kerning(&self) -> bool {
        !self.data.kerning.is_empty()
    }

    fn kerning(&self, unicode: u32, next_unicode: u32) -> f64 {
        match self.data.kerning.get(&unicode) {
            Some(next_entries) => match next_entries.get(&next_unicode) {
                Some(kerning) => {
                    kerning.advance as f64
                },
                None => 0.0,
            },
            None => 0.0
        }
    }
}

pub struct MTSDFFont;

impl MTSDFFont {
    pub fn load<P: AsRef<Path>>(
        texture: Texture,
        data_filepath: P
    ) -> Font<MTSDFFontRendering> {
        Font::new(MTSDFFontRendering::load(texture, data_filepath))
    }
}
