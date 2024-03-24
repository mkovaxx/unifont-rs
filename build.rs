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
    let input_file = File::open(&input_path).unwrap();
    let input_reader = BufReader::new(&input_file);
    let mut glyph_map: BTreeMap<u16, String> = BTreeMap::new();
    for line_result in input_reader.lines() {
        let line = line_result.unwrap();
        let code_point = u16::from_str_radix(&line[0..4], 16).unwrap();
        let data = String::from_str(&line[5..]).unwrap();
        glyph_map.insert(code_point, data);
    }

    // enumerate contiguous ranges
    let mut ranges: Vec<(i32, i32)> = vec![];
    let mut prev_code_point = -2;
    for code_point_u16 in glyph_map.keys() {
        let code_point = *code_point_u16 as i32;
        if prev_code_point + 1 < code_point {
            ranges.push((code_point, code_point + 1));
        } else {
            let i = ranges.len() - 1;
            ranges[i].1 = code_point + 1;
        }
        prev_code_point = code_point;
    }

    let output_dir = env::var("OUT_DIR").unwrap();
    let output_path = Path::new(&output_dir).join("glyph_table.rs");
    let mut output_file = File::create(&output_path).unwrap();

    writeln!(
        output_file,
        "static CODE_POINT_RANGES: [(usize, usize); {}] = [",
        ranges.len()
    );
    for (start, end) in ranges {
        writeln!(output_file, "    ({}, {}),", start, end);
    }
    writeln!(output_file, "];");

    writeln!(
        output_file,
        "static GLYPH_TABLE: [Glyph; {}] = [",
        glyph_map.len()
    );
    for data in glyph_map.values() {
        match data.len() {
            32 => {
                let u8s: Vec<String> = Vec::from_iter(data.chars())
                    .chunks(2)
                    .map(|chunk| {
                        let hex: String = chunk.iter().cloned().collect();
                        format!("0x{}", hex)
                    })
                    .collect();
                writeln!(output_file, "    Glyph::HalfWidth([{}]),", u8s.join(", "));
            }
            64 => {
                let u16s: Vec<String> = Vec::from_iter(data.chars())
                    .chunks(4)
                    .map(|chunk| {
                        let hex: String = chunk.iter().cloned().collect();
                        format!("0x{}", hex)
                    })
                    .collect();
                writeln!(output_file, "    Glyph::FullWidth([{}]),", u16s.join(", "));
            }
            _ => {
                writeln!(output_file, "ERROR: invalid glyph data: {}", data);
            }
        }
    }
    writeln!(output_file, "];");
}
