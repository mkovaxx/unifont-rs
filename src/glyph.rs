pub enum Glyph {
    HalfWidth([u8; 16]),
    FullWidth([u16; 16]),
}

impl Glyph {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if y < 16 {
            match self {
                Glyph::HalfWidth(rows) => {
                    rows[y] & (0x80 >> x) != 0
                }
                Glyph::FullWidth(rows) => {
                    rows[y] & (0x8000 >> x) != 0
                }
            }
        } else {
            false
        }
    }

    pub fn get_width(&self) -> usize {
        match self {
            Glyph::HalfWidth(_) => 8,
            Glyph::FullWidth(_) => 16,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;
    use std::str::FromStr;
    use super::*;

    fn render(glyph: &Glyph) -> Vec<String> {
        let width = glyph.get_width();
        (0..16).map(|y| {
            String::from_iter((0..width).map(|x| {
                if glyph.get_pixel(x, y) { '#' } else { '-' }
            }))
        }).collect()
    }

    fn to_vec_string(s: [&str; 16]) -> Vec<String> {
        s.iter().map(|s| { s.to_string() }).collect()
    }

    #[test]
    fn glyph_a() {
        let glyph = Glyph::HalfWidth([0x00, 0x00, 0x00, 0x00, 0x18, 0x24, 0x24, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00]);
        assert_eq!(glyph.get_width(), 8);
        assert_eq!(render(&glyph), to_vec_string([
            "--------",
            "--------",
            "--------",
            "--------",
            "---##---",
            "--#--#--",
            "--#--#--",
            "-#----#-",
            "-#----#-",
            "-######-",
            "-#----#-",
            "-#----#-",
            "-#----#-",
            "-#----#-",
            "--------",
            "--------",
        ]));
    }

    #[test]
    fn glyph_b() {
        let glyph = Glyph::HalfWidth([0x00, 0x00, 0x00, 0x00, 0x7C, 0x42, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x42, 0x42, 0x7C, 0x00, 0x00]);
        assert_eq!(glyph.get_width(), 8);
        assert_eq!(render(&glyph), to_vec_string([
            "--------",
            "--------",
            "--------",
            "--------",
            "-#####--",
            "-#----#-",
            "-#----#-",
            "-#----#-",
            "-#####--",
            "-#----#-",
            "-#----#-",
            "-#----#-",
            "-#----#-",
            "-#####--",
            "--------",
            "--------",
        ]));
    }
}
