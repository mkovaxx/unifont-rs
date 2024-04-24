//! Test substitution of Arabic presentation forms

use unifont::i18n::preprocess_text;

macro_rules! test_forms {
    ( $name:ident, $description:literal, $input:literal, $expected:literal $(,)? ) => {
        #[test]
        fn $name() {
            assert_eq!(
                preprocess_text($input),
                $expected,
                "{} failed",
                $description
            );
        }
    };
}

test_forms!(
    lam_alef_to_la_isolated,
    "Lam Alef should be transformed to la isolated form",
    "\u{0644}\u{0627}",
    "\u{FEFB}",
);

test_forms!(
    lam_alef_with_hamza_to_la_isolated,
    "Lam Alef with Hamza should be transformed to la isolated form",
    "\u{0644}\u{0623}",
    "\u{FEF7}",
);

test_forms!(
    lam_alef_with_maddah_to_la_isolated,
    "Lam Alef with Maddah should be transformed to la isolated form",
    "\u{0644}\u{0622}",
    "\u{FEF5}",
);

// Words combining different letters

test_forms!(
    words_lan,
    "Lan: Lam, Alef, Noon", // 'لان'
    "\u{0644}\u{0627}\u{0646}",
    "\u{FEFB}\u{FEE5}",
);

test_forms!(
    words_salan,
    "Salan: Fa, Lam, Alef, Noon", // 'فلان'
    "\u{0641}\u{0644}\u{0627}\u{0646}",
    "\u{FED3}\u{FEFC}\u{FEE5}",
);

test_forms!(
    words_sala,
    "Sala: Fa, Lam, Alef", // 'فلا'
    "\u{0641}\u{0644}\u{0627}",
    "\u{FED3}\u{FEFC}",
);

test_forms!(
    words_wala,
    "Wala: Wao, Lam, Alef", // 'ولا'
    "\u{0648}\u{0644}\u{0627}",
    "\u{FEED}\u{FEFB}",
);

test_forms!(
    words_ana,
    "Ana: Alef, Noon, Alef", // 'أنا'
    "\u{0623}\u{0646}\u{0627}",
    "\u{FE83}\u{FEE7}\u{FE8E}",
);

test_forms!(
    words_aldaw_wa_alzalam,
    "Al-Daw' wa al-Zalam", // الضوء و الظلام
    "\u{0627}\u{0644}\u{0636}\u{0648}\u{0621}\u{0020}\u{0648}\u{0020}\u{0627}\u{0644}\u{0638}\u{0644}\u{0627}\u{0645}",
    "\u{FE8D}\u{FEDF}\u{FEC0}\u{FEEE}\u{FE80}\u{0020}\u{FEED}\u{0020}\u{FE8D}\u{FEDF}\u{FEC8}\u{FEFC}\u{FEE1}",
);
