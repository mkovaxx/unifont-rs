mod arabic;

/// Preprocess a sequence of characters so that it may be rendered via Unifont.
/// Works in place to avoid the need for allocation.
pub fn preprocess_text(chars: &mut [char]) {
    // identify and process maximal spans of Arabic
    preprocess_arabic_text(chars);
}

fn preprocess_arabic_text(chars: &mut [char]) {
    use self::arabic::{get_arabic_contextual_form, is_arabic_letter};

    // find maximal spans that consist of Arabic and whitespace
    let spans = chars.split_mut(|c| !c.is_whitespace() && !is_arabic_letter(*c as u32));

    for span in spans {
        let trimmed = whitespace_trimmed(span);
        get_arabic_contextual_form(trimmed);
        trimmed.reverse();
    }
}

fn whitespace_trimmed(chars: &mut [char]) -> &mut [char] {
    let mut left = 0;
    while left < chars.len() && chars[left].is_whitespace() {
        left += 1;
    }

    let mut right = chars.len();
    while right > left && chars[right - 1].is_whitespace() {
        right -= 1;
    }

    &mut chars[left..right]
}
