pub mod mtsdf;

mod glyph;
pub(super) use glyph::Glyph;

mod font_rendering;
pub use font_rendering::FontRendering;

mod mtsdf_font_rendering;
pub use mtsdf_font_rendering::*;

mod text_render_data;
pub use text_render_data::*;

use std::{
    collections::HashMap,
    fmt::Display,
};

use unicode_segmentation::UnicodeSegmentation;
use crate::math::{Size, Vector2};

use super::Texture;

const TABULATION_WHITESPACE_AMOUNT: u32 = 4;
const WHITESPACE_GRAPHEME: Grapheme = Grapheme::Indirect(" ");

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
            size: 32.0,
        }
    }

    pub fn glyph(&self, unicode: u32) -> Option<&Glyph> {
        self.glyphs.get(&unicode)
    }

    pub fn build_text<T: AsRef<str>>(&self, text: T) -> (TextRenderData, Size<f64>) {
        let graphemes: Vec<&str> = text.as_ref()
            .graphemes(true)
            .collect();

        let tab_whitespace_size = 4;
        let extra_space = graphemes.iter()
                                   .map(|g| (g == &"\t") as usize)
                                   .reduce(|acc, e| acc + (e * (tab_whitespace_size - 1)))
                                   .unwrap();

        let mut output = TextRenderData::new(graphemes.len() + extra_space);
        let mut text_em = Size::default();

        let mut pen = Vector2::new(0.0f64, self.rendering.ascender().abs() as f64);
        let mut end_of_line = false;

        let mut i = 0;
        while i < graphemes.len() {
            let grapheme = &Grapheme::Direct(graphemes[i]);

            // special handling
            match grapheme.value() {
                "\n" => {
                    // new line
                    pen.y += self.rendering.line_height() as f64;
                    i += 1;
                    continue
                },
                "\r" => {
                    // carriage return

                    // do nothing, just ignore
                    // TODO: maybe add an option to detect when carriage return handling is needed
                    i += 1;
                    continue
                },
                _ => (),
            }

            if Self::is_at_end_of_line(&graphemes, i) {
                // near the end of line
                end_of_line = true;
            }

            let (grapheme, render_times) = self.normalize_grapheme(grapheme);
            let unicode = grapheme.unicode();

            let glyph = match self.glyph(unicode) {
                Some(glyph) => glyph,
                None => {
                    // glyph not found, just render default symbol
                    println!("Glyph ({0}) not found at '{1}' (index: {2}))", unicode, text.as_ref(), i);
                    let err_unicode = self.missing_glyph().unicode();
                    self.glyph(err_unicode).unwrap()
                }
            };

            for _ in 0..render_times {
                //
                // underrun
                //

                if pen.x == 0.0 {
                    pen.x -= glyph.bearing.x;
                }

                //

                output.append(pen + glyph.bearing, unicode, &glyph);

                pen += glyph.advance;

                //
                // kerning with next repeated character
                //

                // adjust for kerning between this character
                // and the next (if it'll repeat)
                if self.rendering.has_kerning()
                 && !end_of_line
                 && render_times > 1
                {
                    pen.x += self.rendering.kerning(unicode, unicode);
                }
            }

            //
            // kerning with next character
            //

            if self.rendering.has_kerning() && !end_of_line {
                let next_grapheme = &Grapheme::Direct(graphemes[i + 1]);
                let (next_grapheme, _render_times) = self.normalize_grapheme(next_grapheme);
                let next_unicode = next_grapheme.unicode();

                pen.x += self.rendering.kerning(unicode, next_unicode);
            }

            //

            if end_of_line {
                end_of_line = false;

                if pen.x > text_em.x {
                    text_em.x = pen.x;
                }

                pen.x = 0.0;
            }

            i += 1;
        }

        text_em.y = pen.y.abs() + self.rendering.descender() as f64;

        (output, text_em)
    }

    fn is_at_end_of_line(graphemes: &[&str], current_index: usize) -> bool {
            // [.., __]
            //      /\
        return current_index + 1 == graphemes.len()

            // [.., __, \n]
            //      /\
            || graphemes[current_index + 1] == "\n"

            // [.., __, \r, \n]
            //      /\
            || (current_index + 2 < graphemes.len() && graphemes[current_index + 1] == "\r" && graphemes[current_index + 2] == "\n")
    }

    fn missing_glyph(&self) -> Grapheme {
        Grapheme::Indirect("?")
    }

    fn normalize_grapheme<'g>(&'g self, grapheme: &'g Grapheme) -> (&'g Grapheme, u32) {
        if grapheme == "\t" {
            (&WHITESPACE_GRAPHEME, TABULATION_WHITESPACE_AMOUNT)
        } else {
            (grapheme, 1)
        }
    }
}

enum Grapheme<'a> {
    Direct(&'a str),
    Indirect(&'static str),
}

impl<'a> Grapheme<'a> {
    pub fn value(&self) -> &str {
        match self {
            Grapheme::Direct(grapheme) => grapheme,
            Grapheme::Indirect(grapheme) => grapheme,
        }
    }

    pub fn unicode(&self) -> u32 {
        let bytes = self.value().as_bytes();
        let mut byte_buffer: [u8; 4] = [0, 0, 0, 0];
        byte_buffer[(4 - bytes.len())..].copy_from_slice(&bytes);

        if cfg!(target_endian = "little") {
            byte_buffer.reverse();
            u32::from_le_bytes(byte_buffer)
        } else {
            u32::from_be_bytes(byte_buffer)
        }
    }
}

impl<'a> Display for Grapheme<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grapheme::Direct(grapheme) => f.write_str(grapheme),
            Grapheme::Indirect(grapheme) => f.write_str(grapheme),
        }
    }
}

impl<'a> PartialEq<&str> for &Grapheme<'a> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Grapheme::Direct(grapheme) => grapheme == other,
            Grapheme::Indirect(grapheme) => grapheme == other,
        }
    }
}
