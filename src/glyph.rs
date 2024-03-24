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
