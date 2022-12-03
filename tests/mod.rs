use std::io::{Cursor, Write, Result};

use encoded::shift_jis;

#[test]
fn it_works() -> Result<()> {
    let sjis: &[u8] = shift_jis!("漢字");
    assert_eq!(sjis, b"\x8a\xbf\x8e\x9a");

    let mut buff = Cursor::new(Vec::new());
    buff.write_all(shift_jis!("漢字"))?;
    assert_eq!(buff.get_ref(), b"\x8a\xbf\x8e\x9a");
    Ok(())
}
