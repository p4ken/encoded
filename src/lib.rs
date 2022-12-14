//! Macros converting character encodings at compile time
//! which reduce runtime processing costs and binary size.
//!
//! Encoding definitions depend on the [encoding_rs](https://docs.rs/encoding_rs/) crate.
//! 
//!
//! # Examples
//!
//! Convert a string literal into encoded bytes array:
//!
//! ```
//! const BYTES: &[u8] = encoded::shift_jis!("漢字");
//! assert_eq!(BYTES, b"\x8a\xbf\x8e\x9a");
//! ```
//!
//! Can also be used with [std::io::Write]:
//!
//! ```
//! use std::io::{Cursor, Write};
//!
//! let mut buff = Cursor::new(Vec::new());
//! buff.write_all(encoded::shift_jis!("漢字"));
//! assert_eq!(buff.get_ref(), b"\x8a\xbf\x8e\x9a");
//! ```
//!
//! # Compile Errors
//!
//! Argument must be a literal:
//!
//! ```compile_fail
//! const KANJI: &str = "漢字";
//! const BYTES: &[u8] = encoded::shift_jis!(KANJI);
//! //                                       ^^^^^
//! ```
//!
//! Any unmappable characters result a compile error:
//!
//! ```compile_fail
//! const BYTES: &[u8] = encoded::shift_jis!("鷗外");
//! //                                       ^^^^^^
//! ```

use proc_macro::TokenStream;

mod inner;

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::big5!("漢字");
/// assert_eq!(BYTES, b"\xba\x7e\xa6\x72");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn big5(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::BIG5)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::euc_jp!("漢字");
/// assert_eq!(BYTES, b"\xb4\xc1\xbb\xfa");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn euc_jp(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::EUC_JP)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::euc_kr!("한글");
/// assert_eq!(BYTES, b"\xc7\xd1\xb1\xdb");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn euc_kr(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::EUC_KR)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::gbk!("漢字");
/// assert_eq!(BYTES, b"\x9d\x68\xd7\xd6");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn gbk(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::GBK)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::gb18030!('💻');
/// assert_eq!(BYTES, b"\x94\x39\xdc\x31");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn gb18030(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::GB18030)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_2!("abecadło");
/// assert_eq!(BYTES, b"abecad\xb3o");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_2(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_2)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_4!("abėcėlė");
/// assert_eq!(BYTES, b"ab\xecc\xecl\xec");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_4(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_4)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_5!("Кириллица");
/// assert_eq!(BYTES, b"\xba\xd8\xe0\xd8\xdb\xdb\xd8\xe6\xd0");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_5(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_5)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_6!("الْأَبْجَدِيَّة");
/// assert_eq!(BYTES, b"\xc7\xe4\xf2\xc3\xee\xc8\xf2\xcc\xee\xcf\xf0\xea\xee\xf1\xc9");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_6(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_6)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_7!("αλφάβητο");
/// assert_eq!(BYTES, b"\xe1\xeb\xf6\xdc\xe2\xe7\xf4\xef");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_7(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_7)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_8!("אלפבית");
/// assert_eq!(BYTES, b"\xe0\xec\xf4\xe1\xe9\xfa");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_8(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_8)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_10!("stafrófið");
/// assert_eq!(BYTES, b"stafr\xf3fi\xf0");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_10(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_10)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_13!("abecadło");
/// assert_eq!(BYTES, b"abecad\xf9o");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_13(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_13)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_15!("œufs");
/// assert_eq!(BYTES, b"\xbdufs");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_15(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_15)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::iso_8859_16!("virguliță");
/// assert_eq!(BYTES, b"virguli\xfe\xe3");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn iso_8859_16(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::ISO_8859_16)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::koi8_r!("Кириллица");
/// assert_eq!(BYTES, b"\xeb\xc9\xd2\xc9\xcc\xcc\xc9\xc3\xc1");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn koi8_r(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::KOI8_R)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::koi8_u!("кирилиця");
/// assert_eq!(BYTES, b"\xcb\xc9\xd2\xc9\xcc\xc9\xc3\xd1");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn koi8_u(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::KOI8_U)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::shift_jis!("漢字");
/// assert_eq!(BYTES, b"\x8a\xbf\x8e\x9a");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn shift_jis(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::SHIFT_JIS)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_874!("อักษร");
/// assert_eq!(BYTES, b"\xcd\xd1\xa1\xc9\xc3");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_874(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_874)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1251!("Кириллица");
/// assert_eq!(BYTES, b"\xca\xe8\xf0\xe8\xeb\xeb\xe8\xf6\xe0");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1251(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1251)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1252!("œufs");
/// assert_eq!(BYTES, b"\x9cufs");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1252(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1252)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1253!("μικρός");
/// assert_eq!(BYTES, b"\xec\xe9\xea\xf1\xfc\xf2");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1253(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1253)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1254!("sığ");
/// assert_eq!(BYTES, b"s\xfd\xf0");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1254(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1254)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1255!("נִקּוּד‎");
/// assert_eq!(BYTES, b"\xf0\xc4\xf7\xcc\xe5\xcc\xe3\xfd");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1255(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1255)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1256!("الْأَبْجَدِيَّة");
/// assert_eq!(BYTES, b"\xc7\xe1\xfa\xc3\xf3\xc8\xfa\xcc\xf3\xcf\xf6\xed\xf3\xf8\xc9");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1256(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1256)
}

/// # Examples
///
/// ```
/// const BYTES: &[u8] = encoded::windows_1257!("„“");
/// assert_eq!(BYTES, b"\x84\x93");
/// ```
///
/// For more information, see [crate level documentation](crate).
#[proc_macro]
pub fn windows_1257(tokens: TokenStream) -> TokenStream {
    inner::convert(tokens, encoding_rs::WINDOWS_1257)
}
