extern crate alloc;
use alloc::string::{String, ToString};
use unicode_bidi::BidiInfo;

mod arabic;

/// Preprocess a line of text so that it may be rendered via Unifont.
/// Currently supported scripts: Arabic.
pub fn preprocess_line(line: &str) -> String {
    let text = arabic::use_contextual_forms(line);

    // apply the BiDi algorithm, assuming that the text is a single line
    let bidi_info = BidiInfo::new(&text, None);
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);

    display.to_string()
}
