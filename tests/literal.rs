use encoded::shift_jis;

#[test]
fn utf8() {
    const BYTES: &[u8] = shift_jis!("漢字");
    assert_eq!(BYTES, b"\x8a\xbf\x8e\x9a");
}

#[test]
fn unicode() {
    const BYTES: &[u8] = shift_jis!("\u{6f22}\u{5b57}");
    assert_eq!(BYTES, b"\x8a\xbf\x8e\x9a");
}

#[test]
fn char() {
    const BYTES: &[u8] = shift_jis!('字');
    assert_eq!(BYTES, b"\x8e\x9a");
}

#[test]
fn ascii() {
    const BYTES: &[u8] = shift_jis!("abc");
    assert_eq!(BYTES, b"abc");
}

#[test]
fn raw() {
    const BYTES: &[u8] = shift_jis!(r##"a"#b"c"##);
    assert_eq!(BYTES, br##"a"#b"c"##);
}

#[test]
fn hex() {
    const BYTES: &[u8] = shift_jis!("\x61\x62\x63");
    assert_eq!(BYTES, b"abc");
}
