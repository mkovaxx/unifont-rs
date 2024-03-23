use std::iter::FromIterator;

use crate::glyph::Glyph;

pub const GLYPH_A: Glyph = Glyph::HalfWidth([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42,
    0x02, 0x3E, 0x42, 0x42, 0x46, 0x3A, 0x00, 0x00,
]);

pub const GLYPH_A_SHAPE: [&str; 16] = [
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
];

pub const GLYPH_JI: Glyph = Glyph::FullWidth([
    0x0200, 0x0100, 0x7FFE, 0x4002, 0x8004, 0x1FE0, 0x0040, 0x0080,
    0x0100, 0xFFFE, 0x0100, 0x0100, 0x0100, 0x0100, 0x0500, 0x0200,
]);

pub const GLYPH_JI_SHAPE: [&str; 16] = [
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
];

pub fn render(glyph: &Glyph) -> Vec<String> {
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
