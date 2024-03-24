use clap::Parser;
use unifont::{get_glyph, get_arabic_contextual_form, Glyph};
use unicode_bidi::BidiInfo;

/// Simple program to render a text banner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to render
    #[arg(required = true, default_value = "UniFont")]
    text: String,

    /// Character to use as foreground
    #[arg(short, long, default_value = "$")]
    foreground: char,

    /// Character to use as background
    #[arg(short, long, default_value = " ")]
    background: char,
}

fn main() {
    let args = Args::parse();
    let text = args.text.clone();
    let bidi_info = BidiInfo::new(&text, None);
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);

    let arabic_corr = get_arabic_contextual_form(&display);
    let glyphs: Vec<&Glyph> = arabic_corr.chars().map(|c| get_glyph(c).unwrap()).collect();


    for y in 0..16 {
        for glyph in &glyphs {
            for x in 0..glyph.get_width() {
                let pixel = if glyph.get_pixel(x, y) {
                    args.foreground
                } else {
                    args.background
                };
                print!("{pixel}");
            }
        }
        println!();
    }
}
