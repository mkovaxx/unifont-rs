use std::collections::HashMap;

struct ArabicForm {
    isolated: u32,
    final_: u32,
    initial: Option<u32>,
    medial: Option<u32>,
    special: Option<(Option<u32>, Option<u32>)>, // speical case for lam-alef
}

const ARABIC_LETTER_START: u32 = 0x621;
const ARABIC_LETTER_END: u32 = 0x64A;
const UNICODE_LAM: u32 = 0x644;

pub fn get_arabic_presentation_forms(input: &str) -> String {
    let mut result = String::new();
    let mut next_char: char = '\0'; // Initial 'previous' character

    // Since we're working with chars directly, we reverse iterate over the input as chars, not code points
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];
        let prev_char = chars.get(i + 1).copied().unwrap_or('\0'); // Use '\0' to represent the absence of a next character

        if let Some(transformed_cp) = get_presentation_form_b_of_char(prev_char, next_char, c) {
            if let Some(transformed_char) = std::char::from_u32(transformed_cp) {
                result.push(transformed_char);
            } else {
                eprintln!("Warning: Invalid Unicode code point: {:X}", transformed_cp);
            }
        }

        next_char = c; // Update `next_char` for the next iteration
    }

    result
}

static ARABIC_FORMS_B: [ArabicForm; 42] = [
    ArabicForm { isolated: 0xFE80, final_: 0xFE80, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFE81, final_: 0xFE82, initial: None, medial: None, special: Some((Some(0xFEF5), Some(0xFEF6))) },
    ArabicForm { isolated: 0xFE83, final_: 0xFE84, initial: None, medial: None, special: Some((Some(0xFEF7), Some(0xFEF8))) },
    ArabicForm { isolated: 0xFE85, final_: 0xFE86, initial: None, medial: None, special: Some((Some(0xFEF9), Some(0xFEFA))) },
    ArabicForm { isolated: 0xFE87, final_: 0xFE88, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFE89, final_: 0xFE8A, initial: Some(0xFE8B), medial: Some(0xFE8C), special: None },
    ArabicForm { isolated: 0xFE8D, final_: 0xFE8E, initial: None, medial: None, special: Some((Some(0xFEFB), Some(0xFEFC))) },
    ArabicForm { isolated: 0xFE8F, final_: 0xFE90, initial: Some(0xFE91), medial: Some(0xFE92), special: None },
    ArabicForm { isolated: 0xFE93, final_: 0xFE94, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFE95, final_: 0xFE96, initial: Some(0xFE97), medial: Some(0xFE98), special: None },
    ArabicForm { isolated: 0xFE99, final_: 0xFE9A, initial: Some(0xFE9B), medial: Some(0xFE9C), special: None },
    ArabicForm { isolated: 0xFE9D, final_: 0xFE9E, initial: Some(0xFE9F), medial: Some(0xFEA0), special: None },
    ArabicForm { isolated: 0xFEA1, final_: 0xFEA2, initial: Some(0xFEA3), medial: Some(0xFEA4), special: None },
    ArabicForm { isolated: 0xFEA5, final_: 0xFEA6, initial: Some(0xFEA7), medial: Some(0xFEA8), special: None },
    ArabicForm { isolated: 0xFEA9, final_: 0xFEAA, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFEAB, final_: 0xFEAC, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFEAD, final_: 0xFEAE, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFEAF, final_: 0xFEB0, initial: None, medial: None, special: None },
    ArabicForm { isolated: 0xFEB1, final_: 0xFEB2, initial: Some(0xFEB3), medial: Some(0xFEB4), special: None },
    ArabicForm { isolated: 0xFEB5, final_: 0xFEB6, initial: Some(0xFEB7), medial: Some(0xFEB8), special: None },
    ArabicForm { isolated: 0xFEB9, final_: 0xFEBA, initial: Some(0xFEBB), medial: Some(0xFEBC), special: None },
    ArabicForm { isolated: 0xFEBD, final_: 0xFEBE, initial: Some(0xFEBF), medial: Some(0xFEC0), special: None },
    ArabicForm { isolated: 0xFEC1, final_: 0xFEC2, initial: Some(0xFEC3), medial: Some(0xFEC4), special: None },
    ArabicForm { isolated: 0xFEC5, final_: 0xFEC6, initial: Some(0xFEC7), medial: Some(0xFEC8), special: None },
    ArabicForm { isolated: 0xFEC9, final_: 0xFECA, initial: Some(0xFECB), medial: Some(0xFECC), special: None },
    ArabicForm { isolated: 0xFECD, final_: 0xFECE, initial: Some(0xFECF), medial: Some(0xFED0), special: None }, // gh, special: Noneayn
    ArabicForm { isolated: 0, final_: 0, initial: None, medial: None, special: None }, // Placeholder for unused entries
    ArabicForm { isolated: 0, final_: 0, initial: None, medial: None, special: None }, // Placeholder for unused entries
    ArabicForm { isolated: 0, final_: 0, initial: None, medial: None, special: None }, // Placeholder for unused entries
    ArabicForm { isolated: 0, final_: 0, initial: None, medial: None, special: None }, // Placeholder for unused entries
    ArabicForm { isolated: 0, final_: 0, initial: None, medial: None, special: None }, // Placeholder for unused entries
    ArabicForm { isolated: 0x640, final_: 0x640, initial: Some(0x640), medial: Some(0x640), special: None }, // wasla
    ArabicForm { isolated: 0xFED1, final_: 0xFED2, initial: Some(0xFED3), medial: Some(0xFED4), special: None }, // fa2
    ArabicForm { isolated: 0xFED5, final_: 0xFED6, initial: Some(0xFED7), medial: Some(0xFED8), special: None }, // qaf
    ArabicForm { isolated: 0xFED9, final_: 0xFEDA, initial: Some(0xFEDB), medial: Some(0xFEDC), special: None }, // kaf
    ArabicForm { isolated: 0xFEDD, final_: 0xFEDE, initial: Some(0xFEDF), medial: Some(0xFEE0), special: None }, // lam
    ArabicForm { isolated: 0xFEE1, final_: 0xFEE2, initial: Some(0xFEE3), medial: Some(0xFEE4), special: None }, // mim
    ArabicForm { isolated: 0xFEE5, final_: 0xFEE6, initial: Some(0xFEE7), medial: Some(0xFEE8), special: None }, // noon
    ArabicForm { isolated: 0xFEE9, final_: 0xFEEA, initial: Some(0xFEEB), medial: Some(0xFEEC), special: None }, // ha2
    ArabicForm { isolated: 0xFEED, final_: 0xFEEE, initial: None, medial: None , special: None}, // waw
    ArabicForm { isolated: 0xFEEF, final_: 0xFEF0, initial: None, medial: None , special: None}, // 2alif maksoura
    ArabicForm { isolated: 0xFEF1, final_: 0xFEF2, initial: Some(0xFEF3), medial: Some(0xFEF4), special: None }, // ya2
];

fn get_presentation_form_b_of_char( prev: char, next: char, c: char) -> Option<u32> {
    let cp = c as u32;
    let next_cp = next as u32;
    let prev_cp = prev as u32;

    if !is_arabic_letter(cp) {
        return Some(cp); // Not an Arabic letter, so return it as is.
    }
    
    let is_la = is_lam_alef(cp, next_cp);
    let is_apl = is_alef_prev_lam(prev_cp, cp);
    let is_lapl = is_la || is_apl;


    // The reference index now takes into account the special Lam-Alef combination.
    let ref_index = if is_la { next_cp } else { cp } - ARABIC_LETTER_START;

    // Use 'is_lapl' to decide if we need to handle special Lam-Alef combinations.
    let index = ((((is_lapl || is_arabic_letter(next_cp)) && is_linking_type(cp)) as u32) << 1) | is_linking_type(prev_cp) as u32;


    if let Some(form) = ARABIC_FORMS_B.get(ref_index as usize) {
        if is_lapl {
            if is_apl {
                return None;
            }
            // Directly handle special Lam-Alef or Alef-Lam cases without using 'index'.
            return form.special.and_then(|(_, special_initial)| special_initial.or(Some(form.isolated)));
        }
        println!("c: {}, index: {}, prev: {}, prevLink: {}", c, index, prev, is_linking_type(prev_cp));
        match index {
            0 => Some(form.isolated), // Non-linking, isolated form
            1 => Some(form.final_),   // Final form, if 'prev' is non-linking but 'cp' is linking
            2 => form.initial.or(Some(form.isolated)), // Initial form, if 'next' is linking
            3 => form.medial.or(Some(form.isolated)),  // Medial form, if both 'prev' and 'next' are linking
            _ => Some(form.isolated), // Fallback to isolated form for unexpected index values
        }
    } else {
        None // Form not found
    }
}



fn is_arabic_letter(cp: u32) -> bool {
    cp >= ARABIC_LETTER_START && cp <= ARABIC_LETTER_END
}

fn is_lam_alef(cp: u32, next: u32) -> bool {
    if cp == UNICODE_LAM && is_arabic_letter(next) {
        if let Some(form) = ARABIC_FORMS_B.get((next - ARABIC_LETTER_START) as usize) {
            // Assuming the special case logic needs to check the 'initial' form's existence or special condition
            return form.special.map_or(false, |(_, initial)| initial.is_some());
        }
    }
    false
}

fn is_alef_prev_lam(prev: u32, cp: u32) -> bool {
    if prev == UNICODE_LAM && is_arabic_letter(cp) {
        if let Some(form) = ARABIC_FORMS_B.get((cp - ARABIC_LETTER_START) as usize) {
            // Similar logic as in is_lam_alef, adjusted for checking 'cp' instead of 'next'
            return form.special.map_or(false, |(_, initial)| initial.is_some());
        }
    }
    false
}

fn is_linking_type(cp: u32) -> bool {
    if is_arabic_letter(cp) {
        if let Some(form) = ARABIC_FORMS_B.get((cp - ARABIC_LETTER_START) as usize) {
            // Checking the 'medial' form's existence or special condition
            return form.medial.is_some();
            // return form.medial.is_some();
        }
    }
    false
}
