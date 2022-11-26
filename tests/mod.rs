use encoded::shift_jis;

#[test]
fn it_works() {
    let sjis = shift_jis!("日本語");
    // dbg!(sjis_const);
    assert_eq!(sjis, &[147u8, 250, 150, 123, 140, 234]);
}
