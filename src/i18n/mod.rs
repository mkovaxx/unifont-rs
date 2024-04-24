extern crate alloc;
use alloc::string::String;

mod arabic;

/// Preprocess text so that it may be rendered via Unifont.
pub fn preprocess_text(text: &str) -> String {
    // TODO: put the sequence into Normalization Form C to get precomposed characters
    // Ensure that Arabic presentation forms are used where necessary
    let text = arabic::substitute_presentation_forms(text);
    text
}
