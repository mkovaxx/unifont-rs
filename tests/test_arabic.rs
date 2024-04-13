//! Test substitution of Arabic presentation forms

use unifont::i18n::preprocess_text;

#[test]
fn test_isolated_lam_alif() {
    let input = "\u{0644}\u{0627}";
    let expected = "\u{FEFB}";
    assert_eq!(preprocess_text(input), expected);
}
