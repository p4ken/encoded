use std::io::{Cursor, Write};

use encoded::shift_jis;

#[test]
fn literals() {
    const KANJI: &[u8] = shift_jis!("漢字");
    assert_eq!(KANJI, b"\x8a\xbf\x8e\x9a");

    const KANJI_UNICODE: &[u8] = shift_jis!("\u{6f22}\u{5b57}");
    assert_eq!(KANJI_UNICODE, b"\x8a\xbf\x8e\x9a");

    const JI_CHAR: &[u8] = shift_jis!('字');
    assert_eq!(JI_CHAR, b"\x8e\x9a");

    const ABC: &[u8] = shift_jis!("abc");
    assert_eq!(ABC, b"abc");

    const ABC_RAW: &[u8] = shift_jis!(r##"a"#b"c"##);
    assert_eq!(ABC_RAW, br##"a"#b"c"##);

    const ABC_HEX: &[u8] = shift_jis!("\x61\x62\x63");
    assert_eq!(ABC_HEX, b"abc");
}
