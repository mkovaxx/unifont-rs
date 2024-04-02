use clap::Parser;
use unifont::{get_glyph, Glyph};

/// Simple program to render a basic text banner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Text to render
    #[arg(required = true, default_value = "UniFont")]
    text: String,

    /// Character to use as foreground
    #[arg(short, long, default_value = "#")]
    foreground: char,

    /// Character to use as background
    #[arg(short, long, default_value = " ")]
    background: char,
}

fn main() {
    let args = Args::parse();

    let glyphs: Vec<&Glyph> = args.text.chars().map(|c| get_glyph(c).unwrap()).collect();

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
