use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input_dir = env::current_dir().unwrap();
    let input_path = Path::new(&input_dir).join("data/unifont-10.0.07.hex");
    let mut input_file = File::open(&input_path).unwrap();
    let mut input_reader = BufReader::new(&input_file);
    let mut glyph_map: BTreeMap<u16, String> = BTreeMap::new();
    // TODO(mkovacs): enumerate contiguous ranges
    for line_result in input_reader.lines() {
        let line = line_result.unwrap();
        let code_point = u16::from_str_radix(&line[0..4], 16).unwrap();
        let data = String::from_str(&line[5..]).unwrap();
        glyph_map.insert(code_point, data);
    }

    let output_dir = env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&output_dir).join("unifont.rs");
    let mut output_file = File::create(&output_path).unwrap();

    writeln!(output_file, "pub fn get_glyph(code_point: usize) -> Option<&'static Glyph> {{");
    writeln!(output_file, "    Some(&GLYPH_TABLE[code_point])");
    writeln!(output_file, "}}");
    writeln!(output_file, "const GLYPH_TABLE: [Glyph; {}] = [", glyph_map.len());
    for (code_point, data) in glyph_map {
        match data.len() {
            32 => {
                let u8s: Vec<String> =
                    Vec::from_iter(data.chars())
                        .chunks(2)
                        .map(|chunk| {
                            let hex: String = chunk.iter().cloned().collect();
                            format!("0x{}", hex)
                        })
                        .collect();
                writeln!(output_file, "    Glyph::HalfWidth([{}]),", u8s.join(", "));
            },
            64 => {
                let u16s: Vec<String> =
                    Vec::from_iter(data.chars())
                        .chunks(4)
                        .map(|chunk| {
                            let hex: String = chunk.iter().cloned().collect();
                            format!("0x{}", hex)
                        }).collect();
                writeln!(output_file, "    Glyph::FullWidth([{}]),", u16s.join(", "));
            },
            _ => {
                writeln!(output_file, "ERROR: invalid glyph data: {}", data);
            },
        }
    }
    writeln!(output_file, "];");
}
