use unicode_bidi::BidiInfo;

/// Processes a given text string for correct bidi rendering.
///
/// # Arguments
///
/// * `text` - The text string to process.
///
/// # Returns
///
/// * `String` - The processed text string, ready for display according to bidi rules.
pub fn process_bidi_text(text: &str) -> String {
    let bidi_info = BidiInfo::new(text, None);
    let para = &bidi_info.paragraphs[0];
    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);
    display.into_owned()
}
