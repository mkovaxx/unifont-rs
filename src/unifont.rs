use glyph::Glyph;

include!(concat!(env!("OUT_DIR"), "/unifont.rs"));

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;
    use super::*;

    fn render(glyph: &Glyph) -> Vec<String> {
        let width = glyph.get_width();
        (0..16).map(|y| {
            String::from_iter((0..width).map(|x| {
                if glyph.get_pixel(x, y) { '#' } else { '-' }
            }))
        }).collect()
    }

    fn vec_of_strings(s: [&str; 16]) -> Vec<String> {
        s.iter().map(|s| { s.to_string() }).collect()
    }

    #[test]
    fn glyph_a() {
        let glyph = get_glyph('a' as usize).unwrap();
        assert_eq!(glyph.get_width(), 8);
        assert_eq!(render(&glyph), vec_of_strings([
            "--------",
            "--------",
            "--------",
            "--------",
            "--------",
            "--------",
            "--####--",
            "-#----#-",
            "------#-",
            "--#####-",
            "-#----#-",
            "-#----#-",
            "-#---##-",
            "--###-#-",
            "--------",
            "--------",
        ]));
        assert_eq!(glyph.get_pixel(8, 5), false);
        assert_eq!(glyph.get_pixel(3, 42), false);
    }
}
