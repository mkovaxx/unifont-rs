pub enum Glyph {
    HalfWidth([u8; 16]),
    FullWidth([u16; 16]),
}

impl Glyph {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        y < 16 &&
            match self {
                Glyph::HalfWidth(rows) => {
                    x < 8 && rows[y] & (0x80 >> x) != 0
                }
                Glyph::FullWidth(rows) => {
                    x < 16 && rows[y] & (0x8000 >> x) != 0
                }
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
        let glyph = Glyph::HalfWidth([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42,
            0x02, 0x3E, 0x42, 0x42, 0x46, 0x3A, 0x00, 0x00,
        ]);
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

    #[test]
    fn glyph_ji() {
        let glyph = Glyph::FullWidth([
            0x0200, 0x0100, 0x7FFE, 0x4002, 0x8004, 0x1FE0, 0x0040, 0x0080,
            0x0100, 0xFFFE, 0x0100, 0x0100, 0x0100, 0x0100, 0x0500, 0x0200,
        ]);
        assert_eq!(glyph.get_width(), 16);
        assert_eq!(render(&glyph), vec_of_strings([
            "------#---------",
            "-------#--------",
            "-##############-",
            "-#------------#-",
            "#------------#--",
            "---########-----",
            "---------#------",
            "--------#-------",
            "-------#--------",
            "###############-",
            "-------#--------",
            "-------#--------",
            "-------#--------",
            "-------#--------",
            "-----#-#--------",
            "------#---------",
        ]));
        assert_eq!(glyph.get_pixel(16, 5), false);
        assert_eq!(glyph.get_pixel(3, 42), false);
    }
}
