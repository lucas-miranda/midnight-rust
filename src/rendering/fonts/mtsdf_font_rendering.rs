use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::Path,
};

use crate::{
    math::Size2,
    resources::{AssetWeak, Asset},
};

use super::{
    mtsdf::MTSDF,
    Font,
    FontRendering,
    Glyph,
    Texture,
};

pub struct MTSDFFontRendering {
    texture: AssetWeak<Texture>,
    tex_size: Size2<u32>,
    data: MTSDF,
}

impl MTSDFFontRendering {
    pub fn load<P: AsRef<Path>>(texture: &Asset<Texture>, data_filepath: P) -> Self {
        let file = File::open(data_filepath).unwrap();
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).unwrap();


        Self {
            texture: texture.weak(),
            tex_size: texture.get().size(),
            data,
        }
    }

    pub fn data(&self) -> &MTSDF {
        &self.data
    }
}

impl FontRendering for MTSDFFontRendering {
    fn texture(&self) -> Option<AssetWeak<Texture>> {
        Some(self.texture.clone())
    }

    fn texture_size(&self) -> Option<Size2<u32>> {
        Some(self.tex_size)
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
        texture: &Asset<Texture>,
        data_filepath: P
    ) -> Font<MTSDFFontRendering> {
        Font::new(MTSDFFontRendering::load(texture, data_filepath))
    }
}
