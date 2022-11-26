use encoded::shift_jis;
use encoding_rs::SHIFT_JIS;

// const sjis_const: Cow<'static, [u8]> = SHIFT_JIS.encode("日本語").0;
const nihongo_const: &str = "日本語";

#[test]
fn it_works() {
    let nihongo = "日本語";
    let sjis = shift_jis!("日本語");
    // dbg!(sjis_const);
    // let expected = &SHIFT_JIS.encode("日本語").0[..];
    assert_eq!(sjis, &[147u8, 250, 150, 123, 140, 234]);
}
