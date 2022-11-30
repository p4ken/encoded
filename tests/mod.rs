use encoded::shift_jis;

#[test]
fn it_works() {
    let sjis = &shift_jis!("日本語")[..];
    assert_eq!(sjis, b"\x93\xfa\x96{\x8c\xea");
}

// sliceとarrayどちらが良いか？
// #[test]
// fn cow() {
//     let (sjis, _, _) = encoding_rs::SHIFT_JIS.encode("日本語");
//     sjis.concat(sjis)
// }
