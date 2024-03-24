mod util;

#[test]
fn test_glyph_a() {
    let glyph = util::GLYPH_A;
    assert_eq!(glyph.get_width(), 8);
    assert_eq!(util::render(&glyph), util::GLYPH_A_SHAPE);
    assert_eq!(glyph.get_pixel(8, 5), false);
    assert_eq!(glyph.get_pixel(3, 42), false);
}

#[test]
fn test_glyph_ji() {
    let glyph = util::GLYPH_JI;
    assert_eq!(glyph.get_width(), 16);
    assert_eq!(util::render(&glyph), util::GLYPH_JI_SHAPE);
    assert_eq!(glyph.get_pixel(16, 5), false);
    assert_eq!(glyph.get_pixel(3, 42), false);
}
