use crate::glyph::Glyph;
use std::char::from_u32_unchecked;
use std::mem::size_of_val;

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
    Box::new(char_iterator.zip(glyph_iterator))
}

pub fn get_storage_size() -> usize {
    size_of_val(&CODE_POINT_RANGES) + size_of_val(&GLYPH_TABLE)
}

include!(concat!(env!("OUT_DIR"), "/glyph_table.rs"));
