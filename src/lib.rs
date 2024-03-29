use std::char::from_u32_unchecked;
use std::mem::size_of_val;

// FIXME: Needs to be moved?
mod contextual_form;
pub use contextual_form::get_arabic_contextual_form;

#[derive(Debug, PartialEq, Eq)]
pub enum Glyph {
    HalfWidth([u8; 16]),
    FullWidth([u16; 16]),
}

impl Glyph {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        y < 16
            && match self {
                Glyph::HalfWidth(rows) => x < 8 && rows[y] & (0x80 >> x) != 0,
                Glyph::FullWidth(rows) => x < 16 && rows[y] & (0x8000 >> x) != 0,
            }
    }

    pub fn get_width(&self) -> usize {
        match self {
            Glyph::HalfWidth(_) => 8,
            Glyph::FullWidth(_) => 16,
        }
    }
}

pub fn get_glyph(c: char) -> Option<&'static Glyph> {
    let code_point = c as usize;
    let mut offset: usize = 0;
    let mut result = None;
    for (start, end) in CODE_POINT_RANGES.iter() {
        if *start <= code_point && code_point < *end {
            result = Some(&GLYPH_TABLE[offset + code_point - start]);
            break;
        } else {
            offset += end - start;
        }
    }
    result
}

pub fn enumerate_glyphs() -> impl Iterator<Item = (char, &'static Glyph)> {
    let char_iterator = CODE_POINT_RANGES
        .iter()
        .flat_map(|(start, end)| *start..*end)
        .map(|code_point| unsafe { from_u32_unchecked(code_point as u32) });
    let glyph_iterator = GLYPH_TABLE.iter();
    char_iterator.zip(glyph_iterator)
}

pub fn get_storage_size() -> usize {
    size_of_val(&CODE_POINT_RANGES) + size_of_val(&GLYPH_TABLE)
}

include!(concat!(env!("OUT_DIR"), "/glyph_table.rs"));
