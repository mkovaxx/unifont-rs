#[derive(Debug, PartialEq, Eq)]
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
    use crate::testutil;

    #[test]
    fn glyph_a() {
        let glyph = testutil::GLYPH_A;
        assert_eq!(glyph.get_width(), 8);
        assert_eq!(testutil::render(&glyph), testutil::GLYPH_A_SHAPE);
        assert_eq!(glyph.get_pixel(8, 5), false);
        assert_eq!(glyph.get_pixel(3, 42), false);
    }

    #[test]
    fn glyph_ji() {
        let glyph = testutil::GLYPH_JI;
        assert_eq!(glyph.get_width(), 16);
        assert_eq!(testutil::render(&glyph), testutil::GLYPH_JI_SHAPE);
        assert_eq!(glyph.get_pixel(16, 5), false);
        assert_eq!(glyph.get_pixel(3, 42), false);
    }
}
