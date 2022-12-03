use encoded::shift_jis;

#[test]
fn it_works() {
    let sjis: &[u8] = shift_jis!("漢字");
    assert_eq!(sjis, b"\x8a\xbf\x8e\x9a");
}
