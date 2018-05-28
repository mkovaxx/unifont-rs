pub mod glyph;
pub mod hexformat;
pub mod unifont;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use glyph;

    #[test]
    fn glyph_a() {
        let g = glyph::Glyph::HalfWidth([0x00, 0x00, 0x00, 0x00, 0x18, 0x24, 0x24, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00]);
        assert_eq!(g.get_width(), 8);
        assert_eq!(g.show(), vec![
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
        ].iter().map(|s| { s.to_string() }).collect::<Vec<String>>());
    }

    #[test]
    fn glyph_b() {
        let g = glyph::Glyph::HalfWidth([0x00, 0x00, 0x00, 0x00, 0x7C, 0x42, 0x42, 0x42, 0x7C, 0x42, 0x42, 0x42, 0x42, 0x7C, 0x00, 0x00]);
        assert_eq!(g.get_width(), 8);
        assert_eq!(g.show(), vec![
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
        ].iter().map(|s| { s.to_string() }).collect::<Vec<String>>());
    }
}
