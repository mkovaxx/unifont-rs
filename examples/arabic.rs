use clap::Parser;
use unifont::{get_glyph, Glyph};
use unifont::bidi::process_bidi_text; 
use unifont::scripts::arabic::get_arabic_contextual_form;

/// Simple program to render an Arabic text banner
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

    // Now using the `process_bidi_text` function for bidi processing
    let bidi_processed_text = process_bidi_text(&args.text);

    // Applying Arabic contextual forms processing
    let arabic_corrected_text = get_arabic_contextual_form(&bidi_processed_text);

    let glyphs: Vec<&Glyph> = arabic_corrected_text.chars().map(|c| get_glyph(c).unwrap()).collect();

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
