use encoded::shift_jis;

#[test]
fn it_works() {
    let sjis = shift_jis!("日本語");
    assert_eq!(sjis, "日本語");
}
