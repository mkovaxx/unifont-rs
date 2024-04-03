extern crate alloc;
use alloc::{string::String, vec};

const NONE: u32 = u32::MAX;
const AR_START: u32 = 0x621;
const AR_END: u32 = 0x64A;
const UNICODE_LAM: u32 = 0x644;

pub fn use_contextual_forms(input: &str) -> String {
    let mut output = String::new();

    let mut chars = vec!['\0'];
    chars.extend(input.chars());
    chars.push('\0');

    for i in 1..chars.len() - 1 {
        let prev_char = chars[i - 1];
        let c = chars[i];
        let next_char = chars[i + 1];

        if let Some(transformed_cp) = get_contextual_form_of_char(prev_char, next_char, c) {
            if let Some(transformed_char) = core::char::from_u32(transformed_cp) {
                output.push(transformed_char);
            } else {
                panic!("Invalid Unicode code point: {:X}", transformed_cp);
            }
        }
    }

    output
}

// Arabic contextual forms [isolated, final, initial, medial, isolated_lam, final_lam]
static ARABIC_FORMS: [[u32; 6]; 42] = [
    [0xFE80, 0xFE80, NONE, NONE, NONE, NONE],
    [0xFE81, 0xFE82, NONE, NONE, 0xFEF5, 0xFEF6],
    [0xFE83, 0xFE84, NONE, NONE, 0xFEF7, 0xFEF8],
    [0xFE85, 0xFE86, NONE, NONE, 0xFEF9, 0xFEFA],
    [0xFE87, 0xFE88, NONE, NONE, NONE, NONE],
    [0xFE89, 0xFE8A, 0xFE8B, 0xFE8C, NONE, NONE],
    [0xFE8D, 0xFE8E, NONE, NONE, 0xFEFB, 0xFEFC],
    [0xFE8F, 0xFE90, 0xFE91, 0xFE92, NONE, NONE],
    [0xFE93, 0xFE94, NONE, NONE, NONE, NONE],
    [0xFE95, 0xFE96, 0xFE97, 0xFE98, NONE, NONE],
    [0xFE99, 0xFE9A, 0xFE9B, 0xFE9C, NONE, NONE],
    [0xFE9D, 0xFE9E, 0xFE9F, 0xFEA0, NONE, NONE],
    [0xFEA1, 0xFEA2, 0xFEA3, 0xFEA4, NONE, NONE],
    [0xFEA5, 0xFEA6, 0xFEA7, 0xFEA8, NONE, NONE],
    [0xFEA9, 0xFEAA, NONE, NONE, NONE, NONE],
    [0xFEAB, 0xFEAC, NONE, NONE, NONE, NONE],
    [0xFEAD, 0xFEAE, NONE, NONE, NONE, NONE],
    [0xFEAF, 0xFEB0, NONE, NONE, NONE, NONE],
    [0xFEB1, 0xFEB2, 0xFEB3, 0xFEB4, NONE, NONE],
    [0xFEB5, 0xFEB6, 0xFEB7, 0xFEB8, NONE, NONE],
    [0xFEB9, 0xFEBA, 0xFEBB, 0xFEBC, NONE, NONE],
    [0xFEBD, 0xFEBE, 0xFEBF, 0xFEC0, NONE, NONE],
    [0xFEC1, 0xFEC2, 0xFEC3, 0xFEC4, NONE, NONE],
    [0xFEC5, 0xFEC6, 0xFEC7, 0xFEC8, NONE, NONE],
    [0xFEC9, 0xFECA, 0xFECB, 0xFECC, NONE, NONE],
    [0xFECD, 0xFECE, 0xFECF, 0xFED0, NONE, NONE],
    [0, 0, NONE, NONE, NONE, NONE],
    [0, 0, NONE, NONE, NONE, NONE],
    [0, 0, NONE, NONE, NONE, NONE],
    [0, 0, NONE, NONE, NONE, NONE],
    [0, 0, NONE, NONE, NONE, NONE],
    [0x640, 0x640, 0x640, 0x640, NONE, NONE],
    [0xFED1, 0xFED2, 0xFED3, 0xFED4, NONE, NONE],
    [0xFED5, 0xFED6, 0xFED7, 0xFED8, NONE, NONE],
    [0xFED9, 0xFEDA, 0xFEDB, 0xFEDC, NONE, NONE],
    [0xFEDD, 0xFEDE, 0xFEDF, 0xFEE0, NONE, NONE],
    [0xFEE1, 0xFEE2, 0xFEE3, 0xFEE4, NONE, NONE],
    [0xFEE5, 0xFEE6, 0xFEE7, 0xFEE8, NONE, NONE],
    [0xFEE9, 0xFEEA, 0xFEEB, 0xFEEC, NONE, NONE],
    [0xFEED, 0xFEEE, NONE, NONE, NONE, NONE],
    [0xFEEF, 0xFEF0, NONE, NONE, NONE, NONE],
    [0xFEF1, 0xFEF2, 0xFEF3, 0xFEF4, NONE, NONE],
];

fn get_contextual_form_of_char(prev: char, next: char, c: char) -> Option<u32> {
    let cp = c as u32;
    let next_cp = next as u32;
    let prev_cp = prev as u32;

    if !is_arabic_letter(cp) {
        return Some(cp); // Not an Arabic letter, so return it as is.
    }

    let is_la = is_lam_alif(cp, next_cp);
    let is_apl = is_alif_after_lam(prev_cp, cp);
    let is_lapl = is_la || is_apl;

    // The reference index now takes into account the special Lam-Alif combination.
    let ref_index = if is_la { next_cp } else { cp } - AR_START;

    let target = ((((is_lapl || is_arabic_letter(next_cp)) && is_linking_type(cp)) as u32) << 1)
        | is_linking_type(prev_cp) as u32;

    if let Some(form) = ARABIC_FORMS.get(ref_index as usize) {
        if is_lapl {
            if is_apl {
                // Already handled by is_la
                return None;
            }
            if form[5] == NONE {
                return None;
            }
            return form[5].into();
        }
        return Some(form[target as usize]);
    } else {
        None // Form not found
    }
}

fn is_arabic_letter(cp: u32) -> bool {
    cp >= AR_START && cp <= AR_END
}

fn is_lam_alif(cp: u32, next: u32) -> bool {
    if cp == UNICODE_LAM && is_arabic_letter(next) {
        if let Some(form) = ARABIC_FORMS.get((next - AR_START) as usize) {
            return form[5] != NONE;
        }
    }
    false
}

fn is_alif_after_lam(prev: u32, cp: u32) -> bool {
    if prev == UNICODE_LAM && is_arabic_letter(cp) {
        if let Some(form) = ARABIC_FORMS.get((cp - AR_START) as usize) {
            return form[5] != NONE;
        }
    }
    false
}

fn is_linking_type(cp: u32) -> bool {
    if is_arabic_letter(cp) {
        if let Some(form) = ARABIC_FORMS.get((cp - AR_START) as usize) {
            return form[3] != NONE;
        }
    }
    false
}
