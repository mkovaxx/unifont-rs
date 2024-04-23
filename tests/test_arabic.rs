//! Test substitution of Arabic presentation forms

use unifont::i18n::preprocess_text;

const TEST_CASES: &[(&str, &str, &str)] = &[
    // Lam Alef
    (
        "Lam Alef should be transformed to la isolated form",
        "\u{0644}\u{0627}",
        "\u{FEFB}",
    ),
    (
        "Lam Alef with Hamza should be transformed to la isolated form",
        "\u{0644}\u{0623}",
        "\u{FEF7}",
    ),
    (
        "Lam Alef with Maddah should be transformed to la isolated form",
        "\u{0644}\u{0622}",
        "\u{FEF5}",
    ),
    // Words combining different letters
    (
        "Lan: Lam, Alef, Noon", // 'لان'
        "\u{0644}\u{0627}\u{0646}",
        "\u{FEFB}\u{FEE5}",
    ),
    (
        "Sala: Fa, Lam, Alef, Noon", // 'فلان'
        "\u{0641}\u{0644}\u{0627}\u{0646}",
        "\u{FED3}\u{FEFC}\u{FEE5}",
    ),
    (
        "Sala: Fa, Lam, Alef", // 'فلا'
        "\u{0641}\u{0644}\u{0627}",
        "\u{FED3}\u{FEFC}",
    ),
    (
        "Sala: Wao, Lam, Alef", // 'ولا'
        "\u{0648}\u{0644}\u{0627}",
        "\u{FEED}\u{FEFB}",
    ),
    (
        "Ana: Alef, Noon, Alef",// 'أنا'
        "\u{0623}\u{0646}\u{0627}",
        "\u{FE83}\u{FEE7}\u{FE8E}",
    ),
    (
        "Al-Daw' wa al-Zalam", // الضوء و الظلام
        "\u{0627}\u{0644}\u{0636}\u{0648}\u{0621}\u{0020}\u{0648}\u{0020}\u{0627}\u{0644}\u{0638}\u{0644}\u{0627}\u{0645}",
        "\u{FE8D}\u{FEDF}\u{FEC0}\u{FEEE}\u{FE80}\u{0020}\u{FEED}\u{0020}\u{FE8D}\u{FEDF}\u{FEC8}\u{FEFC}\u{FEE1}",
    ),
];

#[test]
fn test_arabic_presentation_forms() {
    for &(name, input, expected) in TEST_CASES {
        assert_eq!(preprocess_text(input), expected, "{} failed", name);
    }
}
