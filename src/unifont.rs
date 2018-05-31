use glyph::Glyph;

pub fn get_glyph(code_point: usize) -> Option<&'static Glyph> {
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

include!(concat!(env!("OUT_DIR"), "/glyph_table.rs"));

#[cfg(test)]
mod tests {
    use testutil;
    use super::*;

    #[test]
    fn glyph_a() {
        let glyph = get_glyph('a' as usize).unwrap();
        assert_eq!(glyph, &testutil::GLYPH_A);
    }

    #[test]
    fn glyph_ji() {
        let glyph = get_glyph('字' as usize).unwrap();
        assert_eq!(glyph, &testutil::GLYPH_JI);
    }
}
