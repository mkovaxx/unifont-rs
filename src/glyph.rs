use std::iter::FromIterator;

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
                },
                Glyph::FullWidth(rows) => {
                    rows[y] & (0x8000 >> x) != 0
                },
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

    pub fn show(&self) -> Vec<String> {
        let width = self.get_width();
        (0..16).map(|y| {
            String::from_iter((0..width).map(|x| {
                if self.get_pixel(x, y) { '#' } else { '-' }
            }))
        }).collect()
    }
}
